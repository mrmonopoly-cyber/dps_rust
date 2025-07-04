mod common;

use std::thread;
use std::thread::sleep;
use std::time::Duration;
use std::sync::{Arc,Mutex,MutexGuard};

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
#[derive(Debug)]
struct Master<const N:usize>
{
    dps_master: Arc<Mutex<DpsMaster<N>>>,
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

impl<const N:usize> Master<N> {
    fn _master_update(master_updated: Arc<Mutex<DpsMaster<N>>>) {
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


    }

    pub fn new(master_id: u16, slaves_id: u16, send_f: dps::common::SendFn) ->Self {
        let dps_master = Arc::new(Mutex::new(DpsMaster::new(master_id, slaves_id, send_f)));
        Master::_master_update(dps_master.clone());
        Self{
            dps_master,
        }
    }

    pub fn execute_on_master(&self, apply_f:  fn(& MutexGuard<DpsMaster<N>>))
    {
        let ref_count = self.dps_master.clone();
        let master = ref_count.lock().unwrap();
        apply_f(&master)
        
    }
    
}

macro_rules! spawn_slave {
    ($name:literal, $index:expr, [$(($type:ty, $var:ident, $value:expr)),* $(,)?]) => {
        std::thread::spawn(move || {
            $(
                let mut $var : $type = $value;
            )*

            let b_name = std::str::from_utf8(&*$name).unwrap();
            let mut slave = Slave::new(b_name, $index, MASTER_ID, SLAVES_ID);

            slave.dps_slave.enable();

            $(
                assert_eq!(slave.dps_slave.monitor_var(stringify!($var), &mut $var, None), Ok(()));
            )*

            slave.update_loop();
        });
    };
}


#[test]
fn init_master() {
    DpsMaster::<1>::new(MASTER_ID, SLAVES_ID, common::send_f);
}

#[test]
fn discover_req() {

    spawn_slave!(b"slave_1",0,[]);
    spawn_slave!(b"slave_2",1,[]);
    spawn_slave!(b"slave_3",2,[]);

    let master = Master::<10>::new(MASTER_ID, SLAVES_ID, common::send_f);

    master.execute_on_master(|master|{
        assert_eq!(master.new_connection().is_ok(),true);
    });

    sleep(Duration::from_millis(10));

    master.execute_on_master(|master|{
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
    });

}
