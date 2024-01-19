use std::{collections::HashMap, fs};

use anyhow::Context;
use serde_json::Value;

use crate::{my_class_map, AppResult, Node};

/// The comfy ui workflow
#[derive(Debug)]
pub struct Workflow {
    /// map from node_id to node
    id_node_map: HashMap<String, Node>,
    /// map from node_typ to node_id, 找接入参数流程Node要求typ唯一
    typ_id_map: HashMap<String, String>,
}

impl Workflow {
    pub fn from_json(json: &str) -> AppResult<Self> {
        let id_node_map: HashMap<String, Node> = serde_json::from_str(json)?;
        let mut typ_id_map = HashMap::new();
        for (id, node) in &id_node_map {
            let typ = &node.class_type;
            if typ_id_map.insert(typ.clone(), id.clone()).is_some() {
                return Err(format!("duplicate node: {typ}").into());
            }
        }
        Ok(Self {
            id_node_map,
            typ_id_map,
        })
    }

    pub fn from_file(json_file: &str) -> AppResult<Self> {
        Self::from_json(fs::read_to_string(json_file)?.as_str())
    }

    pub fn to_json(&self) -> AppResult<Value> {
        serde_json::to_value(&self.id_node_map).map_err(|e| e.into())
    }

    pub fn get_node(&self, typ: &str) -> AppResult<&Node> {
        let id = self.get_node_id(typ)?;
        self.id_node_map
            .get(id)
            .ok_or(format!("get_node: {typ} not found").into())
    }

    pub fn get_node_mut(&mut self, typ: &str) -> AppResult<&mut Node> {
        let id = self.get_node_id(typ)?.clone();
        self.id_node_map
            .get_mut(&id)
            .ok_or(format!("get_node_mut: {typ} not found").into())
    }

    pub fn get_node_id(&self, typ: &str) -> AppResult<&String> {
        let comfy_class = *my_class_map().get(typ).unwrap_or(&typ);
        Ok(self
            .typ_id_map
            .get(comfy_class)
            .context(format!("no id {typ}:{comfy_class}"))?)
    }
}
