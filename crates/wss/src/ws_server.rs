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

pub struct WssServer{
    pub join: JoinHandle<()>
}

impl WssServer {
    pub async fn listen<H: Send + Sync + 'static>(host:String, f:fn(_handle: Arc<Mutex<H>>, rsp: Arc<Mutex<Queue<Vec<u8>>>>, data:Vec<u8>), _handle: Arc<Mutex<H>>, _close: Arc<Mutex<CloseHandle>>) -> Result<WssServer, Box<dyn std::error::Error>> {
        let server = Server::bind(host)?;
        let _clone_handle = _handle.clone();
        let _clone_close = _close.clone();
        let _join = tokio::spawn(async move {
            for request in server.filter_map(Result::ok) {
                if !request.protocols().contains(&"websocket".to_string()) {
                    request.reject().unwrap();
                    return;
                }

                let mut _client = Arc::new(Mutex::new(request.use_protocol("websocket").accept().unwrap()));
                let net_rsp:Arc<Mutex<Queue<Vec<u8>>>> = Arc::new(Mutex::new(Queue::new()));

                let _clone_net_rsp = net_rsp.clone();
                let _clone_client = _client.clone();
                let _clone_h = _clone_handle.clone();
                let _clone_c = _clone_close.clone();
                tokio::spawn(async move {
                    let mut net_pack = NetPack::new();
                    
                    loop {
                        let mut _client_ref = _client.as_ref().lock().unwrap();
                        let message = _client_ref.recv_message().unwrap();
                        match message {
                            OwnedMessage::Close(_) => {
                                let message = OwnedMessage::Close(None);
                                _client_ref.send_message(&message).unwrap();
                                let ip = _client_ref.peer_addr().unwrap();
                                trace!("client {} disconnected", ip);
                                return;
                            },
                            OwnedMessage::Ping(ping) => {
                                let message = OwnedMessage::Pong(ping);
                                _client_ref.send_message(&message).unwrap();
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
                        let mut _client_ref = _clone_client.as_ref().lock().unwrap();
                        let _ = _client_ref.send_message(&msg).unwrap();

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

        Ok(WssServer {
            join: _join
        })
    }
}