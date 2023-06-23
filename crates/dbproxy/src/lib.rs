use std::sync::{Mutex, Arc};
use std::thread;
use std::time::Duration;

use tracing::{trace, debug, info, warn, error};

use tcp::tcp_server::TcpServer;
use close_handle::CloseHandle;
use mongo::MongoProxy;
use timer::utc_unix_timer;

mod db;
mod handle;

pub struct DBProxyServer {
    handle: Arc<Mutex<handle::DBProxyHubMsgHandle>>,
    close: Arc<Mutex<CloseHandle>>,
    server: TcpServer
}

impl DBProxyServer {
    pub async fn new(mongo_url:String, host:String) -> Result<DBProxyServer, Box<dyn std::error::Error>> {
        let _mongo = MongoProxy::new(mongo_url).await?;
        let _handle = handle::DBProxyHubMsgHandle::new(_mongo).await?;
        let mut _close = Arc::new(Mutex::new(CloseHandle::new()));
        let mut _s = _handle.clone();
        let _c = _close.clone();
        let _tcp_s = TcpServer::listen(host, handle::DBProxyHubMsgHandle::do_event, _s, _c).await?;
        Ok(DBProxyServer {
            handle: _handle,
            close: _close,
            server: _tcp_s
        })
    }

    pub fn close(&self) {
        let mut _c_handle = self.close.as_ref().lock().unwrap();
        _c_handle.close();
    }

    pub async fn join(self) {
        let _ = self.server.join().await;
    }

    pub async fn run(&mut self) {
        loop {
            let begin = utc_unix_timer();
            
            let mut _h = self.handle.as_ref().lock().unwrap();
            let _ = _h.poll().await;
        
            let tick = utc_unix_timer() - begin;

            let _c_ref = self.close.as_ref().lock().unwrap();
            if _c_ref.is_closed() {
                break;
            }

            if tick < 33 {
                thread::sleep(Duration::from_millis((33 - tick) as u64));
            }
        }
    }
}