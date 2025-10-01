#![allow(unused)]

use std::{fmt::Debug, string::FromUtf8Error};

use serde::Deserialize;
use serde_json::Value;
use thiserror::Error;
use tokio::{fs::File, io::AsyncReadExt};
use anyhow::Result;

#[derive(Debug, Deserialize)]
struct Settings {
    #[serde(rename = "connectionString")]
    connection_string: String,
}

async fn read_and_parse(file_name: &str) -> String {
    let mut f = File::open(file_name).await.unwrap();
    let mut buf = Vec::new();
    f.read_buf(&mut buf).await.unwrap();
    let content = String::from_utf8(buf).unwrap();
    let v: Value = serde_json::from_str(&content).unwrap();
    v.get("connectionString").unwrap().to_string()
    // let settings: Settings = serde_json::from_str(&content).unwrap();
    // settings.connection_string
}

async fn read_and_parse_better(file_name: &str) -> Result<String, &'static str> {
    let mut f = File::open(file_name).await.map_err(|_| "Cannot open file")?;
    let mut buf = Vec::new();
    f.read_buf(&mut buf).await.map_err(|_| "Cannot read file")?;
    let content = String::from_utf8(buf).map_err(|_| "Cannot decode UTF8")?;
    let v: Value = serde_json::from_str(&content).map_err(|_| "Cannot deserialize JSON")?;
    match v.get("connectionString") {
        Some(s) => Ok(s.to_string()),
        None => Err("Setting not found")
    }
}

async fn read_and_parse_better_with_anyhow(file_name: &str) -> Result<String> {
    let mut f = File::open(file_name).await?;
    let mut buf = Vec::new();
    f.read_buf(&mut buf).await?;
    let content = String::from_utf8(buf)?;
    let v: Value = serde_json::from_str(&content)?;
    match v.get("connectionString") {
        Some(s) => Ok(s.to_string()),
        None => Err(anyhow::anyhow!("Setting not found"))
    }
}

#[derive(Error, Debug)]
enum ReadConfigError {
    #[error("error while accessing file")]
    FileAccess(#[from] std::io::Error),
    #[error("error while decoding UTF-8: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
    #[error("error while deserializing JSON")]
    Json(#[from] serde_json::Error),
    #[error("setting not found")]
    SettingNotFound,
}

async fn read_and_parse_even_better(file_name: &str) -> Result<String, ReadConfigError> {
    let mut f = File::open(file_name).await?;
    let mut buf = Vec::new();
    f.read_buf(&mut buf).await?;
    let content = String::from_utf8(buf)?;
    let v: Value = serde_json::from_str(&content)?;
    match v.get("connectionString") {
        Some(s) => Ok(s.to_string()),
        None => Err(ReadConfigError::SettingNotFound)
    }
}

#[tokio::main]
async fn main() {
    // let conn_str = read_and_parse_better_with_anyhow("settings.json").await;
    // match conn_str {
    //     Ok(s) => println!("Connection string is: {}", s),
    //     Err(e) => {
    //         if let Some(io_err) = e.downcast_ref::<std::io::Error>() {
    //             println!("I/O error: {}", io_err);
    //         } else if let Some(utf8_err) = e.downcast_ref::<std::string::FromUtf8Error>() {
    //             println!("UTF-8 error: {}", utf8_err);
    //         } else if let Some(json_err) = e.downcast_ref::<serde_json::Error>() {
    //             println!("JSON error: {}", json_err);
    //         } else {
    //             println!("Other error: {}", e);
    //         }
    //     },
    // }

    let conn_str = read_and_parse_even_better("settings.json").await;
    match conn_str {
        Ok(s) => println!("Connection string is: {}", s),
        Err(e) => {
            match e {
                ReadConfigError::FileAccess(_) => println!("Cannot access file"),
                ReadConfigError::Utf8(err) => println!("Cannot decode UTF-8: {}", err),
                e => println!("{:?}", e),
            }
        }
    }  

}
