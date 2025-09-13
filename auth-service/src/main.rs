use auth_service::authenticate; 
use auth_service::auth_utils::models::Credentials;

fn main() {
    let credentials = Credentials {
        username: String::from("kartik"),
        password: String::from("3838"),
    };

    authenticate(credentials);
    println!("Hello, world!");
}
