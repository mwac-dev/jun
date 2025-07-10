use crate::parser::parse_fields;
use serde_json::Value;
use std::collections::HashMap;
use tera::{Context, Tera};

pub fn generate_classes(json_data: &Value, root_name: &str) -> HashMap<String, String> {
    let tera = Tera::new("templates/*.tera").expect("Failed to load templates");

    let mut classes = HashMap::new();
    generate_class_recursive(json_data, root_name, &tera, &mut classes);

    classes
}

fn generate_class_recursive(
    json_data: &Value,
    class_name: &str,
    tera: &Tera,
    classes: &mut HashMap<String, String>,
) {
    let fields = parse_fields(json_data);

    // Insert the root class
    let mut context = Context::new();
    context.insert("class_name", class_name);
    context.insert("fields", &fields);

    let rendered = tera
        .render("class.tera", &context)
        .expect("Template render failed");
    classes.insert(class_name.to_string(), rendered);

    // Handle nested objects
    for field in &fields {
        if let Some(nested) = &field.nested_object {
            generate_class_recursive(nested, &field.csharp_type, tera, classes);
        }
    }
}
