use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Serialize)]
pub struct Field {
    pub name: String,
    pub csharp_type: String,
    pub is_array: bool,
    pub nested_object: Option<Value>,
}
