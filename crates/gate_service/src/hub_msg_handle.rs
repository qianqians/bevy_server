use std::sync::{Mutex, Arc};

use tracing::{trace, error};

use thrift::protocol::{TCompactInputProtocol, TSerializable};
use thrift::transport::{TBufferChannel};

use proto::gate::{
    GateHubService, 
    RegHub,
    HubCallClientCreateRemoteEntity, 
    HubCallClientDeleteRemoteEntity,
    HubCallClientRpc, 
    HubCallClientRsp, 
    HubCallClientErr, 
    HubCallClientNtf,
    HubCallClientGroup, 
    HubCallClientGlobal,
    HubCallKickOffClient,
    HubCallTransferClientComplete
};

use proto::client::{
    ClientService,
    CreateRemoteEntity,
    DeleteRemoteEntity,
    TransferComplete,
    KickOff,
    CallRpc,
    CallRsp,
    CallErr,
    CallNtf,
    CallGlobal
};

use proto::hub::{
    HubGateService,
    NtfTransferMsgEnd
};

use tcp::tcp_socket::{TcpReader, TcpWriter};
use queue::Queue;

use crate::entity_manager::Entity;
use crate::gate_service::{DelayHubMsg, ConnManager, HubProxy, ClientProxy};

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
    pub fn new() -> Arc<Mutex<GateHubMsgHandle>> {
        Arc::new(Mutex::new(GateHubMsgHandle {
            queue: Queue::new(), 
        }))
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
                error!("GateHubMsgHandle do_event err:{}", e);
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

    pub async fn poll(&mut self) {
        loop {
            let opt_ev_data = self.queue.deque();
            let mut_ev_data = match opt_ev_data {
                None => break,
                Some(ev_data) => ev_data
            };
            let proxy = mut_ev_data.proxy.clone();
            match mut_ev_data.ev {
                GateHubService::RegHub(ev) => GateHubMsgHandle::do_reg_hub(proxy, ev).await,
                GateHubService::CreateRemoteEntity(ev) => GateHubMsgHandle::do_create_remote_entity(proxy, ev).await,
                GateHubService::DeleteRemoteEntity(ev) => GateHubMsgHandle::do_delete_remote_entity(proxy, ev).await,
                GateHubService::CallRpc(ev) => GateHubMsgHandle::do_call_client_rpc(proxy, ev).await,
                GateHubService::CallRsp(ev) => GateHubMsgHandle::do_call_client_rsp(proxy, ev).await,
                GateHubService::CallErr(ev) => GateHubMsgHandle::do_call_client_err(proxy, ev).await,
                GateHubService::CallNtf(ev) => GateHubMsgHandle::do_call_client_ntf(proxy, ev).await,
                GateHubService::CallGroup(ev) => GateHubMsgHandle::do_call_client_group(proxy, ev).await,
                GateHubService::CallGlobal(ev) => GateHubMsgHandle::do_call_client_global(proxy, ev).await,
                GateHubService::KickOff(ev) => GateHubMsgHandle::do_kick_off_client(proxy, ev).await,
                GateHubService::TransferComplete(ev) => GateHubMsgHandle::do_transfer_client_complete(proxy, ev).await,
            }
        }
    }

    pub async fn do_reg_hub(_proxy: Arc<Mutex<HubProxy>>, ev: RegHub) {
        trace!("do_hub_event reg_hu begin!");

        let name = ev.hub_name.unwrap();
        let _type = ev.hub_type.unwrap();
        HubProxy::set_hub_info(_proxy, name, _type)
    }

    pub async fn do_create_remote_entity(_proxy: Arc<Mutex<HubProxy>>, ev: HubCallClientCreateRemoteEntity) {
        trace!("do_hub_event create_remote_entity begin!");

        let _proxy_clone = _proxy.clone();
        let mut _p = _proxy.as_ref().lock().unwrap();
        let _conn_mgr_arc = _p.get_conn_mgr();
        let mut _conn_mgr = _conn_mgr_arc.as_ref().lock().unwrap();
        let entity_id = ev.entity_id.unwrap();
        let entity_id_clone = entity_id.clone();
        let entity_id_tmp_main = entity_id.clone();
        let entity_id_tmp_other = entity_id.clone();
        let _entity = match _conn_mgr.get_entity(&entity_id) {
            None => {
                let entity_id_clone_tmp = entity_id_clone.clone();
                let e = Entity::new(entity_id_clone, _p.get_hub_name().to_string());
                _conn_mgr.update_entity(e);
                _conn_mgr.get_entity(&entity_id_clone_tmp).unwrap()
            }
            Some(e) => e
        };
        let entity_type = ev.entity_type.unwrap();
        let entity_type_other = entity_type.clone();
        let argvs = ev.argvs.unwrap();
        let argvs_other = argvs.clone();
        if let Some(main_conn_id) = ev.main_conn_id {
            let mut _conn_mgr_tmp = _conn_mgr_arc.as_ref().lock().unwrap();
            let main_conn_id_tmp = main_conn_id.clone();
            if let Some(_client_arc) = _conn_mgr_tmp.get_client_proxy(&main_conn_id) {
                let entity_type_tmp = entity_type.clone();
                let argvs_tmp = argvs.clone();
                let mut _client = _client_arc.as_ref().lock().unwrap();
                if !_client.send_client_msg(ClientService::CreateRemoteEntity(CreateRemoteEntity::new(entity_id_tmp_main, entity_type_tmp, true, argvs_tmp))).await {
                    let mut _conn_mgr_tmp = _conn_mgr_arc.as_ref().lock().unwrap();
                    _conn_mgr_tmp.delete_client_proxy(&main_conn_id);
                }
                else {
                    _entity.set_main_conn_id(Some(main_conn_id_tmp));
                }
            }
        }
        if let Some(conn_dis) = ev.conn_id {
            for id in conn_dis.iter() {
                let mut _conn_mgr_tmp = _conn_mgr_arc.as_ref().lock().unwrap();
                if let Some(_client_arc) = _conn_mgr_tmp.get_client_proxy(id) {
                    let _entity_id_tmp = entity_id_tmp_other.clone();
                    let entity_type_tmp = entity_type_other.clone();
                    let argvs_tmp = argvs_other.clone();
                    let mut _client = _client_arc.as_ref().lock().unwrap();
                    if !_client.send_client_msg(ClientService::CreateRemoteEntity(CreateRemoteEntity::new(_entity_id_tmp, entity_type_tmp, false, argvs_tmp))).await {
                        let mut _conn_mgr_tmp = _conn_mgr_arc.as_ref().lock().unwrap();
                        _conn_mgr_tmp.delete_client_proxy(id);
                    }
                    else {
                        _entity.add_conn_id(id.to_string());
                    }
                }
            }
        }
    }

    pub async fn do_delete_remote_entity(_proxy: Arc<Mutex<HubProxy>>, ev: HubCallClientDeleteRemoteEntity) {
        trace!("do_hub_event delete_remote_entity begin!");

        let mut _p = _proxy.as_ref().lock().unwrap();
        let _conn_mgr_arc = _p.get_conn_mgr();
        let mut _conn_mgr = _conn_mgr_arc.as_ref().lock().unwrap();

        let entity_id = ev.entity_id.unwrap();
        let entity_id_tmp_main = entity_id.clone();
        let entity_id_tmp_other = entity_id.clone();
        if let Some(_entity) = _conn_mgr.delete_entity(&entity_id) {
            if let Some(main_conn_id) = _entity.get_main_conn_id() {
                let mut _conn_mgr_tmp = _conn_mgr_arc.as_ref().lock().unwrap();
                if let Some(_client_arc) = _conn_mgr_tmp.get_client_proxy(&main_conn_id) {
                    let mut _client = _client_arc.as_ref().lock().unwrap();
                    if !_client.send_client_msg(ClientService::DeleteRemoteEntity(DeleteRemoteEntity::new(entity_id_tmp_main))).await {
                        let mut _conn_mgr_tmp = _conn_mgr_arc.as_ref().lock().unwrap();
                        _conn_mgr_tmp.delete_client_proxy(&main_conn_id);
                    }
                }
            }
            for id in _entity.get_conn_ids().iter() {
                let mut _conn_mgr_tmp = _conn_mgr_arc.as_ref().lock().unwrap();
                if let Some(_client_arc) = _conn_mgr_tmp.get_client_proxy(id) {
                    let _entity_id_tmp = entity_id_tmp_other.clone();
                    let mut _client = _client_arc.as_ref().lock().unwrap();
                    if !_client.send_client_msg(ClientService::DeleteRemoteEntity(DeleteRemoteEntity::new(_entity_id_tmp))).await {
                        let mut _conn_mgr_tmp = _conn_mgr_arc.as_ref().lock().unwrap();
                        _conn_mgr_tmp.delete_client_proxy(id);
                    }
                }
            }
        }
    }

    pub async fn do_call_client_rpc(_proxy: Arc<Mutex<HubProxy>>, ev: HubCallClientRpc) {
        trace!("do_hub_event call_client_rpc begin!");

        let mut _p = _proxy.as_ref().lock().unwrap();
        let _conn_mgr_arc = _p.get_conn_mgr();
        let mut _conn_mgr = _conn_mgr_arc.as_ref().lock().unwrap();

        let event = ev.message.unwrap();
        let entity_id = ev.entity_id.unwrap();
        let msg_cb_id = ev.msg_cb_id.unwrap();
        if let Some(_entity) = _conn_mgr.get_entity(&entity_id) {
            if let Some(main_conn_id) = _entity.get_main_conn_id() {
                let mut _conn_mgr_tmp = _conn_mgr_arc.as_ref().lock().unwrap();
                if let Some(_client_arc) = _conn_mgr_tmp.get_client_proxy(&main_conn_id) {
                    let mut _client = _client_arc.as_ref().lock().unwrap();
                    if !_client.send_client_msg(ClientService::CallRpc(CallRpc::new(entity_id, msg_cb_id, event))).await {
                        _entity.set_main_conn_id(None);
                        _conn_mgr.delete_client_proxy(&main_conn_id);
                    }
                }
            }
        }
    }

    pub async fn do_call_client_rsp(_proxy: Arc<Mutex<HubProxy>>, ev: HubCallClientRsp) {
        trace!("do_hub_event call_client_rpc begin!");

        let mut _p = _proxy.as_ref().lock().unwrap();
        let _conn_mgr_arc = _p.get_conn_mgr();
        let mut _conn_mgr = _conn_mgr_arc.as_ref().lock().unwrap();

        let event = ev.rsp.unwrap();
        let event_tmp_main = event.clone();
        let entity_id = event.entity_id.unwrap();
        if let Some(_entity) = _conn_mgr.get_entity(&entity_id) {
            if let Some(main_conn_id) = _entity.get_main_conn_id() {
                let mut _conn_mgr_tmp = _conn_mgr_arc.as_ref().lock().unwrap();
                if let Some(_client_arc) = _conn_mgr_tmp.get_client_proxy(&main_conn_id) {
                    let mut _client = _client_arc.as_ref().lock().unwrap();
                    if !_client.send_client_msg(ClientService::CallRsp(CallRsp::new(event_tmp_main))).await {
                        _entity.set_main_conn_id(None);
                        let mut _conn_mgr_tmp = _conn_mgr_arc.as_ref().lock().unwrap();
                        _conn_mgr_tmp.delete_client_proxy(&main_conn_id);
                    }
                }
            }
        }
    }

    pub async fn do_call_client_err(_proxy: Arc<Mutex<HubProxy>>, ev: HubCallClientErr) {
        trace!("do_hub_event call_client_rpc begin!");

        let mut _p = _proxy.as_ref().lock().unwrap();
        let _conn_mgr_arc = _p.get_conn_mgr();
        let mut _conn_mgr = _conn_mgr_arc.as_ref().lock().unwrap();

        let event = ev.err.unwrap();
        let event_tmp_main = event.clone();
        let entity_id = event.entity_id.unwrap();
        if let Some(_entity) = _conn_mgr.get_entity(&entity_id) {
            if let Some(main_conn_id) = _entity.get_main_conn_id() {
                let mut _conn_mgr_tmp = _conn_mgr_arc.as_ref().lock().unwrap();
                if let Some(_client_arc) = _conn_mgr_tmp.get_client_proxy(&main_conn_id) {
                    let mut _client = _client_arc.as_ref().lock().unwrap();
                    if !_client.send_client_msg(ClientService::CallErr(CallErr::new(event_tmp_main))).await {
                        _entity.set_main_conn_id(None);
                        let mut _conn_mgr_tmp = _conn_mgr_arc.as_ref().lock().unwrap();
                        _conn_mgr_tmp.delete_client_proxy(&main_conn_id);
                    }
                }
            }
        }
    }

    pub async fn do_call_client_ntf(_proxy: Arc<Mutex<HubProxy>>, ev: HubCallClientNtf) {
        trace!("do_hub_event call_client_ntf begin!");

        let mut _p = _proxy.as_ref().lock().unwrap();
        let _conn_mgr_arc = _p.get_conn_mgr();
        let mut _conn_mgr = _conn_mgr_arc.as_ref().lock().unwrap();

        let entity_id = ev.entity_id.unwrap();
        let entity_id_clone = entity_id.clone();
        let event = ev.message.unwrap();
        let event_tmp_main = event.clone();
        let event_tmp_other = event.clone();
        if let Some(_entity) = _conn_mgr.get_entity(&entity_id) {
            let mut invalid_ids: Vec<String> = vec![];
            if let Some(main_conn_id) = _entity.get_main_conn_id() {
                let mut _conn_mgr_tmp = _conn_mgr_arc.as_ref().lock().unwrap();
                if let Some(_client_arc) = _conn_mgr_tmp.get_client_proxy(&main_conn_id) {
                    let mut _client = _client_arc.as_ref().lock().unwrap();
                    if !_client.send_client_msg(ClientService::CallNtf(CallNtf::new(entity_id, event_tmp_main))).await {
                        _entity.set_main_conn_id(None);
                        let mut _conn_mgr_tmp = _conn_mgr_arc.as_ref().lock().unwrap();
                        _conn_mgr_tmp.delete_client_proxy(&main_conn_id);
                    }
                }
            }
            for id in _entity.get_conn_ids().iter() {
                let mut _conn_mgr_tmp = _conn_mgr_arc.as_ref().lock().unwrap();
                if let Some(_client_arc) = _conn_mgr_tmp.get_client_proxy(id) {
                    let mut _client = _client_arc.as_ref().lock().unwrap();
                    let _event_tmp_other = event_tmp_other.clone();
                    let _entity_id_tmp = entity_id_clone.clone();
                    if !_client.send_client_msg(ClientService::CallNtf(CallNtf::new(_entity_id_tmp, _event_tmp_other))).await {
                        invalid_ids.push(id.to_string());
                        let mut _conn_mgr_tmp = _conn_mgr_arc.as_ref().lock().unwrap();
                        _conn_mgr_tmp.delete_client_proxy(id);
                    }
                }
                else {
                    invalid_ids.push(id.to_string());
                }
            }
            for invalid_id in invalid_ids.iter() {
                _entity.delete_conn_id(invalid_id);
            }
        }
    }

    pub async fn do_call_client_group(_proxy: Arc<Mutex<HubProxy>>, ev: HubCallClientGroup) {
        trace!("do_hub_event call_client_group begin!");

        let mut _p = _proxy.as_ref().lock().unwrap();
        let _conn_mgr_arc = _p.get_conn_mgr();
        let mut _conn_mgr = _conn_mgr_arc.as_ref().lock().unwrap();

        let event = ev.message.unwrap();
        let event_tmp_main = event.clone();
        let event_tmp_other = event.clone();
        let entity_ids = ev.entity_id.unwrap();
        for id in entity_ids.iter() {
            if let Some(_entity) = _conn_mgr.get_entity(&id) {
                let mut invalid_ids: Vec<String> = vec![];
                if let Some(main_conn_id) = _entity.get_main_conn_id() {
                    let mut _conn_mgr_tmp = _conn_mgr_arc.as_ref().lock().unwrap();
                    if let Some(_client_arc) = _conn_mgr_tmp.get_client_proxy(&main_conn_id) {
                        let mut _client = _client_arc.as_ref().lock().unwrap();
                        let _event_tmp_main = event_tmp_main.clone();
                        if !_client.send_client_msg(ClientService::CallNtf(CallNtf::new(id.to_string(), _event_tmp_main))).await {
                            _entity.set_main_conn_id(None);
                            let mut _conn_mgr_tmp = _conn_mgr_arc.as_ref().lock().unwrap();
                            _conn_mgr_tmp.delete_client_proxy(&main_conn_id);
                        }
                    }
                }
                for id in _entity.get_conn_ids().iter() {
                    let mut _conn_mgr_tmp = _conn_mgr_arc.as_ref().lock().unwrap();
                    if let Some(_client_arc) = _conn_mgr_tmp.get_client_proxy(id) {
                        let mut _client = _client_arc.as_ref().lock().unwrap();
                        let _event_tmp_other = event_tmp_other.clone();
                        let _entity_id_tmp = id.clone();
                        if !_client.send_client_msg(ClientService::CallNtf(CallNtf::new(_entity_id_tmp, _event_tmp_other))).await {
                            invalid_ids.push(id.to_string());
                            let mut _conn_mgr_tmp = _conn_mgr_arc.as_ref().lock().unwrap();
                            _conn_mgr_tmp.delete_client_proxy(id);
                        }
                    }
                    else {
                        invalid_ids.push(id.to_string());
                    }
                }
                for invalid_id in invalid_ids.iter() {
                    _entity.delete_conn_id(invalid_id);
                }
            }
        }
    }
    
    pub async fn do_call_client_global(_proxy: Arc<Mutex<HubProxy>>, ev: HubCallClientGlobal) {
        trace!("do_hub_event call_client_global begin!");

        let mut _p = _proxy.as_ref().lock().unwrap();
        let _conn_mgr_arc = _p.get_conn_mgr();
        let mut _conn_mgr = _conn_mgr_arc.as_ref().lock().unwrap();
        
        let event = ev.message.unwrap();
        let _clients = _conn_mgr.get_all_client_proxy();
        for _client_arc in _clients.iter() {
            let mut _client = _client_arc.as_ref().lock().unwrap();
            let _event_tmp = event.clone();
            if !_client.send_client_msg(ClientService::CallGlobal(CallGlobal::new(_event_tmp))).await {
                _conn_mgr.delete_client_proxy(&_client.get_conn_id())
            }
        }
    }

    pub async fn do_kick_off_client(_proxy: Arc<Mutex<HubProxy>>, ev: HubCallKickOffClient) {
        trace!("do_hub_event kick_off_client begin!");

        let _proxy_clone = _proxy.clone();
        let mut _p = _proxy.as_ref().lock().unwrap();
        let _conn_mgr_arc = _p.get_conn_mgr();
        let mut _conn_mgr = _conn_mgr_arc.as_ref().lock().unwrap();

        let conn_id = ev.conn_id.unwrap();
        if let Some(_client_arc) = _conn_mgr.get_client_proxy(&conn_id) {
            let mut _client = _client_arc.as_ref().lock().unwrap();
            let _ = _client.send_client_msg(ClientService::KickOff(KickOff::new(ev.prompt_info.unwrap()))).await;
        }
        _conn_mgr.close_client(&conn_id).await;
        _conn_mgr.add_delay_hub_msg(DelayHubMsg::new(_proxy_clone, HubGateService::TransferMsgEnd(NtfTransferMsgEnd::new(conn_id))));
    }
    
    pub async fn do_transfer_client_complete(_proxy: Arc<Mutex<HubProxy>>, ev: HubCallTransferClientComplete) {
        trace!("do_hub_event transfer_client_complete begin!");

        let mut _p = _proxy.as_ref().lock().unwrap();
        let _conn_mgr_arc = _p.get_conn_mgr();
        let mut _conn_mgr = _conn_mgr_arc.as_ref().lock().unwrap();

        if let Some(_client_arc) = _conn_mgr.get_client_proxy(&ev.conn_id.unwrap()) {
            let mut _client = _client_arc.as_ref().lock().unwrap();
            _client.send_client_msg(ClientService::TransferComplete(TransferComplete::new()));
        }
    }
    
}