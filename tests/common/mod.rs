use std::os::unix::process::CommandExt;
use std::process::{self, Command};

/*
 *
 * sudo ip link add dev $1 type vcan
 * sudo ip link set up $1
 *
 */

const CAN_INTERFACE: &str = "dps_vcan";

pub fn setup(){
    let _ = Command::new("sudo")
        .args(["ip", "link", "add", "dev", CAN_INTERFACE, "type", "vcan"])
        .status();
    let _ = Command::new("sudo")
        .args(["ip", "link", "set", "up", CAN_INTERFACE])
        .status();
}
