use std::sync::{Mutex, Arc};
use std::collections::BTreeMap;

use net::{NetReader, NetWriter};
use tcp::tcp_server::TcpServer;
use tcp::tcp_socket::{TcpReader, TcpWriter};
use close_handle::CloseHandle;

use crate::hub_msg_handle::{HubEvent, GateHubMsgHandle};

pub struct HubProxy {
    name: String,
    hub_type: String,
    wr: Arc<Mutex<TcpWriter>>,
    conn_mgr: Arc<Mutex<ConnManager>>
}

impl HubProxy {
    pub fn new(_wr: Arc<Mutex<TcpWriter>>, _conn_mgr: Arc<Mutex<ConnManager>>) -> HubProxy {
        HubProxy {
            name: "tmp_hub_name".to_string(),
            hub_type: "tmp_hub_type".to_string(),
            wr: _wr,
            conn_mgr: _conn_mgr
        }
    }

    pub fn set_hub_info(&mut self, name: String, _type: String) {
        self.name = name;
        self.hub_type = _type
    }

    pub fn on_event(&mut self, ev: HubEvent) {
        let _conn_mgr = self.conn_mgr.as_ref().lock().unwrap();
        let mut _msg_handle = _conn_mgr.handle.as_ref().lock().unwrap();
        _msg_handle.enque_event(ev)
    }
}

pub struct ClientProxy {
    conn_id: String,
    wr: Arc<Mutex<Box<dyn NetWriter + Send + Sync + 'static>>>,
    conn_mgr: Arc<Mutex<ConnManager>>
}

pub struct ConnManager {
    hubs: BTreeMap<String, Arc<Mutex<HubProxy>>>,
    clients: BTreeMap<String, Arc<Mutex<ClientProxy>>>,
    handle: Arc<Mutex<GateHubMsgHandle>>
}

impl ConnManager {
    pub fn new(_handle: Arc<Mutex<GateHubMsgHandle>>) -> ConnManager {
        ConnManager {
            hubs: BTreeMap::new(),
            clients: BTreeMap::new(),
            handle: _handle
        }
    }
}

pub struct GateService {
    server: TcpServer
}

impl GateService {
    pub async fn new(hub_host:String, client_tcp_host:String, client_wss_host:String, c: Arc<Mutex<CloseHandle>>) -> Result<GateService, Box<dyn std::error::Error>> {
        let _msg_handle = GateHubMsgHandle::new()?;
        let _conn_mgr = Arc::new(Mutex::new(ConnManager::new(_msg_handle)));
        let _tcp_s = TcpServer::listen(hub_host, GateService::do_accept_hub, _conn_mgr, c).await?;
        Ok(GateService {
            server: _tcp_s
        })
    }

    pub fn do_accept_hub(_h: Arc<Mutex<ConnManager>>, _c: Arc<Mutex<CloseHandle>>, rd: TcpReader, wr: TcpWriter) {
        let _wr_arc = Arc::new(Mutex::new(wr));
        let _wr_arc_clone = _wr_arc.clone();
        let _hubproxy = Arc::new(Mutex::new(HubProxy::new(_wr_arc, _h)));
        rd.start(GateHubMsgHandle::do_event, _hubproxy, _wr_arc_clone, _c)
    }
}