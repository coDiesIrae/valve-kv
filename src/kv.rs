#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyValue {
    pub key: String,
    pub value: Value,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Value(String),
    Section(Vec<KeyValue>),
}

impl Default for KeyValue {
    fn default() -> Self {
        Self {
            key: Default::default(),
            value: Value::Value(Default::default()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct KeyValueFile {
    pub imports: Vec<String>,
    pub kvs: Vec<KeyValue>,
}

impl From<Vec<KeyValue>> for KeyValueFile {
    fn from(value: Vec<KeyValue>) -> Self {
        KeyValueFile {
            kvs: value,
            ..Default::default()
        }
    }
}
