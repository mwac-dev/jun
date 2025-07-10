use serde_json::Value;
use std::collections::HashMap;
use tera::{Context, Tera};

use crate::parser::parse_fields;
use crate::args::{Args};

pub fn generate_classes(json_data: &Value, root_name: &str, args: &Args) -> HashMap<String, String> {
    let tera = Tera::new("templates/*.tera").expect("Failed to load templates");

    let mut classes = HashMap::new();
    generate_class_recursive(json_data, root_name, &tera, &mut classes, args);

    classes
}

fn generate_class_recursive(
    json_data: &Value,
    class_name: &str,
    tera: &Tera,
    classes: &mut HashMap<String, String>,
    args: &Args
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
        if args.nest {
            // 👇 Insert nested class into same file (inline append)
            let nested_render = tera
                .render("class.tera", &{
                    let mut ctx = Context::new();
                    ctx.insert("class_name", &field.csharp_type);
                    ctx.insert("fields", &parse_fields(nested));
                    ctx
                })
                .expect("Nested class template render failed");

            // Append to current class definition
            classes
                .entry(class_name.to_string())
                .and_modify(|main| {
                    main.push_str("\n\n");
                    main.push_str(&nested_render);
                });
        } else {
            // 👇 Generate as separate file like before
            generate_class_recursive(nested, &field.csharp_type, tera, classes,args);
        }
    }
}
}