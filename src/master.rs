use alloc::boxed::Box;
use alloc::vec::Vec;
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
struct MasterRecord {
    id: u8,
    board_name: [u8; BOARD_NAME_LENGTH],
    vars: Vec<VarRecord>,
}

#[derive(Debug)]
pub struct DpsMaster {
    master_id: u16,
    slaves_id: u16,
    send_f: SendFn,
    board_vec: Vec<MasterRecord>,
    objs_count: u8, //4 bits
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

impl DpsMaster {
    pub fn new(master_id: u16, slaves_id: u16, send_f: SendFn) -> Self {
        Self {
            master_id,
            slaves_id,
            send_f,
            board_vec: Vec::new(),
            objs_count: 0,
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
        for board in self.board_vec.iter() {
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

    pub fn list_board(&self) -> Box<[BoardInfo]> {
        let mut vec_board_res = Vec::with_capacity(self.board_vec.len());
        for board in self.board_vec.iter() {
            let board_info = BoardInfo {
                name: core::str::from_utf8(&board.board_name).ok().unwrap(),
                id: board.id,
            };
            vec_board_res.push(board_info);
        }
        vec_board_res.into_boxed_slice()
    }

    pub fn list_vars(&self, board_id: u8) -> Option<Box<[VarRecord]>> {
        let board = self.board_vec.iter().find(|b| b.id == board_id)?;
        let mut vec_res = Vec::with_capacity(board.vars.len());

        for var in board.vars.iter() {
            vec_res.push(*var);
        }

        Some(vec_res.into_boxed_slice())
    }

    pub fn refresh_value_var(&self, board_id: u8, var_id: u8) -> Result<(), CanError> {
        let board = self.board_vec.iter().find(|b| b.id == board_id);
        if board.is_none() {
            return Ok(());
        }
        let board = board.unwrap();
        let var = board.vars.iter().find(|v| v.id == var_id);
        if var.is_none() {
            return Ok(());
        }
        let var = var.unwrap();
        self._refresh_request_checked(board, var)
    }

    pub fn refresh_value_var_all(&self, board_id: u8) -> Result<(), CanError> {
        let board = self.board_vec.iter().find(|b| b.id == board_id);
        if board.is_none() {
            return Ok(());
        }
        let board = board.unwrap();

        for var in board.vars.iter() {
            let _ = self._refresh_request_checked(board, var);
        }
        Ok(())
    }

    pub fn get_value_var(&self, board_id: u8, var_id: u8) -> Option<VarValue> {
        let board = self.board_vec.iter().find(|b| b.id == board_id)?;
        let var = board.vars.iter().find(|v| v.id == var_id)?;

        Some(VarValue {
            raw_data: &var.value,
            data_type: var.data_type,
        })
    }

    pub fn update_remote_var<T>(&self, board_id: u8, var_id: u8, data: T) -> Result<(), CanError>
    where
        T: Into<u32>,
    {
        let board = self.board_vec.iter().find(|b| b.id == board_id);
        if board.is_none() {
            return Ok(());
        }
        let board = board.unwrap();
        let var = board.vars.iter().find(|v| v.id == var_id);
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
        if self.board_vec.iter_mut().any(|b| b.id == board_id) {
            return Ok(false);
        }

        let new_board = MasterRecord {
            id: board_id,
            board_name: arr[..BOARD_NAME_LENGTH].try_into().unwrap(),
            vars: Vec::new(),
        };
        self.objs_count += 1;
        self.board_vec.push(new_board);
        Ok(true)
    }

    fn _get_var_name(
        &mut self,
        board_id: u8,
        dps_slave_mex_mode_m1: &DpsSlaveMexModeM1,
    ) -> Result<bool, CanError> {
        let var_id = dps_slave_mex_mode_m1.info_var_id();
        let var_name_arr = dps_slave_mex_mode_m1.var_name().to_le_bytes();
        let board = self.board_vec.iter_mut().find(|b| b.id == board_id);
        match board {
            Some(board) => {
                let var = board.vars.iter_mut().find(|v| v.id == var_id);
                if var.is_none() {
                    board.vars.push(VarRecord {
                        id: var_id,
                        name: var_name_arr[0..VAR_NAME_LENGTH].try_into().unwrap(),
                        value: [0; 4],
                        data_type: DataGenericType::Unsigned,
                        size: 0,
                    });
                } else {
                    let var = var.unwrap();
                    var.name = var_name_arr[0..VAR_NAME_LENGTH].try_into().unwrap();
                }
                Ok(true)
            }
            None => Ok(false),
        }
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

        let var_id = dps_slave_mex_mode_m2.value_var_id();
        let board = self.board_vec.iter_mut().find(|b| b.id == board_id);
        match board {
            Some(board) => {
                let var = board.vars.iter_mut().find(|v| v.id == var_id);
                match var {
                    Some(var) => {
                        var.data_type = var_type;
                        var.size = var_size;
                        Ok(true)
                    }
                    None => {
                        board.vars.push(VarRecord {
                            id: var_id,
                            name: [0; VAR_NAME_LENGTH],
                            value: [0; 4],
                            data_type: var_type,
                            size: var_size,
                        });
                        Ok(true)
                    }
                }
            }
            None => Ok(false),
        }
    }

    fn _get_var_value(
        &mut self,
        board_id: u8,
        dps_slave_mex_mode_m3: &DpsSlaveMexModeM3,
    ) -> Result<bool, CanError> {
        let var_id = dps_slave_mex_mode_m3.var_id();
        let board = self.board_vec.iter_mut().find(|b| b.id == board_id);

        match board {
            None => Ok(false),
            Some(board) => {
                let var = board.vars.iter_mut().find(|v| v.id == var_id);
                match var {
                    None => (),
                    Some(var) => {
                        let arr = dps_slave_mex_mode_m3.value().to_le_bytes();
                        var.value = arr.try_into().unwrap();
                    }
                }
                Ok(true)
            }
        }
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
}
