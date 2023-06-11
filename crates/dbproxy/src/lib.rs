use std::any::Any;
use std::collections::HashMap;
use std::sync::Mutex;
use std::ptr::null;
use std::thread;
use std::time::Duration;

use thrift::protocol::{TCompactInputProtocolFactory, TCompactOutputProtocolFactory, TCompactInputProtocol, TCompactOutputProtocol, TInputProtocol, TOutputProtocol};
use thrift::transport::{TFramedReadTransportFactory, TFramedWriteTransportFactory, TFramedReadTransport, TFramedWriteTransport, ReadHalf, WriteHalf, TIoChannel, TTcpChannel };
use thrift::server::{TServer, TProcessor};

use proto::dbproxy::{DbproxySyncHandler, DbproxySyncProcessor};
use proto::hub::HubDbproxyCallbackSyncClient;

use mongo::MongoProxy;
use processor::Processor;
use timer::utc_unix_timer;

mod db;

type HubInputProtocol = TCompactInputProtocol<TFramedReadTransport<ReadHalf<TTcpChannel>>>;
type HubOutputProtocol = TCompactOutputProtocol<TFramedWriteTransport<WriteHalf<TTcpChannel>>>;

pub struct DBProxyThriftServer {
    proxy: MongoProxy,
    processor: Processor<Box<db::DBEvent>>,
    hubs: Mutex<HashMap<String, *mut HubDbproxyCallbackSyncClient<HubInputProtocol, HubOutputProtocol>>>,
}

impl DBProxyThriftServer {
    pub fn new(_processor: Processor<Box<db::DBEvent>>, mongo_proxy:MongoProxy) -> DBProxyThriftServer {
        DBProxyThriftServer {
            proxy: mongo_proxy,
            processor: _processor,
            hubs: Mutex::new(HashMap::new()),
        }
    }

    fn cast_mut(&self) -> &mut Self {
        unsafe { &mut * (self as * const Self as * mut Self) }
    }

    fn run(&mut self) {
        let begin = utc_unix_timer();
        self.processor.process(|ev_data| {
            let mut mut_ev_data = ev_data;
            mut_ev_data.do_event();
        });
        let tick = utc_unix_timer() - begin;

        if tick < 33 {
            thread::sleep(Duration::from_millis((33 - tick) as u64));
        }
    }
}

impl DbproxySyncHandler for DBProxyThriftServer {
    fn handle_reg_hub(&self, hub_name: String, host: String, port: i32) -> thrift::Result<bool> {
        let mut c = TTcpChannel::new();
        c.open(&format!("{}:{}", host, port))?;
        let (i_chan, o_chan) = c.split()?;

        let i_tran = TFramedReadTransport::new(i_chan);
        let o_tran = TFramedWriteTransport::new(o_chan);
    
        let i_prot = TCompactInputProtocol::new(i_tran);
        let o_prot = TCompactOutputProtocol::new(o_tran);

        let p = self.cast_mut();
        let mut hubs = p.hubs.lock().unwrap();
        hubs.insert(hub_name, &mut (HubDbproxyCallbackSyncClient::new(i_prot, o_prot)));

        Ok(true)
    }

    fn handle_get_guid(&self, db: String, collection: String, hub_name: String, callback_id: String) -> thrift::Result<()> {
        let p = self.cast_mut();
        let hubs = p.hubs.lock().unwrap();
        let opt_hub_proxy= hubs.get(&hub_name);
        if let Some(p_hub) = opt_hub_proxy {
            let ev_data = db::DBEvGetGuid::new();
            let ev = db::DBEvent::new(*p_hub, &mut p.proxy, db::DBEventType::EvGetGuid, db, collection, callback_id, Box::new(ev_data));
            p.processor.enque(Box::new(ev));
        };
        Ok(())
    }

