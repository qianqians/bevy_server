use mongodb::{Client, options::ClientOptions};
use mongodb::bson::{Bson, Array, Document};


pub trait MongoProxy {
    fn create_index(&mut self, db: String, collection: String, key: String, is_unique: bool) -> bool;
    fn check_int_guid(&mut self, db: String, collection: String, initial_guid: u32) -> bool;
    fn save(&mut self, db: String, collection: String, data: Vec<u8>) -> bool; 
    fn update(&mut self, db: String, collection: String, query: Vec<u8>, update: Vec<u8>) -> bool; 
    fn find_and_modify(&mut self, db: String, collection: String, query: Vec<u8>, update: Vec<u8>, _new: bool, _upsert: bool) -> bool;
    fn find(&mut self, db: String, collection: String, query: Vec<u8>, skip: u32, limit: u32, sort: String, _ascending: bool) -> Array;
    fn count(&mut self, db: String, collection: String, query: Vec<u8>) -> u32;
    fn remove(&mut self, db: String, collection: String, query: Vec<u8>) -> bool;
    fn get_guid(&mut self, db: String, collection: String) -> u32;
}