use alloc::vec::Vec;
use core::result::Result;

use crate::common::*;

// char name[VAR_NAME_LENGTH];
// uint32_t value;
// uint8_t size;
// enum DATA_GENERIC_TYPE type:2;
// uint8_t updated : 1;

#[derive(Debug)]
pub struct VarRecord{
    name: [u8;VAR_NAME_LENGTH],
    value: u32,
    size: u8,
    data_type: DataGenericType,
    updated: bool,

}

#[derive(Debug)]
struct MasterRecord{
    id: u8,
    vars: Vec<VarRecord>,
}

#[derive(Debug)]
pub struct DpsMaster where 
{
    master_id: u16,
    slaves_id: u16,
    obj_ids_counter: u8, //size 4 bits
    send_f: SendFn,
    board_vec: Vec<MasterRecord>,
}

#[derive(Debug)]
pub struct BoardInfo<'a>{
    pub name: &'a str, 
    pub id: u8,
}

impl DpsMaster {
    pub fn new(master_id:u16, slaves_id: u16, send_f: SendFn) -> Self{
        Self {
            master_id,
            slaves_id,
            obj_ids_counter: 0,
            send_f,
            board_vec:Vec::new()}
    }

    pub fn new_connection(&self){
        todo!()
    }

    pub fn request_info(&self) {
        todo!()
    }

    pub fn list_board(&self) -> &[BoardInfo] {
        todo!()
    }

    pub fn list_vars(&self) -> &[VarRecord] {
        todo!()
    }

    pub fn refresh_value_var(&self, board_id: u8, var_id: u8)
    {
        todo!()
    }

    pub fn refresh_value_var_all(&self, board_id: u8) {
        todo!()
    }

    pub fn get_value_var() -> VarRecord {
        todo!()
    }

    pub fn update_remote_var(&self, board_id: u8, var_id: u8, data: &[u8])
    {
        todo!()
    }

    pub fn check_mex_recv(&mut self, mex: &CanMessage) -> bool {
        todo!()
        
    }
}

