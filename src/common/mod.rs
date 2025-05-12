pub mod types;

#[allow(unused, dead_code)]
pub mod messages;

pub const BOARD_NAME_LENGTH : usize  =7;
pub const VAR_NAME_LENGTH : usize= 6;

#[derive(Debug,Clone, Copy)]
pub enum DataGenericType {
    Unsigned = 0,
    Signed = 1,
    Floated = 2,
}

#[derive(Debug)]
pub struct CanMessage<'a>{
    pub id: u16,
    pub payload: &'a[u8],
}

pub type SendFn = fn(&CanMessage) -> Result<(),messages::CanError>;
