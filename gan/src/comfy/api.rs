use std::collections::HashMap;

use reqwest::Client;
use serde::Deserialize;
use serde_json::{json, Value};
use tracing::info;

#[derive(Debug, Deserialize)]
pub struct WsMsg {
    #[serde(rename = "type")]
    pub typ: String,
    pub data: Value,
}

#[derive(Debug, Deserialize)]
pub struct StatusMsg {
    pub status: Status,
}

#[derive(Debug, Deserialize)]
pub struct Status {
    pub exec_info: ExecInfo,
}

#[derive(Debug, Deserialize)]
pub struct ExecInfo {
    pub queue_remaining: usize,
}

pub struct Comfy {
    host: String,
    uuid: String,
    client: Client,
}

impl Comfy {
    pub fn new(host: &str, uuid: &str) -> Self {
        Self {
            host: host.into(),
            uuid: uuid.into(),
            client: Client::new(),
        }
    }

    pub async fn queue_prompt(&self, prompt: &Value) {
        let api = format!("http://{}/prompt", self.host);
        let mut data = HashMap::new();
        data.insert("prompt", prompt);
        let client_id = self.uuid.clone().into();
        data.insert("client_id", &client_id);
        let extra_data = json!({
            "extra_pnginfo": {
                "workflow": {
                    "nodes": []
                }
            }
        });
        data.insert("extra_data", &extra_data);
        let json = serde_json::to_string(&data).unwrap();
        info!("{json}");
        self.client
            .post(api)
            .body(json)
            .send()
            .await
            .expect("Failed to send prompt");
    }
}

#[cfg(test)]
mod comfy_tests {
    use super::*;

    #[test]
    fn extra_pnginfo_should_work() {
        let data = json!({
            "extra_pnginfo": {
                "workflow": {
                    "nodes": []
                }
            }
        });
        let json = serde_json::to_string(&data).unwrap();
        println!("{json}");
        assert_eq!(json, r#"{"extra_pnginfo":{"workflow":{"nodes":[]}}}"#);
    }
}
