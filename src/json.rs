use std::collections::HashMap;

use crate::http::Data;

pub type SimpleJson = HashMap<String, String>;

pub fn serialize(data: SimpleJson) -> Data {
    let mut content = data.into_iter().map(|(k, v)| format!("\"{k}\":\"{v}\"")).collect::<Vec<String>>().join(",");
    content.insert(0, '{');
    content.push('}');

    Data::from_content("application/json", content)
}

pub fn deserialize(data: &[u8]) -> Option<SimpleJson> {
    let mut result = SimpleJson::new();

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
