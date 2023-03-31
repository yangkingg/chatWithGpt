use std::env;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Data {
  model: String,
  prompt: String,
  max_tokens: isize,
  temperature: f64,
}

#[derive(Serialize, Deserialize)]
struct Params {
  id: String,
  object: String,
}

#[tauri::command]
pub async fn get_message(prompt: &str) -> Result<String, String> {
  use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
  //
  let mut headers = HeaderMap::new();
  headers.insert(AUTHORIZATION, "Bearer ".parse().unwrap());
  headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
  let client = reqwest::Client::new();
  let data = Data {
    model: "text-davinci-003".to_string(),
    prompt: prompt.to_string(),
    max_tokens: 3000,
    temperature: 0.9,
  };

  match env::var("HTTP_PROXY") {
    Ok(val) => println!("读取到环境变量{}", val),
    Err(e) => println!("读取环境变量失败"),
  }

  let resp = client
    .post("https://api.openai.com/v1/completions")
    .headers(headers)
    .json(&data)
    .send()
    .await
    .map_err(|_| "网络错误")?;

  Ok(resp.text().await.map_err(|_| "json获取失败")?)
}
