use std::sync::{Mutex, Arc};
use std::thread;
use std::time::Duration;

use tracing::{trace, debug, info, warn, error};

use thrift::protocol::{TCompactInputProtocol, TSerializable};
use thrift::transport::{TBufferChannel};

use proto::dbproxy::{DbEvent, GetGuidEvent, CreateObjectEvent, UpdateObjectEvent, FindAndModifyEvent, RemoveObjectEvent, GetObjectInfoEvent, GetObjectCountEvent};

use mongo::MongoProxy;
use queue::Queue;
use tcp::server::TcpServer;
use close_handle::CloseHandle;
use timer::utc_unix_timer;

mod db;

pub struct DBProxyThriftServer {
    proxy: MongoProxy,
    queue: Queue<Box<db::DBEvent>>,
    close: Arc<Mutex<CloseHandle>>
}

fn deserialize(data: Vec<u8>) -> Result<DbEvent, Box<dyn std::error::Error>> {
    let mut t = TBufferChannel::with_capacity(data.len(), 0);
    let _ = t.set_readable_bytes(&data);
    let mut i_prot = TCompactInputProtocol::new(t);
    let ev_data = DbEvent::read_from_in_protocol(&mut i_prot)?;
    Ok(ev_data)
}

impl DBProxyThriftServer {
    pub async fn new(host:String, _queue: Queue<Box<db::DBEvent>>, mongo_proxy:MongoProxy) -> Result<Arc<Mutex<DBProxyThriftServer>>, Box<dyn std::error::Error>> {
        let mut _tcp_s = TcpServer::new(host).await?;
        let mut _close = Arc::new(Mutex::new(CloseHandle::new()));
        let _c = _close.clone();
        let mut _db_server = Arc::new(Mutex::new(DBProxyThriftServer {
            proxy: mongo_proxy,
            queue: _queue,
            close: _close
        }));
        let mut _s = _db_server.clone();
        tokio::spawn( async move {
            loop {
                let mut _s_handle = _s.clone();
                let mut _c_handle = _c.clone();
                let _ = _tcp_s.run(DBProxyThriftServer::do_event, _s_handle, _c_handle).await;

                let _c_ref = _c.as_ref().lock().unwrap();
                if _c_ref.is_closed() {
                    break;
                }
            }
        });
        Ok(_db_server)
    }

    fn do_get_guid(&mut self, _data: GetGuidEvent, rsp: Arc<Mutex<Queue<Vec<u8>>>>) {
        let ev_data = db::DBEvGetGuid::new();
        let db = match _data.db {
            None => {
                error!("DBProxyThriftServer do_event GetGuid db is None!");
                return;
            },
            Some(_db) => _db
        };
        let collection = match _data.collection {
            None => {
                error!("DBProxyThriftServer do_event GetGuid collection is None!");
                return;
            },
            Some(_collection) => _collection
        };
        let callback_id = match _data.callback_id {
            None => {
                error!("DBProxyThriftServer do_event GetGuid callback_id is None!");
                return;
            },
            Some(_callback_id) => _callback_id
        };
        let ev = db::DBEvent::new(rsp,  db::DBEventType::EvGetGuid, db, collection, callback_id, Box::new(ev_data));
        self.queue.enque(Box::new(ev));
    }

    fn do_create_object(&mut self, _data: CreateObjectEvent, rsp: Arc<Mutex<Queue<Vec<u8>>>>) {
        let object_info = match _data.object_info {
            None => {
                error!("DBProxyThriftServer do_event CreateObjectEvent object_info is None!");
                return;
            },
            Some(_object_info) => _object_info
        };
        let ev_data = db::DBEvCreateObject::new(object_info);
        let db = match _data.db {
            None => {
                error!("DBProxyThriftServer do_event CreateObjectEvent db is None!");
                return;
            },
            Some(_db) => _db
        };
        let collection = match _data.collection {
            None => {
                error!("DBProxyThriftServer do_event CreateObjectEvent collection is None!");
                return;
            },
            Some(_collection) => _collection
        };
        let callback_id = match _data.callback_id {
            None => {
                error!("DBProxyThriftServer do_event CreateObjectEvent callback_id is None!");
                return;
            },
            Some(_callback_id) => _callback_id
        };
        let ev = db::DBEvent::new(rsp, db::DBEventType::EvCreateObject, db, collection, callback_id, Box::new(ev_data));
        self.queue.enque(Box::new(ev));
    }

