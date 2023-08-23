use std::collections::HashMap;
use async_std::sync::{Arc, RwLock};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum DatabaseId {
    Default,
    Custom(String),
}

pub struct Storage {
    databases: Arc<RwLock<HashMap<DatabaseId, HashMap<String, String>>>>,
}

impl Storage {
    pub fn new() -> Self {
        Storage {
            databases: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    #[allow(dead_code)]
    pub async fn get_database(&self, db_id: DatabaseId) -> Option<HashMap<String, String>> {
        let databases = self.databases.read().await;
        if let Some(db) = databases.get(&db_id) {
            Some(db.clone())
        } else {
            None
        }
    }

    pub async fn set(&self, db_id: DatabaseId, key: &String, value: &String) {
        let mut databases = self.databases.write().await;
        let db = databases.entry(db_id).or_insert_with(HashMap::new);
        let key = key.clone();
        let value = value.clone();
        db.insert(key, value);
    }

    pub async fn get(&self, db_id: DatabaseId, key: &str) -> Option<String> {
        let databases = self.databases.read().await;
        if let Some(db) = databases.get(&db_id) {
            if let Some(value) = db.get(key) {
                return Some(value.clone());
            }
        }
        None
    }

    pub async fn remove(&self, db_id: DatabaseId, key: &str) -> Option<String> {
        let mut databases = self.databases.write().await;
        if let Some(db) = databases.get_mut(&db_id) {
            if let Some(value) = db.remove(key) {
                Some(value)
            } else {
                None
            }
        } else {
            None
        }
    }
}
