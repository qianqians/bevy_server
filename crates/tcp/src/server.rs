use std::sync::{Mutex, Arc};
use std::marker::{Send, Sync};

use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::task::JoinHandle;

use tracing::{trace, debug, info, warn, error};

use net_pack::NetPack;
use queue::Queue;
use close_handle::CloseHandle;

pub struct TcpServer{
    pub join: JoinHandle<()>
}

impl TcpServer {
    pub async fn listen<H: Send + Sync + 'static>(host:String, f:fn(_handle: Arc<Mutex<H>>, rsp: Arc<Mutex<Queue<Vec<u8>>>>, data:Vec<u8>), _handle: Arc<Mutex<H>>, _close: Arc<Mutex<CloseHandle>>) -> Result<TcpServer, Box<dyn std::error::Error>> {
        let _listener = TcpListener::bind(host).await?;
        let _clone_handle = _handle.clone();
        let _clone_close = _close.clone();
        let _join = tokio::spawn(async move {
            loop {
                let _s_listen = _listener.accept().await;
                let (socket, _) = match _s_listen {
                    Err(e) => {
                        error!("TcpServer listener loop err:{}!", e);
                        continue;
                    },
                    Ok(_s) => _s
                };

                let _clone_h = _clone_handle.clone();
                let _clone_c = _clone_close.clone();
                tokio::spawn(async move {
                    let mut buf = vec![0; 1024];
                    let mut net_pack = NetPack::new();
                    let net_rsp:Arc<Mutex<Queue<Vec<u8>>>> = Arc::new(Mutex::new(Queue::new()));
            
                    let (mut rd, mut wr) = io::split(socket);
                    loop {
                        match rd.read(&mut buf).await {
                            Ok(0) => return,
                            Ok(n) => {
                                net_pack.input(&buf[..n]);
                                match net_pack.try_get_pack() {
                                    None => continue,
                                    Some(data) => {
                                        let _h = _clone_h.clone();
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
        
                        let _c_ref = _clone_c.as_ref().lock().unwrap();
                        if _c_ref.is_closed() {
                            break;
                        }
                    }
                });  

                let _c_ref = _clone_close.as_ref().lock().unwrap();
                if _c_ref.is_closed() {
                    break;
                }              
            }
        });

        Ok(TcpServer {
            join: _join
        })
    }
}