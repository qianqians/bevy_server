use std::sync::{Mutex, Arc};

use tracing::{trace, error};

use thrift::protocol::{TCompactInputProtocol, TSerializable};
use thrift::transport::{TBufferChannel};

use proto::gate::{
    GateHubService, 
    RegHub, 
    NtfTransferStart, 
    NtfTransferComplete, 
    CreateRemoteEntity, 
    HubCallClientRpc, 
    HubCallClientRsp, 
    HubCallClientErr, 
    HubCallClientNtf,
    HubCallClientGroup, 
    HubCallClientGlobal
};

use proto::client::{
    ClientService,
    CallRpc,
    CallRsp,
    CallErr,
    CallNtf
};

use tcp::tcp_socket::{TcpReader, TcpWriter};
use queue::Queue;

use crate::gate_service::{ConnManager, HubProxy};

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

    pub fn do_event(_proxy: Arc<Mutex<HubProxy>>, _: Arc<Mutex<TcpWriter>>, data: Vec<u8>) {
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
        _p.on_event(HubEvent {
            proxy: _proxy_cloen,
            ev: _ev
        })
    }

    pub fn do_reg_hub(_proxy: Arc<Mutex<HubProxy>>, ev: RegHub) {
        trace!("do_hub_event reg_hu begin!");

        let mut _p = _proxy.as_ref().lock().unwrap();
        let name = ev.hub_name.unwrap();
        let _type = ev.hub_type.unwrap();
        _p.set_hub_info(name, _type)
    }

    pub fn do_ntf_transfer_start(_proxy: Arc<Mutex<HubProxy>>, ev: NtfTransferStart) {

    }
}