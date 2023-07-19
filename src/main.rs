mod get;
mod send;

#[tokio::main]
async fn main() {
    let get_item = "json/health_temperature";
    let get_json = get::get(get_item).await;
}
