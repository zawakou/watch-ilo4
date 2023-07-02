use dotenv;
use reqwest::header::{HeaderMap, HeaderValue, COOKIE};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let mut url = dotenv::var("url").unwrap();
    url.push_str("json/health_temperature");
    let cookie = "sessionKey=";
    let mut headers = HeaderMap::new();
    headers.insert(COOKIE, HeaderValue::from_str(cookie).unwrap());

    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .default_headers(headers)
        .build()?;

    let res = client.get(url).send().await?.text().await?;

    println!("{}", res);
    Ok(())
}
