use std::collections::BTreeMap;

pub struct Entity {
    entity_id: String,
    hub_name: String,
    main_conn_id: Option<String>,
    conn_ids: Vec<String>
}

impl Entity {
    pub fn new(_entity_id: String, _hub_name: String) -> Entity {
        Entity {
            entity_id: _entity_id,
            hub_name: _hub_name,
            main_conn_id: None,
            conn_ids: vec![]
        }
    }

    pub fn set_main_conn_id(&mut self, id: Option<String>) {
        self.main_conn_id = id
    }

    pub fn get_main_conn_id(&self) -> Option<String> {
        self.main_conn_id.clone()
    }

    pub fn add_conn_id(&mut self, id: String) {
        self.conn_ids.push(id)
    }

    pub fn get_conn_ids(&self) -> &Vec<String> {
        &self.conn_ids
    }
}

pub struct EntityManager {
    entities: BTreeMap<String, Entity>
}

impl EntityManager {
    pub fn new() -> EntityManager {
        EntityManager {
            entities: BTreeMap::new()
        }
    }

    pub fn update_entity(&mut self, e: Entity) {
        let entity_id = e.entity_id.clone();
        self.entities.insert(entity_id, e);
    }

    pub fn get_entity(&mut self, entity_id: &String) -> Option<&mut Entity> {
        self.entities.get_mut(entity_id)
    }

    pub fn delete_entity(&mut self, entity_id: &String) -> Option<Entity> {
        self.entities.remove(entity_id)
    }
}