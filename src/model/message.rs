#![allow(dead_code)]
use uuid::Uuid;

use crate::model::user::User;

#[derive(Debug, Clone)]
pub struct Message {
    pub id: Uuid,
    pub user: User,
    pub body: String,
}

impl Message {
    pub fn new(id: Uuid, user: User, body: &str) -> Self {
        Message {
            id,
            user,
            body: String::from(body),
        }
    }
}
