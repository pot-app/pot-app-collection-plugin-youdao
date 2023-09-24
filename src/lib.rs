use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;

#[no_mangle]
pub fn collection(
    source: &str,
    _target: &str,
    from: &str,
    _to: &str,
    needs: HashMap<String, String>,
) -> Result<Value, Box<dyn Error>> {
    let client = reqwest::blocking::ClientBuilder::new().build()?;
    let cookie = match needs.get("cookie") {
        Some(cookie) => cookie.to_string(),
        None => return Err("cookie is required".into()),
    };

    let res: Value = client
        .get("https://dict.youdao.com/wordbook/webapi/v2/ajax/add")
        .header("Cookie", cookie)
        .query(&[("word", source), ("lan", from)])
        .send()?
        .json()?;
    if let Some(json) = res.as_object() {
        if let Some(code) = json.get("code") {
            if code.as_i64().unwrap() == 0 {
                return Ok(Value::Bool(true));
            } else {
                if let Some(msg) = json.get("msg") {
                    return Ok(Value::String(msg.as_str().unwrap().to_string()));
                }
            }
        }
    }
    Ok(Value::Bool(false))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn try_request() {
        let mut needs = HashMap::new();
        needs.insert("cookie".to_string(), "".to_string());
        let result = collection("Hello", "", "en", "zh", needs).unwrap();
        println!("{result}");
    }
}
