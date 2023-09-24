use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;

#[no_mangle]
pub fn collection(
    source: &str,
    to: &str,
    needs: HashMap<String, String>,
) -> Result<Value, Box<dyn Error>> {
    let client = reqwest::blocking::ClientBuilder::new().build()?;
    let port = match needs.get("port") {
        Some(port) => port.to_string(),
        None => "8765".to_string(),
    };

    let url = format!("http://localhost:{port}");

    // Impl...

    Ok(Value::Bool(true))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn try_request() {
        let mut needs = HashMap::new();
        needs.insert("port".to_string(), "8765".to_string());
        let result = collection("Hello", "你好", needs).unwrap();
        println!("{result}");
    }
}
