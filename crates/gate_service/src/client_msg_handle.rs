use std::sync::{Mutex, Arc};

use tracing::{trace, error};

use thrift::protocol::{TCompactInputProtocol, TSerializable};
use thrift::transport::{TBufferChannel};

use proto::gate::{
    GateClientService, 
    ClientCallHubRpc,
    ClientCallHubRsp, 
    ClientCallHubErr,
    ClientCallHubNtf,
    ClientCallGateHeartbeats
};

use proto::hub::{
    HubGateService,
    CallRpc,
    CallRsp,
	CallErr,
	CallNtf
};

use proto::client::{
    ClientService,
    GateCallHeartbeats
};

use tcp::tcp_socket::{TcpReader, TcpWriter};
use queue::Queue;

use crate::entity_manager::Entity;
use crate::gate_service::{DelayHubMsg, ConnManager, HubProxy, ClientProxy};

pub struct ClientEvent {
    proxy: Arc<Mutex<ClientProxy>>,
    ev: GateClientService
}

pub struct GateClientMsgHandle {
    queue: Queue<Box<ClientEvent>>
}

fn deserialize(data: Vec<u8>) -> Result<GateClientService, Box<dyn std::error::Error>> {
    trace!("deserialize begin!");
    let mut t = TBufferChannel::with_capacity(data.len(), 0);
    let _ = t.set_readable_bytes(&data);
    let mut i_prot = TCompactInputProtocol::new(t);
    let ev_data = GateClientService::read_from_in_protocol(&mut i_prot)?;
    Ok(ev_data)
}

impl GateClientMsgHandle {
    pub fn new() -> Arc<Mutex<GateClientMsgHandle>> {
        Arc::new(Mutex::new(GateClientMsgHandle {
            queue: Queue::new(), 
        }))
    }

    pub fn enque_event(&mut self, ev: ClientEvent) {
        self.queue.enque(Box::new(ev))
    }

    pub fn on_event(_proxy: Arc<Mutex<ClientProxy>>, _: Arc<Mutex<TcpWriter>>, data: Vec<u8>) {
        trace!("do_client_event begin!");

        let _proxy_cloen = _proxy.clone();
        let mut _p = _proxy.as_ref().lock().unwrap();
        let _ev = match deserialize(data) {
            Err(e) => {
                error!("GateClientMsgHandle do_event err:{}", e);
                return;
            }
            Ok(d) => d
        };
        let _handle_arc = _p.get_msg_handle();
        let mut _handle = _handle_arc.as_ref().lock().unwrap();
        _handle.enque_event(ClientEvent {
            proxy: _proxy_cloen,
            ev: _ev
        })
    }

    pub async fn do_call_hub_rpc(_proxy: Arc<Mutex<ClientProxy>>, ev: ClientCallHubRpc) {
        trace!("do_client_event call_hub_rpc begin!");
    
        let mut _p = _proxy.as_ref().lock().unwrap();
        let _conn_mgr_arc = _p.get_conn_mgr();
        let mut _conn_mgr = _conn_mgr_arc.as_ref().lock().unwrap();

        let event = ev.message.unwrap();
        let entity_id = ev.entity_id.unwrap();
        if let Some(_entity) = _conn_mgr.get_entity(&entity_id) {
            let mut _conn_mgr_tmp = _conn_mgr_arc.as_ref().lock().unwrap();
            if let Some(_hub_arc) = _conn_mgr_tmp.get_hub_proxy(_entity.get_hub_name()) {
                let mut _hub = _hub_arc.as_ref().lock().unwrap();
                if !_hub.send_hub_msg(HubGateService::CallRpc(CallRpc::new(entity_id, ev.msg_cb_id.unwrap(), event))).await {
                    let mut _conn_mgr_tmp = _conn_mgr_arc.as_ref().lock().unwrap();
                    _conn_mgr_tmp.delete_hub_proxy(_entity.get_hub_name());
                }
            }
        }
    }
    
