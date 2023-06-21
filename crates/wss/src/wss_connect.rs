use std::sync::{Mutex, Arc};
use std::marker::{Send, Sync};
use std::thread;
use std::time::Duration;

use tokio::task::JoinHandle;

use websocket::client::ClientBuilder;
use websocket::{OwnedMessage};

use tracing::{error};

use net_pack::NetPack;
use queue::Queue;
use close_handle::CloseHandle;
use timer::utc_unix_timer;

pub struct WssConnect {
    pub join: JoinHandle<()>
}

impl WssConnect {
    pub async fn connect<H: Send + Sync + 'static>(host:String, f:fn(_handle: Arc<Mutex<H>>, rsp: Arc<Mutex<Queue<Vec<u8>>>>, data:Vec<u8>), _handle: Arc<Mutex<H>>, _close: Arc<Mutex<CloseHandle>>) -> Result<WssConnect, Box<dyn std::error::Error>> {
        let _client = ClientBuilder::new(&host)
            .unwrap()
            .add_protocol("rust-websocket")
            .connect_insecure()
            .unwrap();

        let net_rsp:Arc<Mutex<Queue<Vec<u8>>>> = Arc::new(Mutex::new(Queue::new()));
        let (mut rd, wr) = _client.split().unwrap();
        let mut _wr_arc = Arc::new(Mutex::new(wr));

        let _wr_clone = _wr_arc.clone();
        let _clone_net_rsp = net_rsp.clone();
        let _clone_h = _handle.clone();
        let _clone_c = _close.clone();
        let _join = tokio::spawn(async move {
            let mut net_pack = NetPack::new();

            loop {
                for message in rd.incoming_messages() {
                    let message = match message {
                        Ok(m) => m,
                        Err(e) => {
                            error!("WssConnect Receive Loop incoming_messages: {:?}", e);
                            return;
                        }
                    };
                    match message {
                        OwnedMessage::Close(_) => {
                            return;
                        },
                        OwnedMessage::Ping(data) => {
                            let mut wr = _wr_arc.as_ref().lock().unwrap();
                            match wr.send_message(&OwnedMessage::Pong(data)) {
                                Ok(()) => (),
                                Err(e) => {
                                    error!("WssConnect Receive Loop send_message Pong: {:?}", e);
                                    return;
                                }
                            }
                        },
                        OwnedMessage::Binary(buf) => {
                            net_pack.input(&buf[..]);
                            match net_pack.try_get_pack() {
                                None => continue,
                                Some(data) => {
                                    let _h = _clone_h.clone();
                                    let _rsp = net_rsp.clone();
                                    f(_h, _rsp, data);
                                }
                            }
                        },
                        _ => {}
                    }
                }
            }
        });

        tokio::spawn(async move {
            loop {
                let begin = utc_unix_timer();

                let mut wait_send_data: Vec<u8> = Vec::new(); 
                loop {
                    let mut _net_rsp = _clone_net_rsp.as_ref().lock().unwrap();
                    let opt_send_data = _net_rsp.deque();
                    match opt_send_data {
                        None => break,
                        Some(mut send_data) => {
                            wait_send_data.append(&mut send_data);
                        }
                    }
                }
                let msg = OwnedMessage::Binary(wait_send_data);
                let mut wr = _wr_clone.as_ref().lock().unwrap();
                let _ = wr.send_message(&msg).unwrap();

                let tick = utc_unix_timer() - begin;

                let _c_ref = _close.as_ref().lock().unwrap();
                if _c_ref.is_closed() {
                    break;
                }

                if tick < 33 {
                    thread::sleep(Duration::from_millis((33 - tick) as u64));
                }
            }
        });

        Ok(WssConnect {
            join: _join
        })
    }
}
