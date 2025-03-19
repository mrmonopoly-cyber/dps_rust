use alloc::vec::Vec;

use crate::common::messages::*;
use crate::common::types::DpsType;
use crate::common::*;

use DataGenericType::*;

pub type PostUpdateFn<'a> = fn(var_name: &'a str, var_data: &'a [u8]);

#[derive(Debug)]
struct VarRecordSlave<'a> {
    ref_var: &'a mut [u8],
    var_name: &'a str,
    var_id: u8, //4 bits
    data_type: DataGenericType,
    post_update_f: PostUpdateFn<'a>,
}

#[derive(Debug)]
pub struct DpsSlave<'a> {
    board_name: &'a str,
    send_f: SendFn,
    vars: Vec<VarRecordSlave<'a>>,
    board_id: u8,
    obj_ids: u8,
    master_id: u16,
    slave_id: u16,
    enable: bool,
}

impl<'a> DpsSlave<'a> {
    pub fn new(
        board_name: &'a [u8],
        send_f: SendFn,
        board_id: u8,
        master_id: u16,
        slave_id: u16,
    ) -> Result<Self, &'a str>{
        if board_name.len() > VAR_NAME_LENGTH {
            return Err("var name string too long ")
        }
        Ok(Self {
            board_name: core::str::from_utf8(board_name).unwrap(),
            send_f,
            vars: Vec::new(),
            board_id,
            obj_ids: 0,
            master_id,
            slave_id,
            enable: false,
        })
    }

    pub fn enable(&mut self) {
        self.enable = true
    }

    pub fn disable(&mut self) {
        self.enable = true
    }

    pub fn monitor_var(&mut self, var: impl DpsType) {
        
    }

    pub fn monitor_primitive_var<T>(
        &mut self,
        data_type: DpsPrimitiveTypes,
        var: &'a mut T,
        post_update_f: PostUpdateFn<'a>,
        var_name: &'a[u8],
    ) -> Result<(),& 'a str>
    {
        if var_name.len() > VAR_NAME_LENGTH {
            return Err("var name string too long ")
        }

        let raw_ptr :*mut u8 = var as *mut T as *mut u8;
        let mut new_var = VarRecordSlave {
            ref_var: unsafe {core::slice::from_raw_parts_mut(raw_ptr, core::mem::size_of_val(var))},
            var_name: core::str::from_utf8(var_name).unwrap(),
            var_id: self.obj_ids,
            data_type: Unsigned,
            post_update_f,
        };
        self.obj_ids += 1;

        match data_type {
            DpsPrimitiveTypes::DpsTypesUint8T
            | DpsPrimitiveTypes::DpsTypesUint16T
            | DpsPrimitiveTypes::DpsTypesUint32T => new_var.data_type = Unsigned,
            DpsPrimitiveTypes::DpsTypesInt8T
            | DpsPrimitiveTypes::DpsTypesInt16T
            | DpsPrimitiveTypes::DpsTypesInt32T => new_var.data_type = Signed,
            DpsPrimitiveTypes::DpsTypesFloatT => new_var.data_type = Floated,
        };
        self.vars.push(new_var);
        Ok(())
    }

    pub fn check_can_mex_recv(&'a mut self, mex: &CanMessage) -> Result<(),CanError> {
        if !self.enable {
            return Ok(());
        };
        if mex.id != self.master_id {
            return Err(messages::CanError::UnknownMessageId(mex.id.into()))
        }

        let mut master_mex = DpsMasterMex::try_from(mex.payload)?;

        match master_mex.mode()? {
            DpsMasterMexMode::M0(_) => self._discover_board(),
            DpsMasterMexMode::M1(mex_mode_1) => self._request_info(&mex_mode_1),
            DpsMasterMexMode::M2(mex_mode_2) => self._request_var_metadata(&mex_mode_2),
            DpsMasterMexMode::M3(mex_mode_3) => self._request_var_value(&mex_mode_3),
            DpsMasterMexMode::M4(mex_mode_4) => self._update_var_value(&mex_mode_4),
        }

    }
    fn _update_var_value(&'a mut self, master_mex: &DpsMasterMexModeM4) -> Result<(),CanError>
    {
        let val_slice = master_mex.update_var_value_var_value().to_le_bytes();

        if self.board_id != master_mex.update_var_value_board_id(){
            return Ok(());
        }

        let var = self.vars
            .iter_mut()
            .find(|x| x.var_id == master_mex.update_var_value_var_id());

        match var
        {
            Some(var) => {
                var.ref_var.copy_from_slice(&val_slice);
                (var.post_update_f)(var.var_name, var.ref_var);
                Ok(())
            },

            None => Err(CanError::ParameterOutOfRange {
                message_id: u32::from(master_mex.update_var_value_var_id())
            }),
        }
    }

    fn _request_var_value(&self, master_mex: &DpsMasterMexModeM3) -> Result<(),CanError>
    {
        let mut slave_mex = DpsSlaveMex::new(self.board_id, 3)?;
        let mut slave_mex_mode_3 = DpsSlaveMexModeM3::new();

        if self.board_id != master_mex.var_value_board_id(){
            return Ok(());
        }
        self.vars.iter()
            .find(|x| x.var_id == master_mex.var_value_var_id())
            .inspect(|x| {
                let var_value : u32 = 42;
                let _ = slave_mex_mode_3.set_var_id(x.var_id);
                let _ = slave_mex_mode_3.set_value(var_value);
                let _ = slave_mex.set_m3(slave_mex_mode_3);
            });
        let raw_mex = CanMessage{
            id: self.slave_id,
            payload: slave_mex.raw()
        };
        (self.send_f)(&raw_mex)
    }

    fn _request_var_metadata(&self, master_mex: &DpsMasterMexModeM2) -> Result<(),CanError>
    {
        if self.board_id != master_mex.var_metadata_board_id(){
            return Ok(());
        }

        for var in self.vars.iter()
        {
            let mut slave_mex = DpsSlaveMex::new(self.board_id, 1)?;
            let mut slave_mode_2 = DpsSlaveMexModeM2::new();
            slave_mode_2.set_value_var_id(var.var_id)?;
            slave_mode_2.set_value_var_size(u8::try_from(var.ref_var.len()).unwrap())?;
            match var.data_type {
                Unsigned |
                Signed => slave_mode_2.set_value_var_type(0)?,
                Floated => slave_mode_2.set_value_var_type(1)?,
            };
            slave_mex.set_m2(slave_mode_2)?;
            let raw_mex = CanMessage{
                id: self.slave_id,
                payload: slave_mex.raw(),
            };
            let mut tries= 0;
            while (self.send_f)(&raw_mex).is_err() && tries < 32
            {
                tries+=1;
            }
        }
        Ok(())
    }

    fn _request_info(&self, master_mex: &DpsMasterMexModeM1) -> Result<(), CanError>{

        if self.board_id != master_mex.var_name_board_id() {
            return Ok(());
        }

        for var in self.vars.iter()
        {
            let mut slave_mex = DpsSlaveMex::new(self.board_id, 1)?;
            let mut slave_mode_1 = DpsSlaveMexModeM1::new();
            slave_mode_1.set_info_var_id(var.var_id)?;
            slave_mode_1.set_var_name(str::parse(var.var_name).unwrap())?;
            slave_mex.set_m1(slave_mode_1)?;
            let raw_mex = CanMessage{
                id: self.slave_id,
                payload: slave_mex.raw(),
            };
            let mut tries= 0;
            while (self.send_f)(&raw_mex).is_err() && tries < 32
            {
                tries+=1;
            }
        }



        Ok(())
    }

    fn _discover_board(&self) -> Result<(), CanError>{
        let mut slave_mex = DpsSlaveMex::new(self.board_id, 0)?;
        let mut slave_mode_0 = DpsSlaveMexModeM0::new();
        slave_mode_0.set_board_name(str::parse(self.board_name).unwrap())?;
        slave_mex.set_m0(slave_mode_0)?;

        let raw_mex = CanMessage{
            id: self.slave_id,
            payload: slave_mex.raw(),
        };

        (self.send_f)(&raw_mex)
    }
}
