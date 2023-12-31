use dotenv;
use reqwest::header::{HeaderMap, HeaderValue, COOKIE};
use serde_json::Value;

pub async fn get(get_item: &str) -> Result<&String, reqwest::Error> {
    let get_json;
    let login_res: Result<String, Box<dyn std::error::Error>> = login().await;
    match login_res {
        Ok(s) => {
            let u: &str = s.as_str();
            let v: Value = serde_json::from_str(u).unwrap();
            let temp_res: Result<String, Box<dyn std::error::Error>> =
                get_temp(&v["session_key"].to_string(), get_item).await;

            match temp_res {
                Ok(s) => {
                    let u: &str = s.as_str();
                    get_json = serde_json::from_str(u).unwrap();
                }
                Err(e) => {
                    println!("{}", e);
                }
            }
        }
        Err(e) => {
            println!("{}", e);
        }
    }
    get_json
}

async fn get_temp(token: &String, get_item: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url = dotenv::var("url").unwrap() + get_item;
    let cookie = format!("sessionKey={}", token.replace("\"", ""));
    let mut headers = HeaderMap::new();
    headers.insert(COOKIE, HeaderValue::from_str(&cookie).unwrap());
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .default_headers(headers)
        .build()?;
    let res = client.get(&url).send().await;

    Ok(res?.text().await?)
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