    fn handle_create_object(&self, db: String, collection: String, hub_name: String, callback_id: String, object_info: Vec<u8>) -> thrift::Result<()> {
        let p = self.cast_mut();
        let hubs = p.hubs.lock().unwrap();
        let opt_hub_proxy= hubs.get(&hub_name);
        if let Some(p_hub) = opt_hub_proxy {
            let ev_data = db::DBEvCreateObject::new(object_info);
            let ev = db::DBEvent::new(*p_hub, &mut p.proxy, db::DBEventType::EvGetGuid, db, collection, callback_id, Box::new(ev_data));
            p.processor.enque(Box::new(ev));
        };
        Ok(())
    }

    fn handle_updata_object(&self, db: String, collection: String, hub_name: String, callback_id: String, query_info: Vec<u8>, updata_info: Vec<u8>, _upsert: bool) -> thrift::Result<()> {
        let p = self.cast_mut();
        let hubs = p.hubs.lock().unwrap();
        let opt_hub_proxy= hubs.get(&hub_name);
        if let Some(p_hub) = opt_hub_proxy {
            let ev_data = db::DBEvUpdataObject::new(query_info, updata_info, _upsert);
            let ev = db::DBEvent::new(*p_hub, &mut p.proxy, db::DBEventType::EvGetGuid, db, collection, callback_id, Box::new(ev_data));
            p.processor.enque(Box::new(ev));
        };
        Ok(())
    }

    fn handle_find_and_modify(&self, db: String, collection: String, hub_name: String, callback_id: String, query_info: Vec<u8>, updata_info: Vec<u8>, _new: bool, _upsert: bool) -> thrift::Result<()> {
        let p = self.cast_mut();
        let hubs = p.hubs.lock().unwrap();
        let opt_hub_proxy= hubs.get(&hub_name);
        if let Some(p_hub) = opt_hub_proxy {
            let ev_data = db::DBEvFindAndModify::new(query_info, updata_info, _new, _upsert);
            let ev = db::DBEvent::new(*p_hub, &mut p.proxy, db::DBEventType::EvGetGuid, db, collection, callback_id, Box::new(ev_data));
            p.processor.enque(Box::new(ev));
        };
        Ok(())
    }

    fn handle_remove_object(&self, db: String, collection: String, hub_name: String, callback_id: String, query_info: Vec<u8>) -> thrift::Result<()> {
        let p = self.cast_mut();
        let hubs = p.hubs.lock().unwrap();
        let opt_hub_proxy= hubs.get(&hub_name);
        if let Some(p_hub) = opt_hub_proxy {
            let ev_data = db::DBEvRemoveObject::new(query_info);
            let ev = db::DBEvent::new(*p_hub, &mut p.proxy, db::DBEventType::EvGetGuid, db, collection, callback_id, Box::new(ev_data));
            p.processor.enque(Box::new(ev));
        }
        Ok(())
    }

    fn handle_get_object_info(&self, db: String, collection: String, hub_name: String, callback_id: String, query_info: Vec<u8>, skip: i32, limit: i32, sort: String, ascending: bool) -> thrift::Result<()> {
        let p = self.cast_mut();
        let hubs = p.hubs.lock().unwrap();
        let opt_hub_proxy= hubs.get(&hub_name);
        if let Some(p_hub) = opt_hub_proxy {
            let ev_data = db::DBEvGetObjectInfo::new(query_info, skip, limit, sort, ascending);
            let ev = db::DBEvent::new(*p_hub, &mut p.proxy, db::DBEventType::EvGetGuid, db, collection, callback_id, Box::new(ev_data));
            p.processor.enque(Box::new(ev));
        };
        Ok(())
    }

    fn handle_get_object_count(&self, db: String, collection: String, hub_name: String, callback_id: String, query_info: Vec<u8>) -> thrift::Result<()> {
        let p = self.cast_mut();
        let hubs = p.hubs.lock().unwrap();
        let opt_hub_proxy= hubs.get(&hub_name);
        if let Some(p_hub) = opt_hub_proxy {
            let ev_data = db::DBEvGetObjectCount::new(query_info);
            let ev = db::DBEvent::new(*p_hub, &mut p.proxy, db::DBEventType::EvGetGuid, db, collection, callback_id, Box::new(ev_data));
            p.processor.enque(Box::new(ev));
        }
        Ok(())
    }
}
