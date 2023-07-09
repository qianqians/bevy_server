use std::sync::{Mutex, Arc};
use std::thread;
use std::time::Duration;

use tracing::{info};

use net::NetReader;
use tcp::tcp_server::TcpServer;
use tcp::tcp_socket::{TcpReader, TcpWriter};
use close_handle::CloseHandle;
use mongo::MongoProxy;
use health::HealthHandle;
use timer::utc_unix_timer;

mod db;
mod handle;

use crate::handle::DBProxyHubMsgHandle;

pub struct DBProxyServer {
    handle: Arc<Mutex<DBProxyHubMsgHandle>>,
    health: Arc<Mutex<HealthHandle>>,
    close: Arc<Mutex<CloseHandle>>,
    server: TcpServer
}

impl DBProxyServer {
    pub async fn new(mongo_url:String, host:String, health_handle: Arc<Mutex<HealthHandle>>) -> Result<DBProxyServer, Box<dyn std::error::Error>> {
        let _mongo = MongoProxy::new(mongo_url).await?;
        let _handle = DBProxyHubMsgHandle::new(_mongo)?;
        let mut _close = Arc::new(Mutex::new(CloseHandle::new()));
        let mut _h = _handle.clone();
        let _c = _close.clone();
        let _tcp_s = TcpServer::listen(host, DBProxyServer::do_accept, _h, _c).await?;
        Ok(DBProxyServer {
            handle: _handle,
            health: health_handle,
            close: _close,
            server: _tcp_s
        })
    }

    fn do_accept(_h: Arc<Mutex<DBProxyHubMsgHandle>>, _c: Arc<Mutex<CloseHandle>>, rd: TcpReader, wr: TcpWriter) {
        let _ = rd.start(DBProxyHubMsgHandle::do_event, _h, Arc::new(Mutex::new(wr)), _c);
    }

    pub fn close(&self) {
        info!("start close!");

        let mut _c_handle = self.close.as_ref().lock().unwrap();
        _c_handle.close();
    }

    pub async fn join(self) {
        info!("await work done!");

        let _ = self.server.join().await;

        let mut _h = self.handle.as_ref().lock().unwrap();
        let _ = _h.poll().await;

        info!("work done!");
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
                let mut _health = self.health.as_ref().lock().unwrap();
                _health.set_health_status(true);
            }
            else if tick > 256 {
                let mut _health = self.health.as_ref().lock().unwrap();
                _health.set_health_status(false);
            }
        }
    }
}