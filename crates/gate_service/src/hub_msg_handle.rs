use std::sync::{Mutex, Arc};

use tracing::{trace, error};

use thrift::protocol::{TCompactInputProtocol, TSerializable};
use thrift::transport::{TBufferChannel};

use proto::gate::{
    GateHubService, 
    RegHub,
    CreateRemoteEntity, 
    HubCallClientRpc, 
    HubCallClientRsp, 
    HubCallClientErr, 
    HubCallClientNtf,
    HubCallClientGroup, 
    HubCallClientGlobal,
    HubCallKickOffClient
};

use proto::client::{
    ClientService,
    KickOff,
    CallRpc,
    CallRsp,
    CallErr,
    CallNtf
};

use proto::hub::{
    HubGateService,
    NtfTransferMsgEnd
};

use tcp::tcp_socket::{TcpReader, TcpWriter};
use queue::Queue;

use crate::entity_manager::Entity;
use crate::gate_service::{DelayHubMsg, ConnManager, HubProxy};

pub struct HubEvent {
    proxy: Arc<Mutex<HubProxy>>,
    ev: GateHubService
}

pub struct GateHubMsgHandle {
    queue: Queue<Box<HubEvent>>
}

fn deserialize(data: Vec<u8>) -> Result<GateHubService, Box<dyn std::error::Error>> {
    trace!("deserialize begin!");
    let mut t = TBufferChannel::with_capacity(data.len(), 0);
    let _ = t.set_readable_bytes(&data);
    let mut i_prot = TCompactInputProtocol::new(t);
    let ev_data = GateHubService::read_from_in_protocol(&mut i_prot)?;
    Ok(ev_data)
}

impl GateHubMsgHandle {
    pub fn new() -> Result<Arc<Mutex<GateHubMsgHandle>>, Box<dyn std::error::Error>> {
        let mut _hub_server = Arc::new(Mutex::new(GateHubMsgHandle {
            queue: Queue::new(), 
        }));
        Ok(_hub_server)
    }

    pub fn enque_event(&mut self, ev: HubEvent) {
        self.queue.enque(Box::new(ev))
    }

    pub fn on_event(_proxy: Arc<Mutex<HubProxy>>, _: Arc<Mutex<TcpWriter>>, data: Vec<u8>) {
        trace!("do_hub_event begin!");

        let _proxy_cloen = _proxy.clone();
        let mut _p = _proxy.as_ref().lock().unwrap();
        let _ev = match deserialize(data) {
            Err(e) => {
                error!("DBProxyThriftServer do_event err:{}", e);
                return;
            }
            Ok(d) => d
        };
        let _handle_arc = _p.get_msg_handle();
        let mut _handle = _handle_arc.as_ref().lock().unwrap();
        _handle.enque_event(HubEvent {
            proxy: _proxy_cloen,
            ev: _ev
        })
    }

    pub async fn do_reg_hub(_proxy: Arc<Mutex<HubProxy>>, ev: RegHub) {
        trace!("do_hub_event reg_hu begin!");

        let name = ev.hub_name.unwrap();
        let _type = ev.hub_type.unwrap();
        HubProxy::set_hub_info(_proxy, name, _type)
    }

    pub async fn do_create_remote_entity(_proxy: Arc<Mutex<HubProxy>>, ev: CreateRemoteEntity) {
        trace!("do_hub_event create_remote_entity begin!");

        let _proxy_clone = _proxy.clone();
        let mut _p = _proxy.as_ref().lock().unwrap();
        let _conn_mgr_arc = _p.get_conn_mgr();
        let mut _conn_mgr = _conn_mgr_arc.as_ref().lock().unwrap();
        let entity_id = ev.entity_id.unwrap();
        let entity_id_clone = entity_id.clone();
        let _entity = match _conn_mgr.get_entity(entity_id) {
            None => {
                let entity_id_clone_tmp = entity_id_clone.clone();
                let e = Entity::new(entity_id_clone, _p.get_hub_name().to_string());
                _conn_mgr.update_entity(e);
                _conn_mgr.get_entity(entity_id_clone_tmp).unwrap()
            }
            Some(e) => e
        };
        if let Some(main_conn_id) = ev.main_conn_id {
            if let Some(_client_arc) = _conn_mgr.get_client_proxy(main_conn_id) {
                let mut _client = _client_arc.as_ref().lock().unwrap();
                let _client_wr_arc = _client.get_writer();
                let _client_wr = _client_wr_arc.as_ref().lock().unwrap();
                
                //_entity.set_main_conn_id(Some(main_conn_id))
            }
        }
        
    }
}