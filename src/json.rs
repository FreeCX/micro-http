use crate::http;
use std::collections::HashMap;

pub fn serialize(data: HashMap<String, String>) -> http::HttpData {
    let mut content = data.into_iter().map(|(k, v)| format!("\"{k}\":\"{v}\"")).collect::<Vec<String>>().join(",");
    content.insert(0, '{');
    content.push('}');

    http::HttpData::from_content("application/json", content)
}

pub fn deserialize(data: &[u8]) -> Option<HashMap<String, String>> {
    let mut result = HashMap::new();

    let data = String::from_utf8(data.to_vec()).ok()?;
    for item in data.replace(['{', '}'], " ").split(',') {
        let index = item.find(':')?;
        let (key, value) = item.split_at(index);
        let key = key.replace('"', " ").trim().to_string();
        let value = value[1..].replace('"', " ").trim().to_string();
        result.insert(key, value);
    }

    Some(result)
}
