use serde::{de::Visitor, ser::SerializeMap, Deserialize, Serialize};

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

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Value::Value(value) => serializer.serialize_str(value),
            Value::Section(section) => {
                let mut state = serializer.serialize_map(Some(section.len()))?;

                for kv in section.clone() {
                    state.serialize_entry(&kv.key, &kv.value)?;
                }

                state.end()
            }
        }
    }
}

impl<'de> Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ValueVisitor;

        impl<'de> Visitor<'de> for ValueVisitor {
            type Value = Value;

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Value::Value(v.to_string()))
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Value::Value(v))
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut res: Vec<KeyValue> = vec![];

                while let Some((key, value)) = map.next_entry::<String, Value>()? {
                    res.push(KeyValue { key, value });
                }

                Ok(Value::Section(res))
            }

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("string or map with string keys")
            }
        }

        deserializer.deserialize_any(ValueVisitor)
    }
}

impl Default for KeyValue {
    fn default() -> Self {
        Self {
            key: Default::default(),
            value: Value::Value(Default::default()),
        }
    }
}
