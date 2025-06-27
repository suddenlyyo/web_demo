use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::model::User;

// 内存数据库类型
pub struct UserRepository {
    pub storage: Arc<Mutex<HashMap<u32, User>>>,
}

impl UserRepository {
    pub fn new() -> Self {
        UserRepository {
            storage: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn create(&self, user: User) -> User {
        let mut storage = self.storage.lock().unwrap();
        storage.insert(user.id, user.clone());
        user
    }

    pub fn find_all(&self) -> Vec<User> {
        let storage = self.storage.lock().unwrap();
        storage.values().cloned().collect()
    }

    pub fn find_by_id(&self, id: u32) -> Option<User> {
        let storage = self.storage.lock().unwrap();
        storage.get(&id).cloned()
    }

    pub fn update(&self, id: u32, dto: UpdateUserDto) -> Option<User> {
        let mut storage = self.storage.lock().unwrap();
        if let Some(user) = storage.get_mut(&id) {
            if let Some(username) = dto.username {
                user.username = username;
            }
            if let Some(email) = dto.email {
                user.email = email;
            }
            Some(user.clone())
        } else {
            None
        }
    }

    pub fn delete(&self, id: u32) -> bool {
        let mut storage = self.storage.lock().unwrap();
        storage.remove(&id).is_some()
    }
}