mod error;
mod responder;

#[macro_use] extern crate rocket;

extern crate reqwest;

use dotenv::dotenv;
use std::env;

use url::Url;
use reqwest::{ClientBuilder, Proxy};

use crate::error::Error;
use crate::responder::image::Image;

#[get("/imageProxy?<url>")]
async fn image_proxy(url: &str) -> Result<Image, Error> {

    let url = Url::parse(url)?;

    let proxy = Proxy::https(
        env::var("PROXY_URL")?
    )?
    .basic_auth(
        env::var("PROXY_USERNAME")?.as_str(),
        env::var("PROXY_PASSWORD")?.as_str()
    );

    let client = ClientBuilder::new()
        .proxy(proxy)
        .build()?;

    let response = client
        .get(url)
        .send()
        .await?;

    let content_type = response.headers()
        .get("content-type")
        .ok_or(Error::ContentType)?
        .to_str()?
        .to_string();

    if !content_type.starts_with("image/") { Err(Error::ContentType)? }

    let bytes = response
        .bytes()
        .await?
        .to_vec();

    Ok(Image { bytes, content_type })
}


#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build().mount("/api", routes![image_proxy])
}