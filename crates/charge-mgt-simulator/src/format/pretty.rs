use serde_json::Value;

pub fn pretty_json(v: &Value) -> String {
    serde_json::to_string_pretty(v).unwrap_or_else(|_| v.to_string())
}

pub fn pretty_one_line(v: &Value) -> String {
    match v {
        Value::Null => "null".to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.to_string(),
        Value::String(s) => format!("\"{s}\""),
        Value::Array(_) | Value::Object(_) => {
            let txt = serde_json::to_string(v).unwrap_or_else(|_| v.to_string());
            if txt.len() > 80 { pretty_json(v) } else { txt }
        }
    }
}
