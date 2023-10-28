use serde::{ser, Serialize};

use crate::error::Error;

pub struct Serializer {
    seq_index: usize,
    output: String,
}

impl Serializer {
    pub fn new() -> Self {
        Self {
            seq_index: 0,
            output: String::new(),
        }
    }

    pub fn try_newline(&mut self) {
        if !self.output.ends_with('\n') && !self.output.is_empty() {
            self.output += "\n";
        }
    }

    pub fn prettify(&mut self) {
        self.prettify_with_tab("  ");
    }

    pub fn prettify_with_tab(&mut self, tab: &str) {
        let mut depth_level = 0;

        let mut res: Vec<String> = vec![];
        let lines = self.output.lines();

        for line in lines {
            if line.ends_with('}') {
                depth_level -= 1;
            }

            let new_line = format!("{}{}", tab.repeat(depth_level), line);

            res.push(new_line);

            if line.ends_with('{') {
                depth_level += 1;
            }
        }

        self.output = res.join("\n");
    }

    // trim starting and ending curly braces
    pub fn trim(&mut self) {
        self.output = self.output[2..self.output.len() - 2].to_string()
    }
}

impl Default for Serializer {
    fn default() -> Self {
        Self::new()
    }
}

pub fn to_string<T>(value: &T) -> Result<String, Error>
where
    T: Serialize,
{
    let mut serializer = Serializer::new();

    value.serialize(&mut serializer)?;
    serializer.trim();
    serializer.prettify();

    Ok(serializer.output)
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = ();

    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(if v { "true" } else { "false" })
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(f64::from(v))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.output += "\"";
        self.output += v;
        self.output += "\"";
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        use ser::SerializeSeq;
        let mut seq = self.serialize_seq(Some(v.len()))?;
        for byte in v {
            seq.serialize_element(byte)?;
        }
        seq.end()
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_str("\"\"")
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        self.try_newline();
        self.output += "{\n";

        variant.serialize(&mut *self)?;

        self.output += " ";

        value.serialize(&mut *self)?;

        self.try_newline();
        self.output += "}\n";

        Ok(())
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        self.seq_index = 0;

        self.try_newline();
        self.output += "{\n";

        Ok(self)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        self.seq_index = 0;

        self.try_newline();
        self.output += "{\n";

        variant.serialize(&mut *self)?;

        self.try_newline();
        self.output += "{\n";

        Ok(self)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        self.try_newline();
        self.output += "{\n";

        Ok(self)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        self.try_newline();
        self.output += "{\n";

        variant.serialize(&mut *self)?;

        self.try_newline();
        self.output += "{\n";

        Ok(self)
    }
}

impl<'a> ser::SerializeSeq for &'a mut Serializer {
    type Ok = ();

    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let index = self.seq_index;

        index.serialize(&mut **self)?;
        self.output += " ";
        value.serialize(&mut **self)?;
        self.try_newline();

        self.seq_index += 1;

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.output += "}\n";
        Ok(())
    }
}

impl<'a> ser::SerializeTuple for &'a mut Serializer {
    type Ok = ();

    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let index = self.seq_index;

        index.serialize(&mut **self)?;
        self.output += " ";
        value.serialize(&mut **self)?;
        self.try_newline();

        self.seq_index += 1;

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.output += "}\n";
        Ok(())
    }
}

impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let index = self.seq_index;

        index.serialize(&mut **self)?;
        self.output += " ";
        value.serialize(&mut **self)?;
        self.try_newline();

        self.seq_index += 1;

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.output += "}\n";
        Ok(())
    }
}

impl<'a> ser::SerializeTupleVariant for &'a mut Serializer {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let index = self.seq_index;

        index.serialize(&mut **self)?;
        self.output += " ";
        value.serialize(&mut **self)?;
        self.try_newline();

        self.seq_index += 1;

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.output += "}\n}\n";
        Ok(())
    }
}

impl<'a> ser::SerializeMap for &'a mut Serializer {
    type Ok = ();

    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        key.serialize(&mut **self)
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.output += " ";
        value.serialize(&mut **self)?;
        self.try_newline();

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.output += "}\n";
        Ok(())
    }
}

impl<'a> ser::SerializeStruct for &'a mut Serializer {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        key.serialize(&mut **self)?;
        self.output += " ";
        value.serialize(&mut **self)?;
        self.try_newline();

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.output += "}\n";
        Ok(())
    }
}

impl<'a> ser::SerializeStructVariant for &'a mut Serializer {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        key.serialize(&mut **self)?;
        self.output += " ";
        value.serialize(&mut **self)?;
        self.try_newline();

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.output += "}\n}\n";
        Ok(())
    }
}
