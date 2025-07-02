use core::result::Result;
use core::usize;

use crate::common::messages::*;
use crate::common::types::DpsType;
use crate::common::*;

extern crate alloc;
use alloc::boxed::Box;

#[allow(unused)]
#[derive(Debug)]
pub struct VarInfo {
    var: Box<dyn DpsType>,
    var_id: u8,
}

#[derive(Debug, Clone, Copy)]
struct VarRecord {
    name: [u8; VAR_NAME_LENGTH],
    value: [u8; 4],
    size: u8,
    data_type: DataGenericType,
}

const MAX_VAR_FOR_SLAVE : usize = 16;
#[derive(Debug)]
struct MasterRecord {
    id: u8,
    board_name: [u8; BOARD_NAME_LENGTH],
    vars: [Option<VarRecord>;MAX_VAR_FOR_SLAVE],
}

#[derive(Debug)]
pub struct DpsMaster<const SB: usize> {
    master_id: u16,
    slaves_id: u16,
    send_f: SendFn,
    board_vec: [Option<MasterRecord>;SB],
    board_vec_cursor :usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
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

impl<const SB:usize> DpsMaster<SB> {
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
        Ok(())
    }

    pub fn list_board(&self) -> [Option<BoardInfo>;SB] {
        let mut vec_board_res = [const {None};SB];
        for (cursor_res,board) in self.board_vec.iter().flatten().enumerate() {
            let board_info = BoardInfo {
                name: core::str::from_utf8(&board.board_name).ok().unwrap(),
                id: board.id,
            };
            vec_board_res[cursor_res].replace(board_info);
        }
        vec_board_res
    }

    pub fn list_vars<'a>(&self, board_id: u8) -> Option<[Option<VarInfo>;MAX_VAR_FOR_SLAVE]> {
        let board = self._find_board(board_id)?;
        let mut vec_res : [Option<VarInfo>;MAX_VAR_FOR_SLAVE] = core::array::from_fn(|_| None);
        let mut empty = true;

        for (var_cursor,var) in board.vars.iter().flatten().enumerate() {
            let var_info_rec : Option<Box<dyn DpsType>> = match (var.data_type, var.size) {
                (DataGenericType::Unsigned, 0) =>
                {
                    Some(Box::new(u8::from_le_bytes(<[u8;1]>::try_from(&var.value[0..1]).unwrap())))
                },
                (DataGenericType::Signed, 0) =>
                {
                    Some(Box::new(i8::from_le_bytes(<[u8;1]>::try_from(&var.value[0..1]).unwrap())))
                },
                (DataGenericType::Unsigned, 1) =>
                {
                    Some(Box::new(u16::from_le_bytes(<[u8;2]>::try_from(&var.value[0..2]).unwrap())))
                },
                (DataGenericType::Signed, 1) =>
                {
                    Some(Box::new(i16::from_le_bytes(<[u8;2]>::try_from(&var.value[0..2]).unwrap())))
                },
                (DataGenericType::Unsigned, 2) =>
                {
                    Some(Box::new(u32::from_le_bytes(var.value)))
                },
                (DataGenericType::Signed, 2) =>
                {
                    Some(Box::new(i32::from_le_bytes(var.value)))
                },
                (DataGenericType::Floated, 2) =>
                {
                    Some(Box::new(f32::from_le_bytes(var.value)))
                },
                (_,_) => None,
            };

            if let Some(v) = var_info_rec{
                vec_res[var_cursor] = Some(VarInfo{
                    var: v,
                    var_id: u8::try_from(var_cursor).unwrap(),
                });
            }
            empty = false;
        }
        match empty {
            true => None,
            false => Some(vec_res),
        }
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
        let mut master_mex_mode_3 = DpsMasterMexModeM3::new();

        master_mex_mode_3.set_var_value_board_id(board_id)?;
        master_mex_mode_3.set_var_value_var_id(var_id)?;
        master_mex_mode_3.set_value(data.into())?;

        master_mex.set_m3(master_mex_mode_3)?;
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

    fn _get_var_id(&self, board: &MasterRecord, var: &VarRecord) -> u8{
        let base_array = board.vars.as_ptr() as *const &VarRecord;
        let ele = var as * const VarRecord;
        unsafe {ele.offset_from(*base_array) as u8}
    }

    fn _send_mex<'a>(&'a self, mex: &CanMessage, err: &'a str) -> Result<(), &'a str>{
        let mut tries = 0;
        while (self.send_f)(mex).is_err() && tries < 32 {
            tries += 1;
        };

        match tries {
            32.. => Err(err),
            _ => Ok(())
        }
    }

