use std::error::Error;

use hyper::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let url = "https://crates.io/api/v1/crates/url";

    let client = Client::new();
    let res = client.get(url.parse()?).await;
    dbg!(res);
    Ok(())
}
