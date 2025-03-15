/*
 *
 *
 * void* p_var;
 * post_update post_update_fun;
 * char var_name[VAR_NAME_LENGTH];
 * uint8_t size;
 * uint8_t var_id : DATA_ID_SIZE_BIT;
 * enum DATA_GENERIC_TYPE type: 2;
 *
 */

use alloc::vec::Vec;

use crate::common::*;
use crate::master::VarRecord;

#[derive(Debug)]
struct VarRecordSlave<'a> {
    ref_var: &'a[u8],
    var_name: [u8;VAR_NAME_LENGTH],
    var_id: u8, //4 bits
    data_type: DataGenericType,
    post_update_f: PostUpdateFn<'a>,
}

/*
 *
 *
 * struct DpsSlave_t{
 *   char board_name[BOARD_NAME_LENGTH];
 *   can_send send_f;
 *   c_vector_h* vars;
 *   int8_t board_id;
 *   uint8_t obj_ids;
 *   uint16_t master_id;
 *   uint16_t slave_id;
 *   uint8_t enable : 1;
 * };
 *
 *
 */

//typedef int8_t (*post_update) (const char* const var_name, const void* const var);
pub type PostUpdateFn<'a> = fn(var_name: &'a str, var_data: &'a[u8]);

#[derive(Debug)]
pub struct DpsSlave<'a> {
    board_name: [u8;BOARD_NAME_LENGTH],
    send_f: SendFn,
    vars: Vec<VarRecordSlave<'a>>,
    board_id: u8,
    obj_ids: u8,
    master_id: u8,
    slave_id: u8,
    enable: bool,
}

impl<'a> DpsSlave<'a> {
    pub fn new(
        board_name: [u8;BOARD_NAME_LENGTH],
        send_f: SendFn,
        board_id: u8,
        master_id: u8,
        slave_id: u8) ->Self{
        Self{
            board_name,
            send_f,
            vars: Vec::new(),
            board_id,
            obj_ids:0,
            master_id,
            slave_id,
            enable:false,
        }
    }

    pub fn enable(&mut self) {
        self.enable = true
    }

    pub fn disable(&mut self) {
        self.enable = true
    }

    pub fn monitor_primitive_var (
        &mut self,
        data_type: DpsPrimitiveTypes,
        var: &'a[u8],
        post_update_f : PostUpdateFn<'a>,
        var_name: [u8;VAR_NAME_LENGTH]
        )
    {
        use DataGenericType::*;
        let mut new_var = VarRecordSlave{
            ref_var: var,
            var_name,
            var_id: self.obj_ids,
            data_type: Unsigned,
            post_update_f
        };
        self.obj_ids+=1;
        match data_type 
        {
            DpsPrimitiveTypes::DpsTypesUint8T |
            DpsPrimitiveTypes::DpsTypesUint16T |
            DpsPrimitiveTypes::DpsTypesUint32T => new_var.data_type = Unsigned,
            DpsPrimitiveTypes::DpsTypesInt8T |
            DpsPrimitiveTypes::DpsTypesInt16T |
            DpsPrimitiveTypes::DpsTypesInt32T => new_var.data_type = Signed,
            DpsPrimitiveTypes::DpsTypesFloatT => new_var.data_type = Floated,
        };
        self.vars.push(new_var);
    }

    pub fn check_can_mex_recv(&mut self,mex: &CanMessage) ->Result<(),()> {
        todo!()
    }
    
}
