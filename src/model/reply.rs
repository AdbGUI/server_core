#![allow(dead_code)]
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GenericReply<T> {
    pub code: i32,
    pub message: String,
    pub data: T,
}

impl<T> GenericReply<T> {
    ///!
    ///! Static function to return a good response
    ///!
    pub fn ok(message: String, data: T) -> Self {
        Self {
            code: 200,
            message,
            data,
        }
    }

    ///!
    ///! Static function to return a bad response (Internal Error)
    ///!
    pub fn err_internal(message: String, data: T) -> Self {
        Self {
            code: 500,
            message,
            data,
        }
    }
}
