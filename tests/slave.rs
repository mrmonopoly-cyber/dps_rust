mod common;

use common::*;
use dps::slave::node::*;

macro_rules! monitor_var {
    ($slave:expr, $v:expr, $pf: expr) => {
        $slave.monitor_var(stringify!($v),&mut $v,$pf)
            .inspect_err(|e| println!("{}",e))
            .unwrap();
    };
}

#[test]
fn init_slave() {
    let mut slave = DpsSlave::new("boa0", send_f, 0, 650, 651)
        .expect("init slave failed");
    slave.enable();
}

#[test]
fn monitor_var() {
    let mut vu8 : u8 = 0;
    let mut vu16 : u16 = 0;
    let mut vu32 : u32 = 0;

    let mut vi8 : i8 = 0;
    let mut vi16 : i16 = 0;
    let mut vi32 : i32 = 0;

    let mut vf32 : f32 = 0.0;

    let mut slave = DpsSlave::new("brd_0", send_f, 0, 650, 651)
        .expect("init slave failed");

    slave.enable();

    monitor_var!(slave,vu8, None);
    monitor_var!(slave,vu16, None);
    monitor_var!(slave,vu32, None);

    monitor_var!(slave,vi8, None);
    monitor_var!(slave,vi16, None);
    monitor_var!(slave,vi32, None);

    monitor_var!(slave,vf32, None);
}
