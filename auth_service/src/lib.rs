mod auth_utils;
mod database;

pub use auth_utils::models::Credentials;


pub fn authenticate(creds: auth_utils::models::Credentials) {
    if let database::Status::Connected = database::connect_database() {
        auth_utils::login(creds);
    }
}
