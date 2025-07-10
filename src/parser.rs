use crate::model::Field;
use convert_case::{Case, Casing};
use serde_json::Value;
use std::fs;

pub fn parse_json_file(path: &str) -> Value {
    let json_str = fs::read_to_string(path).expect("Failed to read JSON file");
    serde_json::from_str(&json_str).expect("Failed to parse JSON")
}

pub fn parse_fields(json_data: &Value) -> Vec<Field> {
    let mut fields = Vec::new();

    if let Value::Object(map) = json_data {
        for (key, value) in map {
            // Skip special `_class` key
            if key == "_class" {
                continue;
            }

            let (csharp_type, nested_object) = match value {
                Value::String(_) => ("string".to_string(), None),
                Value::Number(_) => ("int".to_string(), None),
                Value::Bool(_) => ("bool".to_string(), None),
                Value::Null => ("object".to_string(), None),
                Value::Array(arr) => {
                    if let Some(first) = arr.get(0) {
                        match first {
                            Value::String(_) => ("List<string>".to_string(), None),
                            Value::Number(_) => ("List<int>".to_string(), None),
                            Value::Bool(_) => ("List<bool>".to_string(), None),
                            Value::Object(obj) => {
                                let nested_class = capitalize(key);
                                (
                                    "List<".to_owned() + &nested_class + ">",
                                    Some(Value::Object(obj.clone())),
                                )
                            }
                            _ => ("List<object>".to_string(), None),
                        }
                    } else {
                        ("List<object>".to_string(), None)
                    }
                }
                Value::Object(obj) => {
                    let nested_class = capitalize(key);
                    (nested_class, Some(Value::Object(obj.clone())))
                }
            };

            fields.push(Field {
                name: key.to_case(Case::Pascal),
                csharp_type,
                nested_object,
                is_array: matches!(value, Value::Array(_)),
            });
        }
    }

    fields
}

fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => s.to_string(),
    }
}
