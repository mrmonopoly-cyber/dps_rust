use alloc::vec::Vec;
use alloc::boxed::Box;
use core::result::Result;

use crate::common::messages::{CanError, DpsMasterMex, DpsMasterMexModeM1, DpsMasterMexModeM3, DpsMasterMexModeM4};
use crate::common::*;

// char name[VAR_NAME_LENGTH];
// uint32_t value;
// uint8_t size;
// enum DATA_GENERIC_TYPE type:2;
// uint8_t updated : 1;

#[derive(Debug,Clone, Copy)]
pub struct VarRecord<'a> {
    id: u8,
    name: &'a str,
    value: [u8;4],
    data_type: DataGenericType,
}

#[derive(Debug)]
struct MasterRecord<'a> {
    id: u8,
    board_name: &'a str,
    vars: Vec<VarRecord<'a>>,
}

#[derive(Debug)]
pub struct DpsMaster<'a> {
    master_id: u16,
    slaves_id: u16,
    obj_ids_counter: u8, //size 4 bits
    send_f: SendFn,
    board_vec: Vec<MasterRecord<'a>>,
}

#[derive(Debug,Clone, Copy)]
pub struct BoardInfo<'a> {
    pub name: &'a str,
    pub id: u8,
}

#[allow(unused)]
#[derive(Debug)]
pub struct VarValue<'a>{
    raw_data: &'a [u8],
    data_type: DataGenericType,
}

impl<'a> DpsMaster<'a> {
    pub fn new(master_id: u16, slaves_id: u16, send_f: SendFn) -> Self {
        Self {
            master_id,
            slaves_id,
            obj_ids_counter: 0,
            send_f,
            board_vec: Vec::new(),
        }
    }

    pub fn new_connection(&self) -> Result<(),CanError> {
        let master_mex = DpsMasterMex::new(0)?;
        let raw_mex = CanMessage {
            id:self.master_id,
            payload: master_mex.raw(),
        };

        (self.send_f)(&raw_mex)
    }

    pub fn request_info(&self) -> Result<(),CanError> {
        for board in self.board_vec.iter()
        {
            let mut master_mex = DpsMasterMex::new(1)?;
            let mut master_mode_1 = DpsMasterMexModeM1::new();
            master_mode_1.set_var_name_board_id(board.id)?;
            master_mex.set_m1(master_mode_1)?;
            let can_mex_raw = CanMessage{
                id: self.master_id,
                payload: master_mex.raw(),
            };
            let mut tries = 0;
            while (self.send_f)(&can_mex_raw).is_err() && tries < 32
            {
                tries+=1;
            }
        }
        Ok(())
    }

    pub fn list_board(&self) -> Box<[BoardInfo]> {
        let mut vec_board_res = Vec::with_capacity(self.board_vec.len());
        for board in self.board_vec.iter()
        {
            let board_info = BoardInfo{
                name: board.board_name,
                id: board.id,
            };
            vec_board_res.push(board_info);
        }
        vec_board_res.into_boxed_slice()
    }

    pub fn list_vars(&self, board_id: u8) -> Option<Box<[VarRecord<'a>]>> {
        let board = self.board_vec.iter().find(|b| b.id==board_id)?;
        let mut vec_res = Vec::with_capacity(board.vars.len());

        for var in board.vars.iter()
        {
            vec_res.push(*var);
        }

        Some(vec_res.into_boxed_slice())
    }

    pub fn refresh_value_var(&self, board_id: u8, var_id: u8) -> Result<(),CanError> {
        let board = self.board_vec.iter().find(|b| b.id==board_id);
        if board.is_none(){
            return Ok(());
        }
        let board = board.unwrap();
        let var = board.vars.iter().find(|v|v.id==var_id);
        if var.is_none(){
            return Ok(());
        }
        let var = var.unwrap();
        self._refresh_request_checked(board, var)
        
    }

    pub fn refresh_value_var_all(&self, board_id: u8) -> Result<(), CanError>{
        let board = self.board_vec.iter().find(|b| b.id==board_id);
        if board.is_none(){
            return Ok(());
        }
        let board = board.unwrap();

        for var in board.vars.iter(){
            let _ =self._refresh_request_checked(board, var);
        }
        Ok(())
    }

    pub fn get_value_var(&'a self, board_id: u8, var_id: u8) -> Option<VarValue<'a>>{
        let board = self.board_vec.iter().find(|b| b.id==board_id)?;
        let var = board.vars.iter().find(|v|v.id==var_id)?;

        Some(VarValue {
            raw_data: &var.value,
            data_type: var.data_type
        })

    }

    pub fn update_remote_var<T>(&self, board_id: u8, var_id: u8, data: T) -> Result<(), CanError>
    where T: Into<u32>
    {
        let board = self.board_vec.iter().find(|b| b.id==board_id);
        if board.is_none(){
            return Ok(());
        }
        let board = board.unwrap();
        let var = board.vars.iter().find(|v|v.id==var_id);
        if var.is_none(){
            return Ok(());
        }

        let mut master_mex =DpsMasterMex::new(4)?;
        let mut master_mex_mode_4 = DpsMasterMexModeM4::new();

        master_mex_mode_4.set_update_var_value_board_id(board_id)?;
        master_mex_mode_4.set_update_var_value_var_id(var_id)?;
        master_mex_mode_4.set_update_var_value_var_value(data.into())?;

        master_mex.set_m4(master_mex_mode_4)?;
        let raw_mex = CanMessage { id: self.master_id, payload: master_mex.raw() };
        let mut tries = 0;
        while (self.send_f)(&raw_mex).is_err() && tries < 32
        {
            tries+=1;
        }
        match tries 
        {
            32.. => Err(CanError::InvalidPayloadSize),
            _ => Ok(())
        }
    }

    pub fn check_mex_recv(&mut self, mex: &CanMessage) -> bool {
        todo!()
    }

    fn _refresh_request_checked(&self,board: &MasterRecord<'a>, var: &VarRecord) -> Result<(), CanError>
    {
        let master_mex = DpsMasterMex::new(3).ok();
        if master_mex.is_none(){
            return Err(CanError::InvalidMultiplexor {
                message_id: u32::from(self.master_id),
                multiplexor: 3
            });
        }
        let mut master_mex= master_mex.unwrap();
        let mut master_mex_mode_3 = DpsMasterMexModeM3::new();
        master_mex_mode_3.set_var_value_board_id(board.id)?;
        master_mex_mode_3.set_var_value_var_id(var.id)?;
        master_mex.set_m3(master_mex_mode_3)?;

        let mut tries =0;
        while 
            tries < 32 &&
                (self.send_f)(&CanMessage{id: self.master_id, payload: master_mex.raw()}).is_err()
        {
            tries+=1;
        }

        Ok(())
    }
}
