use std::any::{Any};

use thrift::protocol::{TCompactInputProtocol, TCompactOutputProtocol};
use thrift::transport::{TFramedReadTransport, TFramedWriteTransport, ReadHalf, WriteHalf, TTcpChannel };

use proto::hub::{HubDbproxyCallbackSyncClient, THubDbproxyCallbackSyncClient};

use mongodb::bson::{doc, Document};

use mongo::MongoProxy;

type HubInputProtocol = TCompactInputProtocol<TFramedReadTransport<ReadHalf<TTcpChannel>>>;
type HubOutputProtocol = TCompactOutputProtocol<TFramedWriteTransport<WriteHalf<TTcpChannel>>>;

pub enum DBEventType {
    EvGetGuid,
    EvCreateObject,
    EvUpdataObject,
    EvFindAndModify,
    EvRemoveObject,
    EvGetObjectInfo,
    EvGetObjectCount
}

pub struct DBEvGetGuid {
}

impl DBEvGetGuid {
    pub fn new() -> DBEvGetGuid {
        DBEvGetGuid {}
    }
}

pub struct DBEvCreateObject {
    pub object_info: Vec<u8>
}

impl DBEvCreateObject {
    pub fn new(_object_info:Vec<u8>) -> DBEvCreateObject {
        DBEvCreateObject {
            object_info: _object_info
        }
    }
}

pub struct DBEvUpdataObject {
    pub query_info: Vec<u8>,
    pub updata_info: Vec<u8>,
    pub upsert: bool
}

impl DBEvUpdataObject {
    pub fn new(_query_info: Vec<u8>, _updata_info: Vec<u8>, _upsert: bool) -> DBEvUpdataObject {
        DBEvUpdataObject {
            query_info: _query_info,
            updata_info: _updata_info,
            upsert: _upsert
        }
    }
}

pub struct DBEvFindAndModify {
    pub query_info: Vec<u8>, 
    pub updata_info: Vec<u8>, 
    pub _new: bool, 
    pub upsert: bool
}

impl DBEvFindAndModify {
    pub fn new(_query_info: Vec<u8>, _updata_info: Vec<u8>, _new_: bool, _upsert: bool) -> DBEvFindAndModify {
        DBEvFindAndModify {
            query_info: _query_info,
            updata_info: _updata_info,
            _new: _new_,
            upsert: _upsert
        }
    }
}

pub struct DBEvRemoveObject {
    pub query_info: Vec<u8>
}

impl DBEvRemoveObject {
    pub fn new(_query_info:Vec<u8>) -> DBEvRemoveObject {
        DBEvRemoveObject {
            query_info: _query_info
        }
    }
}

pub struct DBEvGetObjectInfo {
    pub query_info: Vec<u8>, 
    pub skip: u32, 
    pub limit: u32, 
    pub sort: String, 
    pub ascending: bool
}

impl DBEvGetObjectInfo {
    pub fn new(_query_info:Vec<u8>, _skip: i32, _limit: i32, _sort: String, _ascending: bool) -> DBEvGetObjectInfo {
        DBEvGetObjectInfo {
            query_info: _query_info,
            skip: _skip as u32,
            limit: _limit as u32,
            sort: _sort,
            ascending: _ascending
        }
    }
}

pub struct DBEvGetObjectCount {
    pub query_info: Vec<u8>
}

impl DBEvGetObjectCount {
    pub fn new(_query_info:Vec<u8>) -> DBEvGetObjectCount {
        DBEvGetObjectCount {
            query_info: _query_info
        }
    }
}

pub struct DBEvent {
    pub hub_proxy: *mut HubDbproxyCallbackSyncClient<HubInputProtocol, HubOutputProtocol>,
    pub proxy: *mut MongoProxy,
    pub ev_type: DBEventType,
    pub db: String,
    pub collection: String,
    pub callback_id: String,
    pub ev_data: Box<dyn Any>
}

impl DBEvent {
    pub fn new(_hub_proxy: *mut HubDbproxyCallbackSyncClient<HubInputProtocol, HubOutputProtocol>, _proxy: &mut MongoProxy, _ev_type: DBEventType, _db: String, _collection: String,  _callback_id: String, _ev_data: Box<dyn Any>) -> DBEvent {
        DBEvent {
            hub_proxy: _hub_proxy,
            proxy: _proxy,
            ev_type: _ev_type,
            db: _db,
            collection: _collection,
            callback_id: _callback_id,
            ev_data: _ev_data
        }
    }

    async fn do_get_guid(&mut self) {
        unsafe {
            if let Some(p_mongo) = self.proxy.as_mut() {
                let guid = p_mongo.get_guid(self.db.to_string(), self.collection.to_string()).await;
                if let Some(p_hub) = self.hub_proxy.as_mut() {
                    let _ = p_hub.ack_get_guid(self.callback_id.to_string(), guid);
                }
            }
        }
    }

