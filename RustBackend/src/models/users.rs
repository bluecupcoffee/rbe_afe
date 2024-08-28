use std::fmt;
use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    user_id: u64,
    user_name: String,
    pass_hash: String
}

impl User {
    pub fn new(user_name: String, pass_hash: String) -> Self {
        User {
            user_id: 0,
            user_name,
            pass_hash,
        }
    }

    pub fn verify_user(user: &User) -> bool {
        todo!()
    }
}