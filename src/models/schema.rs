// models/schema.rs
use serde::{Serialize, Deserialize};
use sqlx::{mysql::{MyRow}, prelude::*};

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}