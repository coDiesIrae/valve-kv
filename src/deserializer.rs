use std::collections::VecDeque;

use serde::{
    de::{self, DeserializeSeed, IntoDeserializer, MapAccess, SeqAccess},
    Deserialize,
};

use crate::{
    error::Error,
    kv::{KeyValue, Value},
    parser::parse_input,
};

pub fn from_str<'a, T>(input: &'a str) -> Result<T, Error>
where
    T: Deserialize<'a>,
{
    let parsed = parse_input(input)?;

    let mut deserializer = Deserializer::from_kv(Value::Section(parsed.kvs));
    let t = T::deserialize(&mut deserializer)?;
    Ok(t)
}

pub struct Deserializer {
    input: Value,
}

impl Deserializer {
    pub fn from_kv(input: Value) -> Self {
        Deserializer { input }
    }

    pub fn parse_value<T>(&self) -> Result<T, Error>
    where
        T: std::str::FromStr,
    {
        if let Value::Value(v) = &self.input {
            v.parse().map_err(|_| Error::ExpectedValueError)
        } else {
            Err(Error::ExpectedValueError)
        }
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        if let Value::Value(v) = &self.input {
            if v == "0" {
                visitor.visit_bool(false)
            } else if v == "1" {
                visitor.visit_bool(true)
            } else {
                Err(Error::ParseBoolError)
            }
        } else {
            Err(Error::ExpectedValueError)
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i8(self.parse_value()?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i16(self.parse_value()?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i32(self.parse_value()?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i64(self.parse_value()?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u8(self.parse_value()?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u16(self.parse_value()?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u32(self.parse_value()?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u64(self.parse_value()?)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_f32(self.parse_value()?)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_f64(self.parse_value()?)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        if let Value::Value(v) = &self.input {
            if v.len() == 1 {
                visitor.visit_char(v.chars().next().unwrap())
            } else {
                Err(Error::ExpectedCharError)
            }
        } else {
            Err(Error::ExpectedValueError)
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        if let Value::Value(v) = &self.input {
            visitor.visit_str(v)
        } else {
            Err(Error::ExpectedValueError)
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        if let Value::Value(v) = &self.input {
            visitor.visit_string(v.clone())
        } else {
            Err(Error::ExpectedValueError)
        }
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        if let Value::Value(v) = &self.input {
            if v.is_empty() {
                visitor.visit_none()
            } else {
                visitor.visit_some(self)
            }
        } else {
            Err(Error::ExpectedValueError)
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        if let Value::Value(v) = &self.input {
            if v.is_empty() {
                visitor.visit_unit()
            } else {
                Err(Error::ExpectedUnitError)
            }
        } else {
            Err(Error::ExpectedValueError)
        }
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        if let Value::Section(v) = &self.input {
            visitor.visit_seq(SectionSequence::new(v.clone()))
        } else {
            Err(Error::ExpectedSectionError)
        }
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        if let Value::Section(v) = &self.input {
            visitor.visit_map(SectionMap::new(v.clone()))
        } else {
            Err(Error::ExpectedSectionError)
        }
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        if let Value::Value(v) = &self.input {
            visitor.visit_enum(v.as_str().into_deserializer())
        } else {
            Err(Error::ExpectedValueError)
        }
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
}

struct SectionSequence {
    kvs: Vec<KeyValue>,
    current_index: usize,
}

impl SectionSequence {
    fn new(kvs: Vec<KeyValue>) -> SectionSequence {
        let mut sorted_kvs = kvs;

        sorted_kvs.sort_by(|a, b| a.key.cmp(&b.key));

        SectionSequence {
            kvs: sorted_kvs,
            current_index: 0,
        }
    }
}

impl<'de> SeqAccess<'de> for SectionSequence {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Error>
    where
        T: DeserializeSeed<'de>,
    {
        if self.current_index >= self.kvs.len() {
            return Ok(None);
        }

        let res = seed
            .deserialize(&mut Deserializer {
                input: self.kvs[self.current_index].value.clone(),
            })
            .map(Some);

        self.current_index += 1;

        res
    }
}

struct SectionMap {
    keys: VecDeque<String>,
    values: VecDeque<Value>,
}

impl SectionMap {
    fn new(kvs: Vec<KeyValue>) -> SectionMap {
        let mut section_map = SectionMap {
            keys: VecDeque::new(),
            values: VecDeque::new(),
        };

        for kv in &kvs {
            section_map.keys.push_back(kv.key.clone());
            section_map.values.push_back(kv.value.clone());
        }

        section_map
    }
}

impl<'de> MapAccess<'de> for SectionMap {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        if self.keys.is_empty() {
            return Ok(None);
        }

        let key = self.keys.pop_front().unwrap();

        seed.deserialize(&mut Deserializer {
            input: Value::Value(key),
        })
        .map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let value = self.values.pop_front().unwrap();

        seed.deserialize(&mut Deserializer { input: value })
    }
}