    pub async fn do_call_hub_rsp(_proxy: Arc<Mutex<ClientProxy>>, ev: ClientCallHubRsp) {
        trace!("do_client_event call_hub_rsp begin!");
    
        let mut _p = _proxy.as_ref().lock().unwrap();
        let _conn_mgr_arc = _p.get_conn_mgr();
        let mut _conn_mgr = _conn_mgr_arc.as_ref().lock().unwrap();

        let event = ev.rsp.unwrap();
        let event_tmp_main = event.clone();
        let entity_id = event.entity_id.unwrap();
        if let Some(_entity) = _conn_mgr.get_entity(&entity_id) {
            let mut _conn_mgr_tmp = _conn_mgr_arc.as_ref().lock().unwrap();
            if let Some(_hub_arc) = _conn_mgr_tmp.get_hub_proxy(_entity.get_hub_name()) {
                let mut _hub = _hub_arc.as_ref().lock().unwrap();
                if !_hub.send_hub_msg(HubGateService::CallRsp(CallRsp::new(event_tmp_main))).await {
                    let mut _conn_mgr_tmp = _conn_mgr_arc.as_ref().lock().unwrap();
                    _conn_mgr_tmp.delete_hub_proxy(_entity.get_hub_name());
                }
            }
        }
    }
    
    pub async fn do_call_hub_err(_proxy: Arc<Mutex<ClientProxy>>, ev: ClientCallHubErr) {
        trace!("do_client_event call_hub_err begin!");
    
        let mut _p = _proxy.as_ref().lock().unwrap();
        let _conn_mgr_arc = _p.get_conn_mgr();
        let mut _conn_mgr = _conn_mgr_arc.as_ref().lock().unwrap();

        let event = ev.err.unwrap();
        let event_tmp_main = event.clone();
        let entity_id = event.entity_id.unwrap();
        if let Some(_entity) = _conn_mgr.get_entity(&entity_id) {
            let mut _conn_mgr_tmp = _conn_mgr_arc.as_ref().lock().unwrap();
            if let Some(_hub_arc) = _conn_mgr_tmp.get_hub_proxy(_entity.get_hub_name()) {
                let mut _hub = _hub_arc.as_ref().lock().unwrap();
                if !_hub.send_hub_msg(HubGateService::CallErr(CallErr::new(event_tmp_main))).await {
                    let mut _conn_mgr_tmp = _conn_mgr_arc.as_ref().lock().unwrap();
                    _conn_mgr_tmp.delete_hub_proxy(_entity.get_hub_name());
                }
            }
        }
    }

    pub async fn do_call_hub_ntf(_proxy: Arc<Mutex<ClientProxy>>, ev: ClientCallHubNtf) {
        trace!("do_client_event call_hub_ntf begin!");
    
        let mut _p = _proxy.as_ref().lock().unwrap();
        let _conn_mgr_arc = _p.get_conn_mgr();
        let mut _conn_mgr = _conn_mgr_arc.as_ref().lock().unwrap();

        let event = ev.message.unwrap();
        let entity_id = ev.entity_id.unwrap();
        if let Some(_entity) = _conn_mgr.get_entity(&entity_id) {
            let mut _conn_mgr_tmp = _conn_mgr_arc.as_ref().lock().unwrap();
            if let Some(_hub_arc) = _conn_mgr_tmp.get_hub_proxy(_entity.get_hub_name()) {
                let mut _hub = _hub_arc.as_ref().lock().unwrap();
                if !_hub.send_hub_msg(HubGateService::CallNtf(CallNtf::new(entity_id, event))).await {
                    let mut _conn_mgr_tmp = _conn_mgr_arc.as_ref().lock().unwrap();
                    _conn_mgr_tmp.delete_hub_proxy(_entity.get_hub_name());
                }
            }
        }
    }

    pub async fn do_call_gate_heartbeats(_proxy: Arc<Mutex<ClientProxy>>, ev: ClientCallGateHeartbeats) {
        trace!("do_client_event call_gate_heartbeats begin!");

        let mut _client = _proxy.as_ref().lock().unwrap();
        if !_client.send_client_msg(ClientService::Heartbeats(GateCallHeartbeats::new())).await {
            let mut _p = _proxy.as_ref().lock().unwrap();
            let _conn_mgr_arc = _p.get_conn_mgr();
            let mut _conn_mgr_tmp = _conn_mgr_arc.as_ref().lock().unwrap();
            let mut _client = _proxy.as_ref().lock().unwrap();
            _conn_mgr_tmp.delete_client_proxy(_client.get_conn_id());
        }
    }
    
}