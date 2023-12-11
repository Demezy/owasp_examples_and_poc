use rocket::serde::json::Json;
use rocket::serde::uuid::Uuid;
use rocket::serde::{Deserialize, Serialize};
use rocket::tokio::fs::File;
use rocket::tokio::io;
use std::io::Cursor;

use crate::service::BoxedError;

// type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
// create a struct to hold our Date data
// need serialize/deserialize to convert to/from JSON
#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct UploadPictureFile<'r> {
    pub url: &'r str,
}

// get url for file and download it to local storage
#[post("/photo_upload", format = "json", data = "<pic>")]
pub async fn photo_upload(pic: Json<UploadPictureFile<'_>>) -> Result<String, BoxedError> {
    // download file from url
    let filename = Uuid::new_v4().to_string();

    let response = reqwest::get(pic.url).await?;
    let mut file = File::create(format!("content/{}.jpg", filename))
        .await
        .unwrap();

    let mut content = Cursor::new(response.bytes().await?);
    io::copy(&mut content, &mut file).await?;

    Ok(format!("Success, filename: {}", pic.url))
}
