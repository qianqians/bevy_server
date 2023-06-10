use thrift::protocol::{TCompactInputProtocolFactory, TCompactOutputProtocolFactory};
use thrift::transport::{TFramedReadTransportFactory, TFramedWriteTransportFactory};
use thrift::server::TServer;

use proto::dbproxy::DbproxySyncHandler;
use proto::hub::HubDbproxyCallbackSyncClient;

use mongo::MongoProxy;

mod process;

pub struct DBProxyServer {
    proxy : MongoProxy 
}

impl DBProxyServer {
    fn new(mongoProxy:MongoProxy) -> DBProxyServer {
        DBProxyServer {
            proxy: mongoProxy,
        }
    }
}

impl DbproxySyncHandler for DBProxyServer {
    fn handle_reg_hub(&self, hub_name: String) -> thrift::Result<bool> {
        Ok(true)
    }

    fn handle_get_guid(&self, db: String, collection: String) -> thrift::Result<i64> {
        Ok(self.proxy.get_guid(db, collection).await.into())
    }

    fn handle_create_object(&self, db: String, collection: String, object_info: Vec<u8>) -> thrift::Result<bool> {
        Ok(self.proxy.save(db, collection, object_info).await)
    }

    fn handle_updata_object(&self, db: String, collection: String, query_info: Vec<u8>, updata_info: Vec<u8>, _upsert: bool) -> thrift::Result<bool> {
        Ok(self.proxy.update(db, collection, query_info, updata_info, _upsert).await)
    }

    fn handle_find_and_modify(&self, db: String, collection: String, query_info: Vec<u8>, updata_info: Vec<u8>, _new: bool, _upsert: bool) -> thrift::Result<Vec<u8>> {
        let result = self.proxy.find_and_modify(db, collection, query_info, updata_info, _new, _upsert).await;
        let doc = match result {
            Ok(v) => v,
            Err(_e) => Ok(Vec![])
        };
        match doc {
            None => return Ok(Vec![]),
            Some(d) => Ok(d)
        }
    }
    
    fn handle_remove_object(&self, db: String, collection: String, query_info: Vec<u8>) -> thrift::Result<bool> {
        Ok(self.proxy.remove(db, collection, query_info).await)
    }

  //fn handle_get_object_info(&self, db: String, collection: String, query_info: Vec<u8>, skip: i32, limit: i32, sort: String, ascending: bool, cbid: String) -> thrift::Result<()>;
  //fn handle_get_object_count(&self, db: String, collection: String, query_info: Vec<u8>) -> thrift::Result<i32>;
}