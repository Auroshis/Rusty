// src/main.rs
use serde_json::json;
use tera::{Tera, Context, Value, from_value, to_value, Result};

fn main() -> Result<()> {
    // Create initial JSON data
    let mut user_data = json!({
        "name": "Alice",
        "age": 30,
        "address": {
            "city": "Wonderland",
            "zip": "12345"
        }
    });

    // Print initial JSON data
    println!("Initial JSON data: {}", user_data);

    // Update JSON data using to_value and from_value
    let updated_age = 31;
    let updated_age_value = to_value(updated_age)?;
    let mut user_map = from_value::<serde_json::Map<String, Value>>(user_data)?;

    user_map.insert("age".to_string(), updated_age_value);
    user_data = Value::Object(user_map);

    // Print updated JSON data
    println!("Updated JSON data: {}", user_data);

    // Create TERA instance and load templates
    let tera = Tera::new("templates/**/*")?;

    // Create a context and insert JSON data
    let mut context = Context::new();
    context.insert("user", &user_data);

    // Render the template with the context
    let rendered = tera.render("profile.html", &context)?;

    // Print the rendered template
    println!("{}", rendered);

    Ok(())
}
