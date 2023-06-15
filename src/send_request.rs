use reqwest;
use reqwest::Response;

async fn send_request(data: String) -> Result<(), &'static str> {
    let client = reqwest::Client::new();
    let response: Response = client.post("").body(data).send().await.unwrap();
    match response.status() {
        reqwest::StatusCode::OK => {
            return Ok(());
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            return Err("Unauthorized logon request");
        }
        _ => {
            return Err("Bad request");
        }
    };
}
