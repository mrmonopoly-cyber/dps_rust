use crate::common::messages::*;
use crate::common::types::DpsType;
use crate::common::*;

use DataGenericType::*;

pub type PostUpdateFn<'a> = fn(var_name: &'a str, var_data: &'a dyn DpsType);

#[derive(Debug)]
struct VarRecordSlave<'a> {
    ref_var: &'a mut dyn DpsType,
    var_name: &'a str,
    var_id: u8, //4 bits
    post_update_f: PostUpdateFn<'a>,
}

#[derive(Debug)]
pub struct DpsSlave<'a, const S: usize> {
    board_name: &'a str,
    send_f: SendFn,
    vars: [Option<VarRecordSlave<'a>>;S],
    var_cursor:usize,
    board_id: u8,
    obj_ids: u8,
    master_id: u16,
    slave_id: u16,
    enable: bool,
}

impl<'a, const S: usize> DpsSlave<'a,S> {
    pub fn new(
        board_name: &'a [u8],
        send_f: SendFn,
        board_id: u8,
        master_id: u16,
        slave_id: u16,
    ) -> Result<Self, &'a str> {
        if board_name.len() > VAR_NAME_LENGTH {
            return Err("var name string too long ");
        }

        if S >= 2usize.pow(4) {
            return Err("Invalid Var buffer size");
        }

