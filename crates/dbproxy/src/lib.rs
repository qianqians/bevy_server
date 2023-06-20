use std::sync::{Mutex, Arc};
use tcp::server::TcpServer;
use close_handle::CloseHandle;

use tracing::{trace, debug, info, warn, error};

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

    pub async fn close(self) {
        let mut _c_handle = self.close.as_ref().lock().unwrap();
        _c_handle.close();
        let _ = self.server.join.await;
    }

}