use auth_service::{authenticate, Credentials};

fn main() {
    let creds = Credentials::new("letsgetrusty", "password");
    authenticate(creds);
}
