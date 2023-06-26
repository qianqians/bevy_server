use std::sync::{Mutex, Arc};
use std::marker::{Send, Sync};
use std::fs::File;
use std::io::Read;

use tokio::task::JoinHandle;
use websocket::sync::{Server};
use websocket::native_tls::{Identity, TlsAcceptor};
use tracing::{trace, error};

use close_handle::CloseHandle;

use crate::wss_socket::{WSSReader, WSSWriter};

pub struct WSSServer{
    join: JoinHandle<()>
}

impl WSSServer {
    pub async fn listen<H: Send + Sync + 'static>(host:String, pfx:String, f:fn(_h: Arc<Mutex<H>>, _c: Arc<Mutex<CloseHandle>>, rd: WSSReader, wr: WSSWriter), _handle: Arc<Mutex<H>>, _close: Arc<Mutex<CloseHandle>>) -> Result<WSSServer, Box<dyn std::error::Error>> {
        let mut file = File::open(pfx).unwrap();
        let mut pkcs12 = vec![];
        file.read_to_end(&mut pkcs12).unwrap();
        let pkcs12 = Identity::from_pkcs12(&pkcs12, "hacktheplanet")?;
        let acceptor = TlsAcceptor::builder(pkcs12).build()?;
        let server = Server::bind_secure(host, acceptor)?;
        let _clone_handle = _handle.clone();
        let _clone_close = _close.clone();
        let _join = tokio::spawn(async move {
            for request in server.filter_map(Result::ok) {
                if !request.protocols().contains(&"websocket".to_string()) {
                    request.reject().unwrap();
                    error!("wss protocol wrong!");
                    return;
                }

                let mut _client = request.use_protocol("websocket").accept().unwrap();
                trace!("wss accept client ip:{}", _client.peer_addr().unwrap());

                let _client_arc = Arc::new(Mutex::new(_client));
                let _clone_client = _client_arc.clone();
                let _clone_h = _clone_handle.clone();
                let _clone_c = _clone_close.clone();
                f(_clone_h, _clone_c, WSSReader::new(_client_arc), WSSWriter::new(_clone_client));

                let _c_ref = _clone_close.as_ref().lock().unwrap();
                if _c_ref.is_closed() {
                    break;
                }              
            }
        });

        Ok(WSSServer {
            join: _join
        })
    }

    pub async fn join(self) {
        let _ = self.join.await;
    }

}