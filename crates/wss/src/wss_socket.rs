use std::sync::{Mutex, Arc};
use std::marker::{Send, Sync};

use tokio::task::JoinHandle;
use websocket::{OwnedMessage};
use tracing::{trace};
use async_trait::async_trait;

use net::{NetWriter, NetReader, NetPack};
use close_handle::CloseHandle;

pub struct WSSReader {
    s: Arc<Mutex<websocket::sync::Client<websocket::native_tls::TlsStream<std::net::TcpStream>>>>,
    join: Option<JoinHandle<()>>
}


impl WSSReader {
    pub fn new(_s: Arc<Mutex<websocket::sync::Client<websocket::native_tls::TlsStream<std::net::TcpStream>>>>) -> WSSReader {
        WSSReader { 
            s: _s,
            join: None 
        }
    }

    pub async fn join(self) {
        let j = match self.join {
            None => return,
            Some(_j) => _j
        };
        let _ = j.await;
    }
}

impl NetReader for WSSReader {
    fn start<H: Send + Sync + 'static, S: NetWriter + Send + 'static>(self, f:fn(h: Arc<Mutex<H>>, s: Arc<Mutex<S>>, data:Vec<u8>), h: Arc<Mutex<H>>, s: Arc<Mutex<S>>,  c: Arc<Mutex<CloseHandle>>) {
        let mut _p = self;
        _p.join = Some(tokio::spawn(async move {
            let mut net_pack = NetPack::new();
            loop {
                let mut _client_ref = _p.s.as_ref().lock().unwrap();
                let message = _client_ref.recv_message().unwrap();
                match message {
                    OwnedMessage::Close(_) => {
                        let message = OwnedMessage::Close(None);
                        _client_ref.send_message(&message).unwrap();
                        trace!("client {} disconnected", _client_ref.peer_addr().unwrap());
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
                                let _h = h.clone();
                                let _rsp = s.clone();
                                f(_h, _rsp, data);
                            }
                        }
                    },
                    _ => {}
                }

                let _c_ref = c.as_ref().lock().unwrap();
                if _c_ref.is_closed() {
                    break;
                }
            }
        }));
    }
}

pub struct WSSWriter {
    s: Arc<Mutex<websocket::sync::Client<websocket::native_tls::TlsStream<std::net::TcpStream>>>>
}

impl WSSWriter {
    pub fn new(_s: Arc<Mutex<websocket::sync::Client<websocket::native_tls::TlsStream<std::net::TcpStream>>>>) -> WSSWriter {
        WSSWriter{
            s: _s
        }
    }
}

#[async_trait]
impl NetWriter for WSSWriter {
    async fn send(&mut self, buf: &[u8]) -> bool {
        let mut wr = self.s.as_ref().lock().unwrap();
        let msg = OwnedMessage::Binary(buf.to_vec());
        match wr.send_message(&msg) {
            Err(_) => {
                return false;
            },
            Ok(_) => {
                return true;
            }
        }
    }
}
