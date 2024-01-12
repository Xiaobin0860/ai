use std::{collections::HashMap, fs};

use serde_json::Value;

use crate::{AppResult, Node};

/// The comfy ui workflow !先要求每种类型的node只有一个, 多个的话就用stack类node, 找接入参数流程应该可以满足
#[derive(Debug)]
pub struct Workflow {
    /// map from node_id to node
    id_node_map: HashMap<String, Node>,
    /// map from node_type to node_id
    type_id_map: HashMap<String, String>,
}

impl Workflow {
    pub fn from_json(json: &str) -> AppResult<Self> {
        let id_node_map: HashMap<String, Node> = serde_json::from_str(json)?;
        let mut type_id_map = HashMap::new();
        for (id, node) in &id_node_map {
            if type_id_map
                .insert(node.class_type.clone(), id.clone())
                .is_some()
            {
                return Err(format!("duplicate node type: {}", node.class_type).into());
            }
        }
        Ok(Self {
            id_node_map,
            type_id_map,
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

    fn get_node_id(&self, typ: &str) -> AppResult<&String> {
        self.type_id_map
            .get(typ)
            .ok_or(format!("get_node_id: {typ} not found").into())
    }
}
