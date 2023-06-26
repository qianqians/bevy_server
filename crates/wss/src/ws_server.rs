use std::sync::{Mutex, Arc};
use std::marker::{Send, Sync};

use tokio::task::JoinHandle;
use websocket::sync::{Server};
use tracing::{trace, error};

use close_handle::CloseHandle;

use crate::ws_socket::{WSReader, WSWriter};

pub struct WSServer{
    join: JoinHandle<()>
}

impl WSServer {
    pub async fn listen<H: Send + Sync + 'static>(host:String, f:fn(_h: Arc<Mutex<H>>, _c: Arc<Mutex<CloseHandle>>, rd: WSReader, wr: WSWriter), _handle: Arc<Mutex<H>>, _close: Arc<Mutex<CloseHandle>>) -> Result<WSServer, Box<dyn std::error::Error>> {
        let server = Server::bind(host)?;
        let _join = tokio::spawn(async move {
            for request in server.filter_map(Result::ok) {
                if !request.protocols().contains(&"websocket".to_string()) {
                    request.reject().unwrap();
                    error!("ws protocol wrong!");
                    return;
                }

                let mut _client = request.use_protocol("websocket").accept().unwrap();
                trace!("ws accept client ip:{}", _client.peer_addr().unwrap());

                let ip = _client.peer_addr().unwrap();
                let (rd, wr) = _client.split().unwrap();
                let mut _wr_arc = Arc::new(Mutex::new(wr));
                let _wr_clone = _wr_arc.clone();

                let _clone_h = _handle.clone();
                let _clone_c = _close.clone();
                f(_clone_h, _clone_c, WSReader::new(ip, rd, _wr_arc), WSWriter::new(_wr_clone));

                let _clone_close = _close.clone();
                let _c_ref = _clone_close.as_ref().lock().unwrap();
                if _c_ref.is_closed() {
                    break;
                }              
            }
        });

        Ok(WSServer {
            join: _join
        })
    }

    pub async fn join(self) {
        let _ = self.join.await;
    }

}