pub mod types;

#[allow(unused, dead_code)]
pub mod messages;

pub const BOARD_NAME_LENGTH : usize  =7;
pub const VAR_NAME_LENGTH : usize= 6;
pub const CAN_PAYLOAD_MAX_SIZE_CAN_BASE : usize = 8;

#[derive(Debug,Clone, Copy)]
pub enum DataGenericType {
    Unsigned = 0,
    Signed = 1,
    Floated = 2,
}

#[derive(Debug)]
pub enum DpsPrimitiveTypes{
    DpsTypesUint8T,
    DpsTypesUint16T,
    DpsTypesUint32T,
    DpsTypesInt8T,
    DpsTypesInt16T,
    DpsTypesInt32T,
    DpsTypesFloatT,
}


#[derive(Debug)]
pub struct CanMessage<'a>{
    pub id: u16,
    pub payload: &'a[u8],
}

pub type SendFn = fn(&CanMessage) -> Result<(),messages::CanError>;
