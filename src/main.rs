pub mod login;
pub mod send_request;
pub mod setting;
use setting::Settings;

#[tokio::main]
async fn main() {
    let settings = Settings::new();
    match settings {
        Ok(setting) => {
            let uri: String = setting.server.uri;
            let port: u16 = setting.server.port;
            let request_uri: String = format!("{uri}:{port}");
            println!("{}", request_uri);
            println!("Hello, world!");
        }
        Err(_) => {
            panic!("Configuration error detected");
        }
    }
}
