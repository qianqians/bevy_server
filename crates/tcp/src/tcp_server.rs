use std::sync::{Mutex, Arc};
use std::marker::{Send, Sync};

use tokio::io::{self};
use tokio::net::TcpListener;
use tokio::task::JoinHandle;

use tracing::{error};

use close_handle::CloseHandle;

use crate::tcp_socket::{TcpReader, TcpWriter};

pub struct TcpServer{
    join: JoinHandle<()>
}

impl TcpServer {
    pub async fn listen<H: Send + Sync + 'static>(host:String, f:fn(_h: Arc<Mutex<H>>, _c: Arc<Mutex<CloseHandle>>, rd: TcpReader, wr: TcpWriter), _handle: Arc<Mutex<H>>, _close: Arc<Mutex<CloseHandle>>) -> Result<TcpServer, Box<dyn std::error::Error>> {
        let _listener = TcpListener::bind(host).await?;
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

                let _clone_h = _handle.clone();
                let _clone_c = _clone_close.clone();
                let (rd, wr) = io::split(socket);
                f(_clone_h, _clone_c, TcpReader::new(rd), TcpWriter::new(wr));

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

    pub async fn join(self) {
        let _ = self.join.await;
    }

}