    fn do_update_object(&mut self, _data: UpdateObjectEvent, rsp: Arc<Mutex<Queue<Vec<u8>>>>) {
        let query_info = match _data.query_info {
            None => {
                error!("DBProxyThriftServer do_event UpdateObjectEvent object_info is None!");
                return;
            },
            Some(_query_info) => _query_info
        };
        let updata_info = match _data.updata_info {
            None => {
                error!("DBProxyThriftServer do_event UpdateObjectEvent updata_info is None!");
                return;
            },
            Some(_updata_info) => _updata_info
        };
        let _upsert = match _data._upsert {
            None => {
                error!("DBProxyThriftServer do_event UpdateObjectEvent _upsert is None!");
                return;
            },
            Some(_upsert) => _upsert
        };
        let ev_data = db::DBEvUpdataObject::new(query_info, updata_info, _upsert);
        let db = match _data.db {
            None => {
                error!("DBProxyThriftServer do_event UpdateObjectEvent db is None!");
                return;
            },
            Some(_db) => _db
        };
        let collection = match _data.collection {
            None => {
                error!("DBProxyThriftServer do_event UpdateObjectEvent collection is None!");
                return;
            },
            Some(_collection) => _collection
        };
        let callback_id = match _data.callback_id {
            None => {
                error!("DBProxyThriftServer do_event UpdateObjectEvent callback_id is None!");
                return;
            },
            Some(_callback_id) => _callback_id
        };
        let ev = db::DBEvent::new(rsp, db::DBEventType::EvUpdataObject, db, collection, callback_id, Box::new(ev_data));
        self.queue.enque(Box::new(ev));
    }

    fn do_find_and_modify(&mut self, _data: FindAndModifyEvent, rsp: Arc<Mutex<Queue<Vec<u8>>>>) {
        let query_info = match _data.query_info {
            None => {
                error!("DBProxyThriftServer do_event FindAndModifyEvent query_info is None!");
                return;
            },
            Some(_query_info) => _query_info
        };
        let updata_info = match _data.updata_info {
            None => {
                error!("DBProxyThriftServer do_event FindAndModifyEvent updata_info is None!");
                return;
            },
            Some(_updata_info) => _updata_info
        };
        let _new = match _data._new {
            None => {
                error!("DBProxyThriftServer do_event FindAndModifyEvent _new is None!");
                return;
            },
            Some(_new) => _new
        };
        let _upsert = match _data._upsert {
            None => {
                error!("DBProxyThriftServer do_event FindAndModifyEvent _upsert is None!");
                return;
            },
            Some(_upsert) => _upsert
        };
        let ev_data = db::DBEvFindAndModify::new(query_info, updata_info, _new, _upsert);
        let db = match _data.db {
            None => {
                error!("DBProxyThriftServer do_event FindAndModifyEvent db is None!");
                return;
            },
            Some(_db) => _db
        };
        let collection = match _data.collection {
            None => {
                error!("DBProxyThriftServer do_event FindAndModifyEvent collection is None!");
                return;
            },
            Some(_collection) => _collection
        };
        let callback_id = match _data.callback_id {
            None => {
                error!("DBProxyThriftServer do_event FindAndModifyEvent callback_id is None!");
                return;
            },
            Some(_callback_id) => _callback_id
        };
        let ev = db::DBEvent::new(rsp, db::DBEventType::EvFindAndModify, db, collection, callback_id, Box::new(ev_data));
        self.queue.enque(Box::new(ev));
    }

    fn do_remove_object(&mut self, _data: RemoveObjectEvent, rsp: Arc<Mutex<Queue<Vec<u8>>>>) {
        let query_info = match _data.query_info {
            None => {
                error!("DBProxyThriftServer do_event RemoveObjectEvent query_info is None!");
                return;
            },
            Some(_query_info) => _query_info
        };
        let ev_data = db::DBEvRemoveObject::new(query_info);
        let db = match _data.db {
            None => {
                error!("DBProxyThriftServer do_event RemoveObjectEvent db is None!");
                return;
            },
            Some(_db) => _db
        };
        let collection = match _data.collection {
            None => {
                error!("DBProxyThriftServer do_event RemoveObjectEvent collection is None!");
                return;
            },
            Some(_collection) => _collection
        };
        let callback_id = match _data.callback_id {
            None => {
                error!("DBProxyThriftServer do_event RemoveObjectEvent callback_id is None!");
                return;
            },
            Some(_callback_id) => _callback_id
        };
        let ev = db::DBEvent::new(rsp, db::DBEventType::EvRemoveObject, db, collection, callback_id, Box::new(ev_data));
        self.queue.enque(Box::new(ev));
    }

