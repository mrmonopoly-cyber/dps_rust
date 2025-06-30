use std::cell::LazyCell;
use std::process::Command;
use dps::common::*;
use socketcan::{EmbeddedFrame, Socket};


pub fn send_f(mex:&CanMessage<'_>) -> Result<(), dps::common::messages::CanError>
{
    let can_node = socketcan::CanSocket::open(CAN_INTERFACE).ok().unwrap();
    let id = socketcan::StandardId::new(mex.id).unwrap();
    let frame = socketcan::frame::CanFrame::new(id, &mex.payload).unwrap();
    let res =can_node.write_frame(&frame);
    match res {
        Ok(_) => Ok(()),
        Err(_) => Err(dps::common::messages::CanError::InvalidPayloadSize),
    }
}

struct InitNode;

/*
 *
 * sudo ip link add dev $1 type vcan
 * sudo ip link set up $1
 *
 */
impl InitNode {
    pub fn new() -> Self {
        let _ = Command::new("sudo")
            .args(["ip", "link", "add", "dev", CAN_INTERFACE, "type", "vcan"])
            .status();
        let _ = Command::new("sudo")
            .args(["ip", "link", "set", "up", CAN_INTERFACE])
            .status();
        InitNode{}
    }
    
}

pub const CAN_INTERFACE: &str = "dps_vcan";
const INIT_CAN_NODE : LazyCell<InitNode>= LazyCell::new(||InitNode::new());

#[ctor::ctor]
fn init_globals() {
    LazyCell::force(&INIT_CAN_NODE);
}