    async fn do_create_object(&mut self) {
        unsafe {
            if let Some(p_mongo) = self.proxy.as_mut() {
                let p_ev_data = self.ev_data.downcast_ref::<DBEvCreateObject>();
                if let Some(ev_data) = p_ev_data {
                    let result = p_mongo.save(self.db.to_string(), self.collection.to_string(), &ev_data.object_info).await;
                    if let Some(p_hub) = self.hub_proxy.as_mut() {
                        let _ = p_hub.ack_create_object(self.callback_id.to_string(), result);
                    }
                }
            }
        }
    }

    async fn do_updata_object(&mut self) {
        unsafe {
            if let Some(p_mongo) = self.proxy.as_mut() {
                let p_ev_data = self.ev_data.downcast_ref::<DBEvUpdataObject>();
                if let Some(ev_data) = p_ev_data {
                    let result = p_mongo.update(self.db.to_string(), self.collection.to_string(), &ev_data.query_info, &ev_data.updata_info, ev_data.upsert).await;
                    if let Some(p_hub) = self.hub_proxy.as_mut() {
                        let _ = p_hub.ack_updata_object(self.callback_id.to_string(), result);
                    }
                }
            }
        }
    }

    async fn do_find_and_modify(&mut self) {
        unsafe {
            if let Some(p_mongo) = self.proxy.as_mut() {
                let p_ev_data = self.ev_data.downcast_ref::<DBEvFindAndModify>();
                if let Some(ev_data) = p_ev_data {
                    let result = p_mongo.find_and_modify(self.db.to_string(), self.collection.to_string(), &ev_data.query_info, &ev_data.updata_info, ev_data._new, ev_data.upsert).await;
                    let opt_doc = match result {
                        Err(e) => return,
                        Ok(v) => v
                    };
                    if let Some(doc) = opt_doc {
                        let mut bin: Vec<u8> = Vec::new();
                        let _ = doc.to_writer(&mut bin);
                        if let Some(p_hub) = self.hub_proxy.as_mut() {
                            let _ = p_hub.ack_find_and_modify(self.callback_id.to_string(), bin);
                        }
                    }
                }
            }
        }
    }

    async fn do_remove_object(&mut self) {
        unsafe {
            if let Some(p_mongo) = self.proxy.as_mut() {
                let p_ev_data = self.ev_data.downcast_ref::<DBEvRemoveObject>();
                if let Some(ev_data) = p_ev_data {
                    let result = p_mongo.remove(self.db.to_string(), self.collection.to_string(), &ev_data.query_info).await;
                    if let Some(p_hub) = self.hub_proxy.as_mut() {
                        let _ = p_hub.ack_remove_object(self.callback_id.to_string(), result);
                    }
                }
            }
        }
    }

    async fn do_get_object_info(&mut self) {
        unsafe {
            if let Some(p_mongo) = self.proxy.as_mut() {
                let p_ev_data = self.ev_data.downcast_ref::<DBEvGetObjectInfo>();
                if let Some(ev_data) = p_ev_data {
                    let result = p_mongo.find(self.db.to_string(), self.collection.to_string(), &ev_data.query_info, ev_data.skip, ev_data.limit, ev_data.sort.to_string(), ev_data.ascending).await;
                    let docs = match result {
                        Err(e) => return,
                        Ok(v) => v
                    };
                    let doc = doc!{"__list__":docs};
                    let mut bin: Vec<u8> = Vec::new();
                    let _ = doc.to_writer(&mut bin);
                    if let Some(p_hub) = self.hub_proxy.as_mut() {
                        let _ = p_hub.ack_get_object_info(self.callback_id.to_string(), bin);
                    }
                }
            }
        }
    }

    async fn do_get_object_count(&mut self) {
        unsafe {
            if let Some(p_mongo) = self.proxy.as_mut() {
                let p_ev_data = self.ev_data.downcast_ref::<DBEvGetObjectCount>();
                if let Some(ev_data) = p_ev_data {
                    let count = p_mongo.count(self.db.to_string(), self.collection.to_string(), &ev_data.query_info).await;
                    if let Some(p_hub) = self.hub_proxy.as_mut() {
                        let _ = p_hub.ack_get_object_count(self.callback_id.to_string(), count);
                    }
                }
            }
        }
    }

    pub async fn do_event(&mut self) {
        match self.ev_type {
            DBEventType::EvGetGuid => self.do_get_guid().await,
            DBEventType::EvCreateObject => self.do_create_object().await,
            DBEventType::EvUpdataObject => self.do_updata_object().await,
            DBEventType::EvFindAndModify => self.do_find_and_modify().await,
            DBEventType::EvRemoveObject => self.do_remove_object().await,
            DBEventType::EvGetObjectInfo => self.do_get_object_info().await,
            DBEventType::EvGetObjectCount => self.do_get_object_count().await
        }
    }
}