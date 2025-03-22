use core::result::Result;

use crate::common::messages::*;
use crate::common::*;

#[derive(Debug, Clone, Copy)]
pub struct VarRecord {
    id: u8,
    name: [u8; VAR_NAME_LENGTH],
    value: [u8; 4],
    size: u8,
    data_type: DataGenericType,
}

#[derive(Debug)]
struct MasterRecord<const S:usize> {
    id: u8,
    board_name: [u8; BOARD_NAME_LENGTH],
    vars: [Option<VarRecord>;S],
    vars_cursor: usize,
}

#[derive(Debug)]
pub struct DpsMaster<const SB: usize, const SM:usize> {
    master_id: u16,
    slaves_id: u16,
    send_f: SendFn,
    board_vec: [Option<MasterRecord<SM>>;SB],
    board_vec_cursor :usize,
}

#[derive(Debug, Clone, Copy)]
pub struct BoardInfo<'a> {
    pub name: &'a str,
    pub id: u8,
}

#[allow(unused)]
#[derive(Debug)]
pub struct VarValue<'a> {
    raw_data: &'a [u8],
    data_type: DataGenericType,
}

impl<const SB:usize, const SM: usize> DpsMaster<SB,SM> {
    pub fn new(master_id: u16, slaves_id: u16, send_f: SendFn) -> Self {
        Self {
            master_id,
            slaves_id,
            send_f,
            board_vec: [const {None};SB],
            board_vec_cursor:0,
        }
    }

    pub fn new_connection(&self) -> Result<(), CanError> {
        let master_mex = DpsMasterMex::new(0)?;
        let raw_mex = CanMessage {
            id: self.master_id,
            payload: master_mex.raw(),
        };

        (self.send_f)(&raw_mex)
    }

    pub fn request_info(&self) -> Result<(), CanError> {
        for board in self.board_vec.iter().flatten() {
            let mut master_mex = DpsMasterMex::new(1)?;
            let mut master_mode_1 = DpsMasterMexModeM1::new();
            master_mode_1.set_var_name_board_id(board.id)?;
            master_mex.set_m1(master_mode_1)?;
            let can_mex_raw = CanMessage {
                id: self.master_id,
                payload: master_mex.raw(),
            };
            let mut tries = 0;
            while (self.send_f)(&can_mex_raw).is_err() && tries < 32 {
                tries += 1;
            }
        }
        todo!()
    }

    pub fn list_board(&self) -> [Option<BoardInfo>;SB] {
        let mut vec_board_res = [const {None};SB];
        for (cursor_res,board) in self.board_vec.iter().flatten().enumerate() {
            let board_info = BoardInfo {
                name: core::str::from_utf8(&board.board_name).ok().unwrap(),
                id: board.id,
            };
            vec_board_res[cursor_res] = Some(board_info);
        }
        vec_board_res
    }

    pub fn list_vars(&self, board_id: u8) -> Option<[Option<VarRecord>;SM]> {
        let board = self._find_board(board_id)?;
        let mut vec_res = [None;SM];

        for (var_cursor,var) in board.vars.iter().flatten().enumerate() {
            vec_res[var_cursor].replace(*var);
        }

        Some(vec_res)
    }

    pub fn refresh_value_var(&self, board_id: u8, var_id: u8) -> Result<(), CanError> {
        let board = self._find_board(board_id);
        if board.is_none() {
            return Ok(());
        }
        let board = board.unwrap();
        let var = self._find_var(board_id, var_id);
        if var.is_none() {
            return Ok(());
        }
        let var = var.unwrap();
        self._refresh_request_checked(board, var)
    }

    pub fn refresh_value_var_all(&self, board_id: u8) -> Result<(), CanError> {
        let board = self._find_board(board_id);
        if board.is_none() {
            return Ok(());
        }
        let board = board.unwrap();

        for var in board.vars.iter().flatten() {
            let _ = self._refresh_request_checked(board, var);
        }
        Ok(())
    }

    pub fn get_value_var(&self, board_id: u8, var_id: u8) -> Option<VarValue> {
        let var = self._find_var(board_id, var_id)?;

        Some(VarValue {
            raw_data: &var.value,
            data_type: var.data_type,
        })
    }

    pub fn update_remote_var<T>(&self, board_id: u8, var_id: u8, data: T) -> Result<(), CanError>
    where
        T: Into<u32>,
    {
        let var = self._find_var(board_id, var_id);
        if var.is_none() {
            return Ok(());
        }

        let mut master_mex = DpsMasterMex::new(4)?;
        let mut master_mex_mode_4 = DpsMasterMexModeM4::new();

        master_mex_mode_4.set_update_var_value_board_id(board_id)?;
        master_mex_mode_4.set_update_var_value_var_id(var_id)?;
        master_mex_mode_4.set_update_var_value_var_value(data.into())?;

        master_mex.set_m4(master_mex_mode_4)?;
        let raw_mex = CanMessage {
            id: self.master_id,
            payload: master_mex.raw(),
        };
        let mut tries = 0;
        while (self.send_f)(&raw_mex).is_err() && tries < 32 {
            tries += 1;
        }
        match tries {
            32.. => Err(CanError::InvalidPayloadSize),
            _ => Ok(()),
        }
    }