        Ok(Self {
            board_name: core::str::from_utf8(board_name).unwrap(),
            send_f,
            vars: [const {None};S],
            var_cursor:0,
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

    pub fn monitor_var(
        &'a mut self,
        var_name: &'a str,
        var: &'a mut dyn DpsType,
        post_update_f: PostUpdateFn<'a>,
    ) -> Result<(), &'a str> {
        if !self.enable {
            return Err("dps is disable");
        }
        if var_name.len() > VAR_NAME_LENGTH {
            return Err("var name string too long ");
        }

        if self.obj_ids >= 2u8.pow(4){
            return Err("Current dps node full");
        }

        if self.obj_ids >= 2u8.pow(4)
        {
            return Err("Dps slave is full");   
        }

        let new_id = self.obj_ids;
        self.obj_ids+=1;

        let new_var = VarRecordSlave{
            ref_var: var,
            var_name,
            var_id: new_id,
            post_update_f,
        };

        self.vars[self.var_cursor] = Some(new_var);
        self.var_cursor+=1;

        Ok(())
    }

    pub fn check_can_mex_recv(&'a mut self, mex: &CanMessage) -> Result<(), &'a str> {
        if !self.enable {
            return Err("dps is disable");
        }
        if !self.enable {
            return Ok(());
        };
        if mex.id != self.master_id {
            return Err("not right master id");
        }

        let mut master_mex = DpsMasterMex::try_from(mex.payload)
            .map_err(|_| "invalid master mex payload")?;

        let mode = master_mex.mode()
            .map_err(|_| "invalid master mex mode")?;

        match mode{
            DpsMasterMexMode::M0(_) => self._discover_board(),
            DpsMasterMexMode::M1(mex_mode_1) => self._request_info(&mex_mode_1),
            DpsMasterMexMode::M2(mex_mode_2) => self._request_var_value(&mex_mode_2),
            DpsMasterMexMode::M3(mex_mode_3) => self._update_var_value(&mex_mode_3),
        }
    }

    //private

    fn _send_mex(&'a self, mex: &CanMessage, err: &'a str) -> Result<(), &'a str>{
        let mut tries = 0;
        while (self.send_f)(mex).is_err() && tries < 32 {
            tries += 1;
        };

        match tries {
            32.. => Err(err),
            _ => Ok(())
        }
    }

    fn _update_var_value(&'a mut self, master_mex: &DpsMasterMexModeM3) -> Result<(), &'a str> {
        let val_slice = master_mex.value().to_le_bytes();

        if self.board_id != master_mex.var_value_board_id() {
            return Ok(());
        }

        let var = self.vars
            .iter_mut()
            .find(|x| {
                if let Some(var) = x {
                    var.var_id == master_mex.var_value_var_id()
                }else{
                    false
                }
            });

        match var 
        {
            Some(var) => 
            {
                if let Some(var) = var 
                {
                    let _ = var.ref_var.update(&val_slice);
                    (var.post_update_f)(var.var_name, var.ref_var);
                    Ok(())
                }else{
                    Err("variable not found")
                }
            },
            None => Err("variable not found"),
        }
    }



    fn _request_var_value(&'a self, master_mex: &DpsMasterMexModeM2) -> Result<(), &'a str> {
        if self.board_id != master_mex.var_refresh_board_id()
        {
            return Ok(());
        }

        let mut slave_mex = DpsSlaveMex::new(self.board_id, 3).ok().unwrap();
        let mut slave_mex_mode_3 = DpsSlaveMexModeM3::new();

        self.vars
            .iter()
            .find(|x|{
                if let Some(var) = x
                {
                    var.var_id == master_mex.var_refresh_var_id()
                }else{
                    false
                }
            })
            .inspect(|x| {
                if  x.is_none() {
                    return;
                };
                let x = x.as_ref().unwrap();
                let var_value: u32 = 42;
                slave_mex_mode_3.set_var_id(x.var_id).ok().unwrap();
                slave_mex_mode_3.set_value(var_value).ok().unwrap();
                slave_mex.set_m3(slave_mex_mode_3).ok().unwrap();
            });
        let raw_mex = CanMessage {
            id: self.slave_id,
            payload: slave_mex.raw(),
        };
        self._send_mex(&raw_mex, "failed send message for request var value")
    }

    fn _request_info(&'a self, master_mex: &DpsMasterMexModeM1) -> Result<(), &'a str> {
        if self.board_id != master_mex.var_name_board_id() {
            return Ok(());
        }

        for var in self.vars.iter().flatten() {
            let mut slave_mex = DpsSlaveMex::new(self.board_id, 1).ok().unwrap();
            let mut slave_mode_1 = DpsSlaveMexModeM1::new();
            let mut slave_mode_2 = DpsSlaveMexModeM2::new();

            slave_mode_1.set_info_var_id(var.var_id).ok().unwrap();
            slave_mode_1.set_var_name(str::parse(var.var_name).unwrap()).ok().unwrap();
            slave_mex.set_m1(slave_mode_1).ok().unwrap();
            let raw_mex = CanMessage {
                id: self.slave_id,
                payload: slave_mex.raw(),
            };
            self._send_mex(&raw_mex, "failed sending slave resp request info mex name")?;

            slave_mode_2.set_value_var_id(var.var_id).ok().unwrap();
            slave_mode_2.set_value_var_size(var.ref_var.get_type_size().try_into().unwrap())
                .ok().unwrap();
            match var.ref_var.get_type_category(){
                Unsigned | Signed => slave_mode_2.set_value_var_type(0).ok().unwrap(),
                Floated => slave_mode_2.set_value_var_type(1).ok().unwrap(),
            };
            slave_mex.set_m2(slave_mode_2).ok().unwrap();
            let raw_mex = CanMessage {
                id: self.slave_id,
                payload: slave_mex.raw(),
            };
            self._send_mex(&raw_mex, "failed sending slave resp request info mex metadata")?;
        }

        Ok(())
    }

    fn _discover_board(&'a self) -> Result<(), &'a str> {
        let mut slave_mex = DpsSlaveMex::new(self.board_id, 0).ok().unwrap();
        let mut slave_mode_0 = DpsSlaveMexModeM0::new();
        slave_mode_0.set_board_name(str::parse(self.board_name).unwrap()).ok().unwrap();
        slave_mex.set_m0(slave_mode_0).ok().unwrap();

        let raw_mex = CanMessage {
            id: self.slave_id,
            payload: slave_mex.raw(),
        };

        self._send_mex(&raw_mex,"failed to send discover board response")
    }
}
