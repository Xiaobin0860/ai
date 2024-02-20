use anyhow::Context;
use macros::FromValue;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Text String
#[derive(Debug, Serialize, Deserialize, Clone, FromValue)]
pub struct TextString {
    /// positive prompts
    pub text: String,
    pub text_b: String,
    pub text_c: String,
    pub text_d: String,
}

/// Text Concatenate (JPS)
#[derive(Debug, Serialize, Deserialize, Clone, FromValue)]
pub struct TextConcat {
    /// delimiter comma|space|none
    pub delimiter: String,
    /// TextString.text
    pub text1: Option<Value>,
    /// Tagger result
    pub text2: Option<Value>,
    pub text3: Option<Value>,
    pub text4: Option<Value>,
}