    fn _get_board_name(
        &mut self,
        board_id: u8,
        dps_slave_mex_mode_m0: &DpsSlaveMexModeM0,
    ) -> Result<bool, CanError> {
        let arr = dps_slave_mex_mode_m0.board_name().to_ne_bytes();
        if self._find_board(board_id).is_some() {
            return Ok(false);
        }

        let new_board = MasterRecord {
            id: board_id,
            board_name: arr[..BOARD_NAME_LENGTH].try_into().unwrap(),
            vars: [const {None};MAX_VAR_FOR_SLAVE],
        };
        let cursor = self.board_vec_cursor;
        self.board_vec_cursor+=1;
        self.board_vec[cursor].replace(new_board);
        Ok(true)
    }

    fn _update_var_data(&mut self, board_id: u8, var_id: u8, update_fun: impl Fn(&mut VarRecord))
    -> bool
    {
        if let Some(board) = self._find_board_mut(board_id){
            if let None = &mut board.vars[usize::from(var_id)]{
                board.vars[usize::from(var_id)] = Some(VarRecord{
                    name: [0;VAR_NAME_LENGTH],
                    value: [0;4],
                    size: 0,
                    data_type: DataGenericType::Unsigned,
                });
            }
            let var = &mut board.vars[usize::from(var_id)].unwrap();
            update_fun(var);
            return true;
        };
        false
    }

    fn _get_var_name(
        &mut self,
        board_id: u8,
        dps_slave_mex_mode_m1: &DpsSlaveMexModeM1,
    ) -> Result<bool, CanError> {
        let var_id = dps_slave_mex_mode_m1.info_var_id();
        let var_name_arr = dps_slave_mex_mode_m1.var_name().to_le_bytes();
        let var_name_arr = <[u8;VAR_NAME_LENGTH]>::try_from(&var_name_arr[0..VAR_NAME_LENGTH]).unwrap();

        let update_fun = |var: &mut VarRecord| {
            var.name = var_name_arr
        };

        Ok(self._update_var_data(board_id, var_id, update_fun))
    }

    fn _get_var_metadata(
        &mut self,
        board_id: u8,
        dps_slave_mex_mode_m2: &DpsSlaveMexModeM2,
    ) -> Result<bool, CanError> {
        let var_id = dps_slave_mex_mode_m2.value_var_id();

        let var_type = match dps_slave_mex_mode_m2.value_var_type() {
            DpsSlaveMexValueVarType::Unsigned=> DataGenericType::Unsigned,
            DpsSlaveMexValueVarType::Signed=> DataGenericType::Signed,
            DpsSlaveMexValueVarType::Floated=> DataGenericType::Floated,
            DpsSlaveMexValueVarType::_Other(_) => DataGenericType::Unsigned,
        };

        let var_size = match dps_slave_mex_mode_m2.value_var_size() {
            DpsSlaveMexValueVarSize::X8Bit=> 1,
            DpsSlaveMexValueVarSize::X16Bit => 2,
            DpsSlaveMexValueVarSize::X32Bit => 4,
            DpsSlaveMexValueVarSize::_Other(_) => 1,
        };

        let update_fun = |var: &mut VarRecord| {
            var.size = var_size;
            var.data_type = var_type;
        };

        Ok(self._update_var_data(board_id, var_id, update_fun))
    }

    fn _get_var_value(
        &mut self,
        board_id: u8,
        dps_slave_mex_mode_m3: &DpsSlaveMexModeM3,
    ) -> Result<bool, CanError> {
        let var_id = dps_slave_mex_mode_m3.var_id();
        let var_value = dps_slave_mex_mode_m3.value().to_le_bytes();

        let update_fun = |var: &mut VarRecord| {
            var.value = var_value;
        };

        Ok(self._update_var_data(board_id, var_id, update_fun))
    }

    fn _refresh_request_checked(
        &self,
        board: &MasterRecord,
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
        let var_id = self._get_var_id(board,var);
        master_mex_mode_3.set_var_value_board_id(board.id)?;
        master_mex_mode_3.set_var_value_var_id(var_id)?;
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

    fn _find_board(&self, board_id: u8) -> Option<&MasterRecord>
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

    fn _find_board_mut(&mut self, board_id: u8) -> Option<&mut MasterRecord>
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
        let var = &b.vars[usize::from(var_id)];

        match var {
            None => None,
            Some(var) => Some(&var)
        }
    }

    fn _find_var_mut(&mut self, board_id: u8, var_id: u8) -> Option<&mut VarRecord>
    {
        let b = self._find_board_mut(board_id)?;
        let var = &mut b.vars[usize::from(var_id)];

        match var {
            None => None,
            Some(var) => Some(var)
        }
    }
}
