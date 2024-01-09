use std::collections::HashMap;

use reqwest::Client;
use serde::Deserialize;
use serde_json::Value;
use tracing::info;

#[derive(Debug, Deserialize)]
pub struct WsMsg {
    pub typ: String,
    pub data: Value,
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
        info!("{api} {data:?}");
        self.client
            .post(api)
            .json(&data)
            .send()
            .await
            .expect("Failed to send prompt");
    }
}
