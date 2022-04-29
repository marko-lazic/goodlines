use bevy_ecs::prelude::Entity;
use common::data::USERNAMES;
use naia_bevy_server::UserKey;
use std::collections::hash_map::Iter;
use std::collections::HashMap;

pub struct GoodUser {
    user_key: UserKey,
    _username: String,
    pub entity: Option<Entity>,
}

impl GoodUser {
    pub fn new(user_key: UserKey, username: &String) -> Self {
        Self {
            user_key,
            _username: username.clone(),
            entity: None,
        }
    }
}

#[derive(Default)]
pub struct UserState {
    users: HashMap<UserKey, GoodUser>,
}

impl UserState {
    pub fn create(&mut self, user: GoodUser) {
        self.users.insert(user.user_key, user);
    }

    pub fn iter(&self) -> Iter<'_, UserKey, GoodUser> {
        self.users.iter()
    }

    pub fn find_mut(&mut self, user_key: &UserKey) -> Option<&mut GoodUser> {
        self.users.get_mut(user_key)
    }

    pub fn remove(&mut self, user_key: &UserKey) -> Option<GoodUser> {
        self.users.remove(user_key)
    }

    pub fn is_authorized(username: &String, password: &String) -> bool {
        if USERNAMES.contains(&&*username.as_str()) && &*password == "12345" {
            true
        } else {
            false
        }
    }
}
