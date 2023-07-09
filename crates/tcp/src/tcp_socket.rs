use std::sync::{Mutex, Arc};
use std::marker::{Send, Sync};

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::io::{ReadHalf, WriteHalf};
use tokio::net::TcpStream;
use tokio::task::JoinHandle;

use tracing::{error};

use async_trait::async_trait;

use net::{NetWriter, NetReader, NetPack};
use close_handle::CloseHandle;

pub struct TcpReader {
    rd: ReadHalf<TcpStream>
}

impl TcpReader {
    pub fn new(_rd: ReadHalf<TcpStream>) -> TcpReader {
        TcpReader { 
            rd: _rd
        }
    }
}

impl NetReader for TcpReader {
    fn start<H: Send + Sync + 'static, S: NetWriter + Send + 'static>(self, 
        f:fn(h: Arc<Mutex<H>>, s: Arc<Mutex<S>>, data:Vec<u8>), 
        h: Arc<Mutex<H>>, 
        s: Arc<Mutex<S>>,  
        c: Arc<Mutex<CloseHandle>>) -> JoinHandle<()>
    {
        let mut _p = self;
        tokio::spawn(async move {
            let mut buf = vec![0; 1024];
            let mut net_pack = NetPack::new();

            loop {
                match _p.rd.read(&mut buf).await {
                    Ok(0) => return,
                    Ok(n) => {
                        net_pack.input(&buf[..n]);
                        match net_pack.try_get_pack() {
                            None => continue,
                            Some(data) => {
                                let _h = h.clone();
                                let _s = s.clone();
                                f(_h, _s, data);
                            }
                        }
                    }
                    Err(err) => {
                        error!("network err:{}!", err);
                        return;
                    }
                }

                let _c_ref = c.as_ref().lock().unwrap();
                if _c_ref.is_closed() {
                    break;
                }
            }
        })
    }
}

pub struct TcpWriter {
    wr: WriteHalf<TcpStream>, 
}

impl TcpWriter {
    pub fn new(_wr: WriteHalf<TcpStream>) -> TcpWriter {
        TcpWriter{
            wr: _wr
        }
    }
}

#[async_trait]
impl NetWriter for TcpWriter {
    async fn send(&mut self, buf: &[u8]) -> bool {
        match self.wr.write_all(buf).await {
            Err(_) => {
                return false;
            },
            Ok(_) => {
                return true;
            }
        }
    }

    async fn close(&mut self) {
        let _ = self.wr.shutdown().await;
    }
}