use std::sync::{Mutex, Arc};
use std::marker::{Send, Sync};

use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::task::JoinHandle;

use tracing::{trace, debug, info, warn, error};

use net_pack::NetPack;
use queue::Queue;

pub struct TcpConnect {
    join: JoinHandle<()>
}

impl TcpConnect {
    pub async fn new<H: Send + Sync + 'static>(host:String, f:fn(_handle: Arc<Mutex<H>>, rsp: Arc<Mutex<Queue<Vec<u8>>>>, data:Vec<u8>), _handle: Arc<Mutex<H>>) -> Result<TcpConnect, Box<dyn std::error::Error>> {
        let mut _socket = TcpStream::connect(host).await?;

        let _join = tokio::spawn(async move {
            let mut buf = vec![0; 1024];
            let mut net_pack = NetPack::new();
            let net_rsp:Arc<Mutex<Queue<Vec<u8>>>> = Arc::new(Mutex::new(Queue::new()));

            let (mut rd, mut wr) = io::split(_socket);
            loop {
                match rd.read(&mut buf).await {
                    Ok(0) => return,
                    Ok(n) => {
                        net_pack.input(&buf[..n]);
                        match net_pack.try_get_pack() {
                            None => continue,
                            Some(data) => {
                                let _h = _handle.clone();
                                let _rsp = net_rsp.clone();
                                f(_h, _rsp, data);
                            }
                        }
                    }
                    Err(err) => {
                        error!("network err:{}!", err);
                        return;
                    }
                }

                let mut wait_send_data: Vec<u8> = Vec::new(); 
                loop {
                    let mut _net_rsp = net_rsp.as_ref().lock().unwrap();
                    let opt_send_data = _net_rsp.deque();
                    match opt_send_data {
                        None => break,
                        Some(mut send_data) => {
                            wait_send_data.append(&mut send_data);
                        }
                    }
                }
                let _ = wr.write_all(&wait_send_data).await;
            }
        });

        Ok(TcpConnect {
            join: _join
        })
    }
}
