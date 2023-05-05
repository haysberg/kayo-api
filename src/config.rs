use dotenvy::dotenv;
use tracing::{error};

pub fn from_env() -> (u16, String) {

    dotenv().ok();

    // Initializing the environment variables
    let port = std::env::var("PORT").unwrap_or("3000".to_string());
    let port: u16 = match port.parse(){
        Ok(value) => {
            value
        },
        Err(_) => {
            error!("This is not a valid port number ! Using 3000 instead.");
            3000
        }
    };

    let mongo_uri = std::env::var("MONGO_URI").unwrap_or("mongodb://localhost:27017".into());

    (port, mongo_uri)
}