use rocket::serde::{Serialize, Deserialize};
use sqlx::prelude::FromRow;
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize,FromRow)]
#[serde(crate = "rocket::serde")]
pub struct Note {
    pub id: i32,
    pub content: String,
}

#[derive(Default)]
pub struct Notes {
    pub notes: Mutex<Vec<Note>>,
}
