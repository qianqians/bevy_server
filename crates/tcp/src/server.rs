use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::task::JoinHandle;

use tracing::{trace, debug, info, warn, error};

use net_pack::{NetPack, NetResponse};

struct TcpServer {
    listener: TcpListener
}

impl TcpServer {
    pub async fn new(host:String) -> Result<TcpServer, Box<dyn std::error::Error>> {
        let _listener = TcpListener::bind(host).await?;
        Ok(TcpServer {
            listener: _listener
        })
    }

    pub async fn run(&mut self, f:fn(rsp:&mut NetResponse, data:Vec<u8>)) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            let (mut socket, _) = self.listener.accept().await?;

            tokio::spawn(async move {
                let mut buf = vec![0; 1024];
                let mut net_pack = NetPack::new();
                let mut net_rsp = NetResponse::new();
    
                loop {
                    match socket.read(&mut buf).await {
                        Ok(0) => return,
                        Ok(n) => {
                            net_pack.input(&buf[..n]);
                            match net_pack.try_get_pack() {
                                None => continue,
                                Some(data) => {
                                    f(&mut net_rsp, data);
                                }
                            }
                        }
                        Err(err) => {
                            error!("network err:{}!", err);
                            return;
                        }
                    }
                }
            });
        }
    }
}