extern crate prism_suburi;

fn main() {
    let code = r#"
def hello_world
  puts "Hello, world!"
end
hello_world
"#;

    match prism_suburi::parse_ruby_code(code) {
        Ok(_) => {
            println!("Parsed successfully");
        }
        Err(e) => {
            eprintln!("Failed to parse code: {}", e);
        }
    }
}
