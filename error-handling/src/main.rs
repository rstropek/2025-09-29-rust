use serde_json::Value;
use tokio::{fs::File, io::AsyncReadExt};

async fn read_and_parse(file_name: &str) -> String {
    // 1. Open the file
    let mut f = File::open(file_name).await.unwrap();

    // 2. Read the content
    let mut buf = Vec::new();
    f.read_buf(&mut buf).await.unwrap();

    // 3. Decode UTF8
    let content = String::from_utf8(buf).unwrap();

    // 4. Parse JSON
    let v: Value = serde_json::from_str(&content).unwrap();

    // 5. Extract connection string
    v.get("connectionString").unwrap().to_string()
}

#[tokio::main]
async fn main() {
    let conn_str = read_and_parse("settings.json").await;
    println!("Connection string is: {}", conn_str);
}
