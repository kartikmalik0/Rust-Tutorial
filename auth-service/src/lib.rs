#![allow(dead_code, unused_variables)]

pub mod auth_utils;
mod database;
mod utils;

use database::{Status, connect_to_database};
use auth_utils::login;

pub fn authenticate(cred: auth_utils::models::Credentials) {
    if let Status::CONNECTED = connect_to_database() {
        login(cred);
    }
}
