use dotenv;
use reqwest::header::{HeaderMap, HeaderValue, COOKIE};
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let login_res: Result<String, Box<dyn std::error::Error>> = login().await;
    match login_res {
        Ok(s) => {
            let u: &str = s.as_str();
            let v: Value = serde_json::from_str(u).unwrap();
            println!("{}", v["session_key"]);
        }
        Err(e) => {
            println!("{}", e);
        }
    }

    Ok(())
}

async fn login() -> Result<String, Box<dyn std::error::Error>> {
    let url = dotenv::var("url").unwrap() + "json/login_session";
    let user = dotenv::var("user").unwrap();
    let pw = dotenv::var("pw").unwrap();
    let body = format!(
        "{{\"method\":\"login\",\"user_login\":\"{}\",\"password\":\"{}\"}}",
        user, pw
    );

    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?;
    let res = client.post(&url).body(body).send().await;

    Ok(res?.text().await?)
}