    fn do_get_object_info(&mut self, _data: GetObjectInfoEvent, rsp: Arc<Mutex<Queue<Vec<u8>>>>) {
        let query_info = match _data.query_info {
            None => {
                error!("DBProxyThriftServer do_event GetObjectInfoEvent query_info is None!");
                return;
            },
            Some(_query_info) => _query_info
        };
        let skip = match _data.skip {
            None => {
                error!("DBProxyThriftServer do_event GetObjectInfoEvent skip is None!");
                return;
            },
            Some(_skip) => _skip
        };
        let limit = match _data.limit {
            None => {
                error!("DBProxyThriftServer do_event GetObjectInfoEvent limit is None!");
                return;
            },
            Some(_limit) => _limit
        };
        let sort = match _data.sort {
            None => {
                error!("DBProxyThriftServer do_event GetObjectInfoEvent sort is None!");
                return;
            },
            Some(_sort) => _sort
        };
        let ascending = match _data.ascending {
            None => {
                error!("DBProxyThriftServer do_event GetObjectInfoEvent ascending is None!");
                return;
            },
            Some(_ascending) => _ascending
        };
        let ev_data = db::DBEvGetObjectInfo::new(query_info, skip, limit, sort, ascending);
        let db = match _data.db {
            None => {
                error!("DBProxyThriftServer do_event GetObjectInfoEvent db is None!");
                return;
            },
            Some(_db) => _db
        };
        let collection = match _data.collection {
            None => {
                error!("DBProxyThriftServer do_event GetObjectInfoEvent collection is None!");
                return;
            },
            Some(_collection) => _collection
        };
        let callback_id = match _data.callback_id {
            None => {
                error!("DBProxyThriftServer do_event GetObjectInfoEvent callback_id is None!");
                return;
            },
            Some(_callback_id) => _callback_id
        };
        let ev = db::DBEvent::new(rsp, db::DBEventType::EvGetObjectInfo, db, collection, callback_id, Box::new(ev_data));
        self.queue.enque(Box::new(ev));
    }

    fn do_get_object_count(&mut self, _data: GetObjectCountEvent, rsp: Arc<Mutex<Queue<Vec<u8>>>>) {
        let query_info = match _data.query_info {
            None => {
                error!("DBProxyThriftServer do_event GetObjectCountEvent query_info is None!");
                return;
            },
            Some(_query_info) => _query_info
        };
        let ev_data = db::DBEvGetObjectCount::new(query_info);
        let db = match _data.db {
            None => {
                error!("DBProxyThriftServer do_event GetObjectCountEvent db is None!");
                return;
            },
            Some(_db) => _db
        };
        let collection = match _data.collection {
            None => {
                error!("DBProxyThriftServer do_event GetObjectCountEvent collection is None!");
                return;
            },
            Some(_collection) => _collection
        };
        let callback_id = match _data.callback_id {
            None => {
                error!("DBProxyThriftServer do_event GetObjectCountEvent callback_id is None!");
                return;
            },
            Some(_callback_id) => _callback_id
        };
        let ev = db::DBEvent::new(rsp, db::DBEventType::EvGetGuid, db, collection, callback_id, Box::new(ev_data));
        self.queue.enque(Box::new(ev));
    }

    fn do_event(_handle: Arc<Mutex<DBProxyThriftServer>>, rsp: Arc<Mutex<Queue<Vec<u8>>>>, data: Vec<u8>) {
        let mut _p = _handle.as_ref().lock().unwrap();
        let ev = match deserialize(data) {
            Err(e) => {
                error!("DBProxyThriftServer do_event err:{}", e);
                return;
            }
            Ok(d) => d
        };
        match ev {
            DbEvent::RegHub(_) => {},
            DbEvent::GetGuid(_data) => {
                _p.do_get_guid(_data, rsp);
            },
            DbEvent::CreateObject(_data) => {
                _p.do_create_object(_data, rsp);
            },
            DbEvent::UpdateObject(_data) => {
                _p.do_update_object(_data, rsp);
            },
            DbEvent::FindAndModify(_data) => {
                _p.do_find_and_modify(_data, rsp);
            },
            DbEvent::RemoveObject(_data) => {
                _p.do_remove_object(_data, rsp);
            },
            DbEvent::GetObjectInfo(_data) => {
                _p.do_get_object_info(_data, rsp);
            },
            DbEvent::GetObjectCount(_data) => {
                _p.do_get_object_count(_data, rsp);
            }
        }
    }

    async fn poll(&mut self) {
        let begin = utc_unix_timer();
        loop {
            let opt_ev_data = self.queue.deque();
            match opt_ev_data {
                None => break,
                Some(ev_data) => {
                    let mut mut_ev_data = ev_data;
                    mut_ev_data.do_event(&mut self.proxy).await;
                }
            }
        }
        let tick = utc_unix_timer() - begin;

        if tick < 33 {
            thread::sleep(Duration::from_millis((33 - tick) as u64));
        }
    }
}