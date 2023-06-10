use std::any::{Any};

use thrift::protocol::{TCompactInputProtocolFactory, TCompactOutputProtocolFactory};
use thrift::transport::{TFramedReadTransportFactory, TFramedWriteTransportFactory};
use thrift::server::TServer;

use proto::dbproxy::DbproxySyncHandler;
use proto::hub::HubDbproxyCallbackSyncClient;

use mongo::MongoProxy;
use processor::Processor;

mod db;

pub struct DBProxyThriftServer {
    proxy : MongoProxy,
    processor : Processor<Box<db::DBEvent>>
}

impl DBProxyThriftServer {
    pub fn new(_processor: Processor<Box<db::DBEvent>>, mongo_proxy:MongoProxy) -> DBProxyThriftServer {
        DBProxyThriftServer {
            proxy: mongo_proxy,
            processor: _processor
        }
    }

    fn cost_mut(&self) -> &mut Self {
        unsafe { &mut * (self as * const Self as * mut Self) }
    }
}

impl DbproxySyncHandler for DBProxyThriftServer {
    fn handle_reg_hub(&self, _: String) -> thrift::Result<bool> {
        Ok(true)
    }

    fn handle_get_guid(&self, db: String, collection: String, callback_id: String) -> thrift::Result<()> {
        let ev_data = db::DBEvGetGuid::new();
        let p = self.cost_mut();
        let ev = db::DBEvent::new(&mut p.proxy, db::DBEventType::EvGetGuid, db, collection, callback_id, Box::new(ev_data));
        p.processor.enque(Box::new(ev));
        Ok(())
    }

    fn handle_create_object(&self, db: String, collection: String, callback_id: String, object_info: Vec<u8>) -> thrift::Result<()> {
        let ev_data = db::DBEvCreateObject::new(object_info);
        let p = self.cost_mut();
        let ev = db::DBEvent::new(&mut p.proxy, db::DBEventType::EvGetGuid, db, collection, callback_id, Box::new(ev_data));
        p.processor.enque(Box::new(ev));
        Ok(())
    }

    fn handle_updata_object(&self, db: String, collection: String, callback_id: String, query_info: Vec<u8>, updata_info: Vec<u8>, _upsert: bool) -> thrift::Result<()> {
        let ev_data = db::DBEvUpdataObject::new(query_info, updata_info, _upsert);
        let p = self.cost_mut();
        let ev = db::DBEvent::new(&mut p.proxy, db::DBEventType::EvGetGuid, db, collection, callback_id, Box::new(ev_data));
        p.processor.enque(Box::new(ev));
        Ok(())
    }

    fn handle_find_and_modify(&self, db: String, collection: String, callback_id: String, query_info: Vec<u8>, updata_info: Vec<u8>, _new: bool, _upsert: bool) -> thrift::Result<()> {
        let ev_data = db::DBEvFindAndModify::new(query_info, updata_info, _new, _upsert);
        let p = self.cost_mut();
        let ev = db::DBEvent::new(&mut p.proxy, db::DBEventType::EvGetGuid, db, collection, callback_id, Box::new(ev_data));
        p.processor.enque(Box::new(ev));
        Ok(())
    }

    fn handle_remove_object(&self, db: String, collection: String, callback_id: String, query_info: Vec<u8>) -> thrift::Result<()> {
        let mut ev_data = db::DBEvRemoveObject::new(query_info);
        let p = self.cost_mut();
        let ev = db::DBEvent::new(&mut p.proxy, db::DBEventType::EvGetGuid, db, collection, callback_id, Box::new(ev_data));
        p.processor.enque(Box::new(ev));
        Ok(())
    }

    fn handle_get_object_info(&self, db: String, collection: String, callback_id: String, query_info: Vec<u8>, skip: i32, limit: i32, sort: String, ascending: bool) -> thrift::Result<()> {
        let ev_data = db::DBEvGetObjectInfo::new(query_info, skip, limit, sort, ascending);
        let p = self.cost_mut();
        let ev = db::DBEvent::new(&mut p.proxy, db::DBEventType::EvGetGuid, db, collection, callback_id, Box::new(ev_data));
        p.processor.enque(Box::new(ev));
        Ok(())
    }

    fn handle_get_object_count(&self, db: String, collection: String, callback_id: String, query_info: Vec<u8>) -> thrift::Result<()> {
        let ev_data = db::DBEvGetObjectCount::new(query_info);
        let p = self.cost_mut();
        let ev = db::DBEvent::new(&mut p.proxy, db::DBEventType::EvGetGuid, db, collection, callback_id, Box::new(ev_data));
        p.processor.enque(Box::new(ev));
        Ok(())
    }
}
