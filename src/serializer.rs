use crate::kv::{KeyValue, Value};

pub fn serialize_kv(input: KeyValue) -> String {
    impl_serialize_kv(input, 0)
}

fn impl_serialize_kv(input: KeyValue, depth: i8) -> String {
    let mut res = "".to_string();

    res = pad_tabs(&res, depth);
    res = format!("{}\"{}\"", res, input.key);

    match input.value {
        Value::Value(value) => {
            res = format!("{} \"{}\"", res, value);

            res
        }
        Value::Section(sections) => {
            res.push('\n');
            res = pad_tabs(&res, depth);
            res.push('{');

            for section in sections {
                res = format!("{}\n{}", res, impl_serialize_kv(section, depth + 1),);
            }

            res.push('\n');
            res = pad_tabs(&res, depth);
            res.push('}');

            res
        }
    }
}

fn pad_tabs(input: &str, depth: i8) -> String {
    let mut res = input.to_string();

    for _ in 0..depth {
        res.push('\t');
    }

    res
}

pub trait KVSerialize {
    fn to_key_value(&self) -> KeyValue;
}

pub fn serialize_obj(input: Box<dyn KVSerialize>) -> String {
    serialize_kv(input.to_key_value())
}
