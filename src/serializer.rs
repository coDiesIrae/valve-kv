use crate::kv::{KeyValue, Value};

pub fn serialize_kv(input: KeyValue) -> String {
    format!(r#""{}"{}"#, input.key, serialize_kv_value(input.value))
}

fn serialize_kv_value(input: Value) -> String {
    match input {
        Value::Value(value) => format!(" \"{}\"", value),
        Value::Section(section) => {
            let mut res = "\n{".to_string();

            for kv in section {
                res.push_str("\n\t");
                res.push_str(&serialize_kv(kv));
            }

            res.push_str("\n}");

            res
        }
    }
}
