mod common;

use std::sync::Arc;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

use dps::common::CanMessage;
use dps::master::node::DpsMaster;
use dps::slave::node::DpsSlave;

use dps;
use socketcan::Socket;

use self::common::CAN_INTERFACE;

const MASTER_ID : u16 = 65;
const SLAVES_ID: u16 = 78;

#[allow(unused)]
#[derive(Debug)]
struct Slave<'a>
{
    dps_slave: DpsSlave<'a>,
    can_node: socketcan::CanSocket,
}


#[allow(unused)]
impl<'a> Slave<'a>{
    pub fn new(board_name: &str, board_id: u8, master_id: u16, slave_id : u16) -> Self {
        let can_node =socketcan::CanSocket::open(CAN_INTERFACE).ok().unwrap(); 
        let dps_slave = DpsSlave::new(board_name, common::send_f, board_id, master_id, slave_id).expect("init slave error");

        Self{
            dps_slave,
            can_node,
        }
    }

    pub fn update_loop(&mut self)
    {
        loop{
            let frame = self.can_node.read_raw_frame().ok().unwrap();
            let message =&CanMessage{
                id: frame.can_id.try_into().unwrap(),
                payload: &frame.data}; 
            if frame.can_id == MASTER_ID.into(){
                let res = self.dps_slave.check_can_mex_recv(&message);
                match res {
                    Ok(_) => (),
                    Err(e) => println!("{}",e),
                };
            }
        }
    }
}



#[test]
fn init_master() {
    DpsMaster::<1>::new(MASTER_ID, SLAVES_ID, common::send_f);
}


#[test]
fn discover_req() {
    thread::spawn(move ||
        {
            let mut u8 = 2_u8;
            let b_name = str::from_utf8(&*b"slave_1").unwrap();
            let mut slave = Slave::new(b_name, 0, MASTER_ID, SLAVES_ID);

            slave.dps_slave.enable();

            assert_eq!(slave.dps_slave.monitor_var("u8", &mut u8, None),Ok(()));

            slave.update_loop();
        });

    thread::spawn(move ||
        {
            let mut u16 = 512_u16;
            let mut f32 = 4.3_f32;
            let b_name = str::from_utf8(&*b"slave_2").unwrap();
            let mut slave = Slave::new(b_name, 1, MASTER_ID, SLAVES_ID);

            slave.dps_slave.enable();

            assert_eq!(slave.dps_slave.monitor_var("u16", &mut u16, None),Ok(()));
            assert_eq!(slave.dps_slave.monitor_var("f32", &mut f32, None),Ok(()));

            slave.update_loop();
        });

    thread::spawn(move ||
        {
            let mut i16 = -512_i16;
            let mut u32 = 1024_u32;
            let b_name = str::from_utf8(&*b"slave_3").unwrap();
            let mut slave = Slave::new(b_name, 2, MASTER_ID, SLAVES_ID);

            slave.dps_slave.enable();

            assert_eq!(slave.dps_slave.monitor_var("i16", &mut i16, None),Ok(()));
            assert_eq!(slave.dps_slave.monitor_var("u32", &mut u32, None),Ok(()));

            slave.update_loop();
        });

    let master = DpsMaster::<10>::new(MASTER_ID,SLAVES_ID, common::send_f);
    let mutex = Arc::new(std::sync::Mutex::new(master));
    let master_updated = mutex.clone();
    thread::spawn(move||
        {
            let can_node =socketcan::CanSocket::open(CAN_INTERFACE).ok().unwrap(); 
            loop{
                let frame =can_node.read_raw_frame().ok().unwrap();
                let mex = CanMessage{
                    id: frame.can_id.try_into().unwrap(),
                    payload: &frame.data
                };
                if mex.id == SLAVES_ID{
                    let mut master_updated = master_updated.lock().ok().unwrap();
                    match master_updated.check_mex_recv(&mex)
                    {
                        Ok(true) =>println!("recv dps slave mex"),
                        Ok(false) => println!("incorrect slaves id"),
                        Err(_) => println!("error"),
                    };
                };
            }
        });

    {
        let master = mutex.lock().unwrap();
        assert_eq!(master.new_connection().is_ok(),true);
    }

    sleep(Duration::from_millis(500));

    {
        let master = mutex.lock().unwrap();
        assert_eq!(master.new_connection().is_ok(),true);
        let boards = master.list_board();

        for board in boards.iter().flatten(){
            match board.id
                {
                    0 => assert_eq!(board.name,"slave_1\0"),
                    1 => assert_eq!(board.name,"slave_2\0"),
                    2 => assert_eq!(board.name,"slave_3\0"),
                    _ => panic!("invalid board: name: {}, id: {}",board.name, board.id),
                }
        }
    }

}
