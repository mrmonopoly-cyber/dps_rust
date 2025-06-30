mod common;

use std::thread::{self, sleep};
use std::time::Duration;

use dps::common::CanMessage;
use dps::master::node::DpsMaster;
use dps::slave::node::DpsSlave;

use dps;
use socketcan::Socket;

use self::common::CAN_INTERFACE;

const MASTER_ID : u16 = 650;
const SLAVES_ID: u16 = 651;

pub fn start(_f: impl FnOnce(&mut DpsSlave) + Send + 'static){
    thread::spawn(move ||{
        let mut slave = DpsSlave::new("sl1", common::send_f, 0, MASTER_ID, SLAVES_ID).ok().unwrap();
        let can_node = socketcan::CanSocket::open(CAN_INTERFACE).ok().unwrap();
        slave.enable();
        _f(&mut slave);
        loop {
            if let Ok(frame) = can_node.read_raw_frame()
            {
                let id = frame.can_id.try_into().unwrap();
                let mex = CanMessage{ id, payload: &frame.data};
                let _ =slave.check_can_mex_recv(&mex);
            }
        }
    });
}

#[test]
fn init_master() {
    DpsMaster::<1>::new(MASTER_ID, SLAVES_ID, common::send_f);
}


#[test]
fn discover_req() {
    let master = DpsMaster::<1>::new(MASTER_ID, SLAVES_ID, common::send_f);

    master.new_connection().ok().unwrap();

    sleep(Duration::from_millis(100));

}
