use std::sync::{Mutex, Arc};
use std::marker::{Send, Sync};
use std::thread;
use std::time::Duration;

use tokio::task::JoinHandle;

use websocket::sync::{Server};
use websocket::{OwnedMessage};

use tracing::{trace};

use net_pack::NetPack;
use queue::Queue;
use close_handle::CloseHandle;
use timer::utc_unix_timer;

pub struct WSServer{
    pub join: JoinHandle<()>
}

impl WSServer {
    pub async fn listen<H: Send + Sync + 'static>(host:String, f:fn(_handle: Arc<Mutex<H>>, rsp: Arc<Mutex<Queue<Vec<u8>>>>, data:Vec<u8>), _handle: Arc<Mutex<H>>, _close: Arc<Mutex<CloseHandle>>) -> Result<WSServer, Box<dyn std::error::Error>> {
        let server = Server::bind(host)?;
        let _clone_handle = _handle.clone();
        let _clone_close = _close.clone();
        let _join = tokio::spawn(async move {
            for request in server.filter_map(Result::ok) {
                if !request.protocols().contains(&"websocket".to_string()) {
                    request.reject().unwrap();
                    return;
                }

                let mut _client = request.use_protocol("websocket").accept().unwrap();
                let ip = _client.peer_addr().unwrap();
                let (mut rd, wr) = _client.split().unwrap();
                let mut _wr_arc = Arc::new(Mutex::new(wr));
                let net_rsp:Arc<Mutex<Queue<Vec<u8>>>> = Arc::new(Mutex::new(Queue::new()));

                let _clone_net_rsp = net_rsp.clone();
                let _wr_clone = _wr_arc.clone();
                let _clone_h = _clone_handle.clone();
                let _clone_c = _clone_close.clone();
                tokio::spawn(async move {
                    let mut net_pack = NetPack::new();
                    
                    loop {
                        let message = rd.recv_message().unwrap();
                        match message {
                            OwnedMessage::Close(_) => {
                                let message = OwnedMessage::Close(None);
                                let mut wr = _wr_clone.as_ref().lock().unwrap();
                                wr.send_message(&message).unwrap();
                                trace!("client {} disconnected", ip);
                                return;
                            },
                            OwnedMessage::Ping(ping) => {
                                let message = OwnedMessage::Pong(ping);
                                let mut wr = _wr_clone.as_ref().lock().unwrap();
                                wr.send_message(&message).unwrap();
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

                        let _c_ref = _clone_c.as_ref().lock().unwrap();
                        if _c_ref.is_closed() {
                            break;
                        }
                    }
                });

                let _clone_c = _clone_close.clone();
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
                        let mut wr = _wr_arc.as_ref().lock().unwrap();
                        let _ = wr.send_message(&msg).unwrap();

                        let tick = utc_unix_timer() - begin;

                        let _c_ref = _clone_c.as_ref().lock().unwrap();
                        if _c_ref.is_closed() {
                            break;
                        }

                        if tick < 33 {
                            thread::sleep(Duration::from_millis((33 - tick) as u64));
                        }
                    }
                });  

                let _c_ref = _clone_close.as_ref().lock().unwrap();
                if _c_ref.is_closed() {
                    break;
                }              
            }
        });

        Ok(WSServer {
            join: _join
        })
    }
}