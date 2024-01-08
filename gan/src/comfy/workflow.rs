use std::{collections::HashMap, fs};

use crate::{AppError, AppResult, Node};

/// The comfy ui workflow
#[derive(Debug)]
pub struct Workflow {
    id_node_map: HashMap<String, Node>,
    type_id_map: HashMap<String, String>,
}

impl Workflow {
    pub fn from_json(json: &str) -> AppResult<Self> {
        let id_node_map: HashMap<String, Node> = serde_json::from_str(json)?;
        let type_id_map = id_node_map
            .iter()
            .map(|(id, node)| (node.class_type.clone(), id.clone()))
            .collect();
        Ok(Self {
            id_node_map,
            type_id_map,
        })
    }

    pub fn from_file(json_file: &str) -> AppResult<Self> {
        Self::from_json(fs::read_to_string(json_file)?.as_str())
    }

    pub fn get_node(&self, typ: &str) -> AppResult<&Node> {
        let id = self.get_node_id(typ)?;
        self.id_node_map
            .get(id)
            .ok_or(AppError::from_string(format!("get_node: {typ} not found")))
    }

    pub fn get_node_mut(&mut self, typ: &str) -> AppResult<&mut Node> {
        let id = self.get_node_id(typ)?.clone();
        self.id_node_map
            .get_mut(&id)
            .ok_or(AppError::from_string(format!(
                "get_node_mut: {typ} not found"
            )))
    }

    fn get_node_id(&self, typ: &str) -> AppResult<&String> {
        self.type_id_map
            .get(typ)
            .ok_or(AppError::from_string(format!(
                "get_node_id: {typ} not found"
            )))
    }
}
