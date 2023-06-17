use std::sync::Arc;
use std::marker::{Send, Sync};

use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

use tracing::{trace, debug, info, warn, error};

use net_pack::NetPack;
use queue::Queue;

pub struct TcpServer{
    listener: TcpListener
}

impl TcpServer {
    pub async fn new(host:String) -> Result<TcpServer, Box<dyn std::error::Error>> {
        let _listener = TcpListener::bind(host).await?;
        Ok(TcpServer {
            listener: _listener
        })
    }

    pub async fn run<H: Send + Sync + 'static>(& mut self, f:fn(_handle: Arc<H>, rsp:&mut Queue<Vec<u8>>, data:Vec<u8>), _handle: Arc<H>) -> Result<(), Box<dyn std::error::Error>> {
        let (socket, _) = self.listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];
            let mut net_pack = NetPack::new();
            let mut net_rsp:Queue<Vec<u8>> = Queue::new();
    
            let (mut rd, mut wr) = io::split(socket);
            loop {
                match rd.read(&mut buf).await {
                    Ok(0) => return,
                    Ok(n) => {
                        net_pack.input(&buf[..n]);
                        match net_pack.try_get_pack() {
                            None => continue,
                            Some(data) => {
                                let _h = _handle.clone();
                                f(_h, &mut net_rsp, data);
                            }
                        }
                    }
                    Err(err) => {
                        error!("network err:{}!", err);
                        return;
                    }
                }

                loop {
                    let opt_send_data = net_rsp.deque();
                    match opt_send_data {
                        None => break,
                        Some(send_data) => {
                            let _ = wr.write_all(&send_data).await;
                        }
                    }
                }
            }
        });
        Ok(())
    }
}