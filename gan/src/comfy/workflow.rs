use std::{collections::HashMap, fs};

use serde_json::Value;

use crate::{AppResult, Node};

/// The comfy ui workflow
#[derive(Debug)]
pub struct Workflow {
    /// map from node_id to node
    id_node_map: HashMap<String, Node>,
    /// map from node_title to node_id, 找接入参数流程需要修改的Node要求title唯一
    title_id_map: HashMap<String, String>,
}

impl Workflow {
    pub fn from_json(json: &str) -> AppResult<Self> {
        let id_node_map: HashMap<String, Node> = serde_json::from_str(json)?;
        let mut title_id_map = HashMap::new();
        for (id, node) in &id_node_map {
            let title = &node.meta.title;
            if title_id_map.insert(title.clone(), id.clone()).is_some() {
                return Err(format!("duplicate node: {title}").into());
            }
        }
        Ok(Self {
            id_node_map,
            title_id_map,
        })
    }

    pub fn from_file(json_file: &str) -> AppResult<Self> {
        Self::from_json(fs::read_to_string(json_file)?.as_str())
    }

    pub fn to_json(&self) -> AppResult<Value> {
        serde_json::to_value(&self.id_node_map).map_err(|e| e.into())
    }

    pub fn get_node(&self, title: &str) -> AppResult<&Node> {
        let id = self.get_node_id(title)?;
        self.id_node_map
            .get(id)
            .ok_or(format!("get_node: {title} not found").into())
    }

    pub fn get_node_mut(&mut self, title: &str) -> AppResult<&mut Node> {
        let id = self.get_node_id(title)?.clone();
        self.id_node_map
            .get_mut(&id)
            .ok_or(format!("get_node_mut: {title} not found").into())
    }

    fn get_node_id(&self, title: &str) -> AppResult<&String> {
        self.title_id_map
            .get(title)
            .ok_or(format!("get_node_id: {title} not found").into())
    }
}