    pub fn check_mex_recv(&mut self, mex: &CanMessage) -> Result<bool, CanError> {
        let mut slave_mex = DpsSlaveMex::try_from(mex.payload)?;
        if mex.id != self.slaves_id {
            return Ok(false);
        }

        match slave_mex.mode()? {
            messages::DpsSlaveMexMode::M0(mode_mex) => {
                self._get_board_name(slave_mex.board_id(), &mode_mex)
            }
            messages::DpsSlaveMexMode::M1(mode_mex) => {
                self._get_var_name(slave_mex.board_id(), &mode_mex)
            }
            messages::DpsSlaveMexMode::M2(mode_mex) => {
                self._get_var_metadata(slave_mex.board_id(), &mode_mex)
            }
            messages::DpsSlaveMexMode::M3(mode_mex) => {
                self._get_var_value(slave_mex.board_id(), &mode_mex)
            }
        }
    }

    //private

    fn _get_board_name(
        &mut self,
        board_id: u8,
        dps_slave_mex_mode_m0: &DpsSlaveMexModeM0,
    ) -> Result<bool, CanError> {
        let arr = dps_slave_mex_mode_m0.board_name().to_ne_bytes();
        if self._find_board(board_id).is_none() {
            return Ok(false);
        }

        let new_board = MasterRecord {
            id: board_id,
            board_name: arr[..BOARD_NAME_LENGTH].try_into().unwrap(),
            vars: [const {None};SM],
            vars_cursor:0,
        };
        self.board_vec[self.board_vec_cursor].replace(new_board);
        self.board_vec_cursor+=1;
        Ok(true)
    }

    fn _get_var_name(
        &mut self,
        board_id: u8,
        dps_slave_mex_mode_m1: &DpsSlaveMexModeM1,
    ) -> Result<bool, CanError> {
        let var_id = dps_slave_mex_mode_m1.info_var_id();
        let var_name_arr = dps_slave_mex_mode_m1.var_name().to_le_bytes();
        
        Ok(false)
    }

    fn _get_var_metadata(
        &mut self,
        board_id: u8,
        dps_slave_mex_mode_m2: &DpsSlaveMexModeM2,
    ) -> Result<bool, CanError> {
        let var_type = match dps_slave_mex_mode_m2.value_var_type() {
            DpsSlaveMexValueVarType::UnsignedInteger => DataGenericType::Unsigned,
            DpsSlaveMexValueVarType::SignedInteger => DataGenericType::Signed,
            DpsSlaveMexValueVarType::Float => DataGenericType::Floated,
            DpsSlaveMexValueVarType::_Other(_) => DataGenericType::Unsigned,
        };

        let var_size = match dps_slave_mex_mode_m2.value_var_size() {
            DpsSlaveMexValueVarSize::X8bit => 1,
            DpsSlaveMexValueVarSize::X16bit => 2,
            DpsSlaveMexValueVarSize::X32bit => 4,
            DpsSlaveMexValueVarSize::_Other(_) => 1,
        };

        Ok(false)
    }

    fn _get_var_value(
        &mut self,
        board_id: u8,
        dps_slave_mex_mode_m3: &DpsSlaveMexModeM3,
    ) -> Result<bool, CanError> {
        let var_id = dps_slave_mex_mode_m3.var_id();

        let var = self._find_var_mut(board_id, var_id);
        match var {
            None => (),
            Some(var) => {
                let arr = dps_slave_mex_mode_m3.value().to_le_bytes();
                var.value = arr;
            }
        }
        Ok(true)
    }

    fn _refresh_request_checked(
        &self,
        board: &MasterRecord<SM>,
        var: &VarRecord,
    ) -> Result<(), CanError> {
        let master_mex = DpsMasterMex::new(3).ok();
        if master_mex.is_none() {
            return Err(CanError::InvalidMultiplexor {
                message_id: u32::from(self.master_id),
                multiplexor: 3,
            });
        }
        let mut master_mex = master_mex.unwrap();
        let mut master_mex_mode_3 = DpsMasterMexModeM3::new();
        master_mex_mode_3.set_var_value_board_id(board.id)?;
        master_mex_mode_3.set_var_value_var_id(var.id)?;
        master_mex.set_m3(master_mex_mode_3)?;

        let mut tries = 0;
        while tries < 32
            && (self.send_f)(&CanMessage {
                id: self.master_id,
                payload: master_mex.raw(),
            })
            .is_err()
        {
            tries += 1;
        }

        Ok(())
    }

    fn _find_board(&self, board_id: u8) -> Option<&MasterRecord<SM>>
    {
        let f =self.board_vec.iter().find(|b|{
            match b {
                Some(b) => b.id == board_id,
                None => false,
            }
        })?;

        if let Some(board) = f{
            return Some(board);
        }
        None
    }

    fn _find_board_mut(&mut self, board_id: u8) -> Option<&mut MasterRecord<SM>>
    {
        let f =self.board_vec.iter_mut().find(|b|{
            match b {
                Some(b) => b.id == board_id,
                None => false,
            }
        })?;

        if let Some(board) = f{
            return Some(board);
        }
        None
    }

    fn _find_var(&self, board_id: u8, var_id: u8) -> Option<&VarRecord>
    {
        let b = self._find_board(board_id)?;
        let var = b.vars.iter().find(|v|{
            match v{
                Some(var) => var.id == var_id,
                None => false,
            }
        })?;

        var.as_ref()
    }

    fn _find_var_mut(&mut self, board_id: u8, var_id: u8) -> Option<&mut VarRecord>
    {
        let b = self._find_board_mut(board_id)?;
        let var = b.vars.iter_mut().find(|v|{
            match v{
                Some(var) => var.id == var_id,
                None => false,
            }
        })?;

        var.as_mut()
    }
}
