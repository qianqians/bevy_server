use std::sync::{Mutex, Arc};
use std::collections::BTreeMap;

use net::NetWriter;
use tcp::tcp_server::TcpServer;
use tcp::tcp_socket::{TcpReader, TcpWriter};
use close_handle::CloseHandle;

use crate::hub_msg_handle;

pub struct HubProxy {
    name: String,
    wr: Arc<Mutex<TcpWriter>>
}

pub struct ClientProxy {
    conn_id: String,
    wr: Arc<Mutex<Box<dyn NetWriter + Send + Sync + 'static>>>
}

pub struct ConnManager {
    hubs: BTreeMap<String, Arc<Mutex<HubProxy>>>,
    clients: BTreeMap<String, Arc<Mutex<ClientProxy>>>,
}

impl ConnManager {
    pub fn new() -> ConnManager {
        ConnManager {
            hubs: BTreeMap::new(),
            clients: BTreeMap::new()
        }
    }
}

pub struct GateService {
    hub_mgr: Arc<Mutex<ConnManager>>,
    server: TcpServer
}

impl GateService {
    pub async fn new(hub_host:String, client_tcp_host:String, client_wss_host:String, c: Arc<Mutex<CloseHandle>>) -> Result<GateService, Box<dyn std::error::Error>> {
        let _conn_mgr = Arc::new(Mutex::new(ConnManager::new()));
        let _conn_mgr_clone = _conn_mgr.clone();
        let _tcp_s = TcpServer::listen(hub_host, GateService::do_accept_hub, _conn_mgr_clone, c).await?;
        Ok(GateService {
            hub_mgr: _conn_mgr,
            server: _tcp_s
        })
    }

    pub fn do_accept_hub(_h: Arc<Mutex<ConnManager>>, _c: Arc<Mutex<CloseHandle>>, rd: TcpReader, wr: TcpWriter) {

    }
}