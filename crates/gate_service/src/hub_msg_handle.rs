use std::sync::{Mutex, Arc};

use tracing::{trace, error};

use thrift::protocol::{TCompactInputProtocol, TSerializable};
use thrift::transport::{TBufferChannel};

use proto::gate::{GateHubService, RegHub, NtfTransferStart, NtfTransferComplete, CreateRemoteEntity, HubCallClientRpc, HubCallClientRsp, HubCallClientErr, HubCallClientGroup, HubCallClientGlobal};

use queue::Queue;

use crate::gate_service::ConnManager;

pub struct GateHubMsgHandle {
    conn_mgr: Arc<Mutex<ConnManager>>,
    queue: Queue<Box<GateHubService>>
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
    pub fn new(_conn_mgr: Arc<Mutex<ConnManager>>) -> Result<Arc<Mutex<GateHubMsgHandle>>, Box<dyn std::error::Error>> {
        let mut _hub_server = Arc::new(Mutex::new(GateHubMsgHandle {
            conn_mgr: _conn_mgr,
            queue: Queue::new(), 
        }));
        Ok(_hub_server)
    }
}