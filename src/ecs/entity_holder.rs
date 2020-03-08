use legion::entity;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EntityHolder {
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    entity: Option<legion::entity::Entity>,

    entity_id: Option<String>,
}

impl EntityHolder {
    pub fn new(entity: legion::entity::Entity) -> Self {
        Self {
            entity: Some(entity),
            entity_id: None,
        }
    }

    pub fn entity(&self) -> legion::entity::Entity {
        self.entity.unwrap()
    }

    pub fn store_entity_id(&mut self) {
        self.entity_id = Some(format!("{}", self.entity.unwrap()));
    }

    pub fn restore_entity(&mut self, entity_dic: &HashMap<String, entity::Entity>) {
        if let Some(entity_id) = self.entity_id.take() {
            self.entity = Some(entity_dic.get(&entity_id).unwrap().clone());
        }
    }
}
