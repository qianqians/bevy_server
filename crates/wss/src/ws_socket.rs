use std::sync::{Mutex, Arc};
use std::marker::{Send, Sync};
use std::net::{SocketAddr, TcpStream};

use tokio::task::JoinHandle;

use websocket::sync::{Reader, Writer};
use websocket::{OwnedMessage};

use tracing::{trace};

use async_trait::async_trait;

use net::{NetWriter, NetReader, NetPack};
use close_handle::CloseHandle;

pub struct WSReader {
    ip: SocketAddr,
    rd: Reader<TcpStream>, 
    wr: Arc<Mutex<Writer<TcpStream>>>,
}

impl WSReader {
    pub fn new(_ip: SocketAddr, _rd: Reader<TcpStream>, _wr: Arc<Mutex<Writer<TcpStream>>>) -> WSReader {
        WSReader { 
            ip: _ip,
            rd: _rd, 
            wr: _wr
        }
    }
}

impl NetReader for WSReader {
    fn start<H: Send + Sync + 'static, S: NetWriter + Send + 'static>(self, 
        f:fn(h: Arc<Mutex<H>>, s: Arc<Mutex<S>>, data:Vec<u8>), 
        h: Arc<Mutex<H>>, 
        s: Arc<Mutex<S>>,  
        c: Arc<Mutex<CloseHandle>>) -> JoinHandle<()>
    {
        let mut _p = self;
        tokio::spawn(async move {
            let mut net_pack = NetPack::new();
            loop {
                let message = _p.rd.recv_message().unwrap();
                match message {
                    OwnedMessage::Close(_) => {
                        let message = OwnedMessage::Close(None);
                        let mut wr = _p.wr.as_ref().lock().unwrap();
                        wr.send_message(&message).unwrap();
                        trace!("client {} disconnected", _p.ip);
                        return;
                    },
                    OwnedMessage::Ping(ping) => {
                        let message = OwnedMessage::Pong(ping);
                        let mut wr = _p.wr.as_ref().lock().unwrap();
                        wr.send_message(&message).unwrap();
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
        })
    }
}

pub struct WSWriter {
    wr: Arc<Mutex<Writer<TcpStream>>>, 
}

impl WSWriter {
    pub fn new(_wr: Arc<Mutex<Writer<TcpStream>>>) -> WSWriter {
        WSWriter{
            wr: _wr
        }
    }
}

#[async_trait]
impl NetWriter for WSWriter {
    async fn send(&mut self, buf: &[u8]) -> bool {
        let mut wr = self.wr.as_ref().lock().unwrap();
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

    async fn close(&mut self) {
        let wr = self.wr.as_ref().lock().unwrap();
        let _ = wr.shutdown_all();
    }
}