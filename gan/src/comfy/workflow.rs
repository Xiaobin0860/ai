use std::{collections::HashMap, fs};

use anyhow::Context;
use serde_json::Value;

use crate::{AppResult, Node, NODE_KSAMPLER};

/// The comfy ui workflow
#[derive(Debug)]
pub struct Workflow {
    /// map from node_id to node
    id_node_map: HashMap<String, Node>,
    /// map from title to node_id, 找接入参数流程Node要求title唯一
    title_id_map: HashMap<String, String>,
}

impl Workflow {
    pub fn from_json(json: &str) -> AppResult<Self> {
        let mut id_node_map: HashMap<String, Node> = serde_json::from_str(json)?;
        let mut title_id_map = HashMap::new();
        for (id, node) in id_node_map.iter_mut() {
            node.id = id.clone();
            let title = node.meta.title.clone();
            if title_id_map.insert(title, id.clone()).is_some() {
                return Err(format!("dup: {} <-> {}", node.meta.title, node.class_type).into());
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

    pub fn rem_node(&mut self, title: &str) {
        if let Ok(id) = self.get_node_id(title).cloned() {
            self.id_node_map.remove(&id);
        }
    }

    pub fn get_node_id(&self, title: &str) -> AppResult<&String> {
        Ok(self
            .title_id_map
            .get(title)
            .context(format!("no id {title}"))?)
    }

    pub fn set_seed(&mut self, seed: i64) -> AppResult<()> {
        self.get_node_mut(NODE_KSAMPLER)?.k_sampler_mut().seed = seed;
        Ok(())
    }
}
