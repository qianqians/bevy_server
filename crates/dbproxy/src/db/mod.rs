use std::any::{Any};

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
    pub skip: i32, 
    pub limit: i32, 
    pub sort: String, 
    pub ascending: bool
}

impl DBEvGetObjectInfo {
    pub fn new(_query_info:Vec<u8>, _skip: i32, _limit: i32, _sort: String, _ascending: bool) -> DBEvGetObjectInfo {
        DBEvGetObjectInfo {
            query_info: _query_info,
            skip: _skip,
            limit: _limit,
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
    pub ev_type: DBEventType,
    pub db: String,
    pub collection: String,
    pub callback_id: String,
    pub ev_data: Box<dyn Any>
}

impl DBEvent {
    pub fn new(_ev_type: DBEventType, _db: String, _collection: String,  _callback_id: String, _ev_data: Box<dyn Any>) -> DBEvent {
        DBEvent {
            ev_type: _ev_type,
            db: _db,
            collection: _collection,
            callback_id: _callback_id,
            ev_data: _ev_data
        }
    }
}