pub mod models;

pub fn login(creds: models::Credentials) {
    // authenticate
    crate::database::get_user();
}

fn logout() {
    // log user out
}
