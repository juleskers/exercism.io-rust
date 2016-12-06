pub fn hello(input: Option<&str>) -> String {
  match input {
    Some(name) => return format!("Hello, {}!", name).to_string(),
    None       => return "Hello, World!".to_string(),
  }
}

