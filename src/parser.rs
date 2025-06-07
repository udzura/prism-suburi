extern crate ruby_prism;

pub fn parse_ruby_code(code: &str) -> Result<(), String> {
    let result = ruby_prism::parse(code.as_bytes());
    let errors: Vec<String> = result.errors().map(|v| v.message().to_string()).collect();
    if !errors.is_empty() {
        return Err(errors.join("\n"));
    }

    dbg!(result.node());

    Ok(())
}