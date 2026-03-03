use std::collections::HashMap;
use serde_json::{Value, from_str};


pub fn parse_msg_to_json(msg: &str) -> HashMap<String, String> {
    let json = from_str::<Value>(msg);
    match json {
        Ok(json) => {
            let json_map = json.as_object();
            match json_map {
                Some(json_map) => {
                    let mut map: HashMap<String, String> = HashMap::new();
                    for (key, value) in json_map {
                        let normalized = match value {
                            Value::String(s) => s.clone(),
                            _ => value.to_string(),
                        };
                        map.insert(key.to_string(), normalized);
                    }
                    map
                },
                None => {
                    // cerrar conexion
                    HashMap::new()
                },
            }
        },
        Err(_) => {
            // cerrar conexion
            HashMap::new()
        }
    }

}