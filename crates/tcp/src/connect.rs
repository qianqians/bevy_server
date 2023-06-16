use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::task::JoinHandle;

use tracing::{trace, debug, info, warn, error};

use net_pack::{NetPack, NetResponse};

struct TcpConnect {
    join: JoinHandle<()>
}

impl TcpConnect {
    pub async fn new(host:String, f:fn(rsp:&mut NetResponse, data:Vec<u8>)) -> Result<TcpConnect, Box<dyn std::error::Error>> {
        let mut _socket = TcpStream::connect(host).await?;

        let _join = tokio::spawn(async move {
            let mut buf = vec![0; 1024];
            let mut net_pack = NetPack::new();
            let mut net_rsp = NetResponse::new();

            loop {
                match _socket.read(&mut buf).await {
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

        Ok(TcpConnect {
            join: _join
        })
    }
}
