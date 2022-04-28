use bevy_ecs::prelude::Entity;
use common::data::USERNAMES;
use naia_bevy_server::UserKey;
use std::collections::HashMap;

pub struct GoodUser {
    user_key: UserKey,
    username: String,
    pub entity: Option<Entity>,
}

impl GoodUser {
    pub fn new(user_key: UserKey, username: &String) -> Self {
        Self {
            user_key,
            username: username.clone(),
            entity: None,
        }
    }
}

#[derive(Default)]
pub struct UserState {
    users: HashMap<UserKey, GoodUser>,
}

impl UserState {
    pub fn create(&mut self, key: UserKey, user: GoodUser) {
        self.users.insert(user.user_key, user);
    }

    pub fn find(&self, user_key: &UserKey) -> Option<&GoodUser> {
        self.users.get(user_key)
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
