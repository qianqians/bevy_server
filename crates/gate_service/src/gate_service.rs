use std::sync::{Mutex, Arc};
use std::collections::BTreeMap;
use tokio::task::JoinHandle;

use tracing::{trace, error};

use thrift::protocol::{TCompactOutputProtocol, TSerializable};
use thrift::transport::{TIoChannel, TBufferChannel};

use queue::Queue;
use net::{NetReader, NetWriter};
use tcp::tcp_server::TcpServer;
use tcp::tcp_socket::{TcpReader, TcpWriter};
use close_handle::CloseHandle;

use proto::hub::{
    HubGateService
};

use proto::client::{
    ClientService,
};

use crate::hub_msg_handle::{GateHubMsgHandle};
use crate::client_msg_handle::{GateClientMsgHandle};
use crate::entity_manager::{Entity, EntityManager};

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

    pub fn set_hub_info(p: Arc<Mutex<HubProxy>>, name: String, _type: String) {
        let _p_clone = p.clone();
        let _name_clone = name.clone();
        let mut _p = p.as_ref().clone().lock().unwrap();

        _p.name = name;
        _p.hub_type = _type;

        let mut _conn_mgr = _p.conn_mgr.as_ref().lock().unwrap();
        _conn_mgr.hubs.insert(_name_clone, _p_clone);
    }

    pub fn get_hub_name(&self) -> &String {
        &self.name
    }

    pub fn get_msg_handle(&mut self) -> Arc<Mutex<GateHubMsgHandle>> {
        let _conn_mgr = self.conn_mgr.as_ref().lock().unwrap();
        _conn_mgr.hub_msg_handle.clone()
    }

    pub fn get_conn_mgr(&mut self) -> Arc<Mutex<ConnManager>> {
        self.conn_mgr.clone()
    }

    pub async fn send_hub_msg(&mut self, msg: HubGateService) -> bool {
        let t = TBufferChannel::with_capacity(0, 1024);
        let (rd, wr) = match t.split() {
            Ok(_t) => (_t.0, _t.1),
            Err(_e) => {
                error!("do_get_guid t.split error {}", _e);
                return false;
            }
        };
        let mut o_prot = TCompactOutputProtocol::new(wr);
        let _ = HubGateService::write_to_out_protocol(&msg, &mut o_prot);
        let mut p_send = self.wr.as_ref().lock().unwrap();
        p_send.send(&rd.write_bytes()).await
    }
}

pub struct DelayHubMsg {
    hubproxy: Arc<Mutex<HubProxy>>,
    ev: HubGateService
}

impl DelayHubMsg {
    pub fn new(proxy: Arc<Mutex<HubProxy>>, _ev: HubGateService) -> DelayHubMsg {
        DelayHubMsg {
            hubproxy: proxy,
            ev: _ev
        }
    }
}

pub struct ClientProxy {
    conn_id: String,
    wr: Arc<Mutex<Box<dyn NetWriter + Send + Sync + 'static>>>,
    join: JoinHandle<()>,
    conn_mgr: Arc<Mutex<ConnManager>>
}

impl ClientProxy {
    pub fn get_conn_id(&self) -> &String {
        &self.conn_id
    }

    pub fn get_writer(&mut self) -> Arc<Mutex<Box<dyn NetWriter + Send + Sync + 'static>>> {
        self.wr.clone()
    }

    pub fn get_msg_handle(&mut self) -> Arc<Mutex<GateClientMsgHandle>> {
        let _conn_mgr = self.conn_mgr.as_ref().lock().unwrap();
        _conn_mgr.client_msg_handle.clone()
    }

    pub fn get_conn_mgr(&mut self) -> Arc<Mutex<ConnManager>> {
        self.conn_mgr.clone()
    }

    pub async fn send_client_msg(&mut self, msg: ClientService) -> bool {
        let t = TBufferChannel::with_capacity(0, 1024);
        let (rd, wr) = match t.split() {
            Ok(_t) => (_t.0, _t.1),
            Err(_e) => {
                error!("do_get_guid t.split error {}", _e);
                return false;
            }
        };
        let mut o_prot = TCompactOutputProtocol::new(wr);
        let _ = ClientService::write_to_out_protocol(&msg, &mut o_prot);
        let mut p_send = self.wr.as_ref().lock().unwrap();
        p_send.send(&rd.write_bytes()).await
    }
}

pub struct ConnManager {
    hubs: BTreeMap<String, Arc<Mutex<HubProxy>>>,
    clients: BTreeMap<String, Arc<Mutex<ClientProxy>>>,
    entities: EntityManager,
    delay_hub_msg: Queue<DelayHubMsg>,
    hub_msg_handle: Arc<Mutex<GateHubMsgHandle>>,
    client_msg_handle: Arc<Mutex<GateClientMsgHandle>>
}

impl ConnManager {
    pub fn new(_hub_handle: Arc<Mutex<GateHubMsgHandle>>, _client_handle: Arc<Mutex<GateClientMsgHandle>>) -> ConnManager {
        ConnManager {
            hubs: BTreeMap::new(),
            clients: BTreeMap::new(),
            entities: EntityManager::new(),
            delay_hub_msg: Queue::new(),
            hub_msg_handle: _hub_handle,
            client_msg_handle: _client_handle
        }
    }

    pub fn update_entity(&mut self, e: Entity) {
        self.entities.update_entity(e)
    }

    pub fn get_entity(&mut self, entity_id: &String) -> Option<&mut Entity> {
        self.entities.get_entity(entity_id)
    }

    pub fn delete_entity(&mut self, entity_id: &String) -> Option<Entity> {
        self.entities.delete_entity(entity_id)
    }

    pub fn get_client_proxy(&mut self, conn_id: &String) -> Option<&Arc<Mutex<ClientProxy>>> {
        self.clients.get(conn_id)
    }

    pub fn delete_client_proxy(&mut self, conn_id: &String) {
        let _ = self.clients.remove(conn_id);
    }

    pub fn get_all_client_proxy(&mut self) -> Vec<Arc<Mutex<ClientProxy>>> {
        let _client_clone = self.clients.clone();
        _client_clone.into_values().collect()
    }

    pub fn get_hub_proxy(&mut self, name: &String) -> Option<&Arc<Mutex<HubProxy>>> {
        self.hubs.get(name)
    }

    pub fn delete_hub_proxy(&mut self, name: &String) {
        let _ = self.hubs.remove(name);
    }

    pub async fn close_client(&mut self, conn_id: &String) {
        if let Some(client) = self.clients.remove(conn_id) {
            let _c = client.as_ref().lock().unwrap();
            let mut _wr = _c.wr.as_ref().lock().unwrap();
            _wr.close().await;
            _c.join.abort()
        }
    }

    pub fn add_delay_hub_msg(&mut self, ev: DelayHubMsg) {
        self.delay_hub_msg.enque(ev)
    }
}

pub struct GateService {
    server: TcpServer
}

impl GateService {
    pub async fn new(hub_host:String, client_tcp_host:String, client_wss_host:String, c: Arc<Mutex<CloseHandle>>) -> Result<GateService, Box<dyn std::error::Error>> {
        let _hub_handle = GateHubMsgHandle::new();
        let _client_handle = GateClientMsgHandle::new();
        let _conn_mgr = Arc::new(Mutex::new(ConnManager::new(_hub_handle, _client_handle)));
        let _tcp_s = TcpServer::listen(hub_host, GateService::do_accept_hub, _conn_mgr, c).await?;
        Ok(GateService {
            server: _tcp_s
        })
    }

    pub fn do_accept_hub(_h: Arc<Mutex<ConnManager>>, _c: Arc<Mutex<CloseHandle>>, rd: TcpReader, wr: TcpWriter) {
        let _wr_arc = Arc::new(Mutex::new(wr));
        let _wr_arc_clone = _wr_arc.clone();
        let _hubproxy = Arc::new(Mutex::new(HubProxy::new(_wr_arc, _h)));
        let _ = rd.start(GateHubMsgHandle::on_event, _hubproxy, _wr_arc_clone, _c);
    }
}