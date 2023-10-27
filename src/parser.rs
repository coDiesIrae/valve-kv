use std::{collections::VecDeque, fs, path::Path, string::FromUtf8Error};

use pest::{
    iterators::{Pair, Pairs},
    Parser,
};
use pest_derive::Parser;

use crate::kv::{KeyValue, KeyValueFile, Value};

#[derive(Parser)]
#[grammar = "src/kv.pest"]
pub struct KeyValueTokenizer;

pub fn parse_input(input: &str) -> Result<KeyValueFile, Box<pest::error::Error<Rule>>> {
    let pairs = KeyValueTokenizer::parse(Rule::file, input)?;

    let mut kvf = KeyValueFile::default();

    for pair_outer in pairs {
        if let Rule::file = pair_outer.as_rule() {
            for pair in pair_outer.into_inner() {
                match pair.as_rule() {
                    Rule::keyvalue => kvf.kvs.push(parse_single_kv(pair)),
                    Rule::import => kvf.imports.push(parse_import(pair)),
                    _ => (),
                }
            }
        }
    }

    Ok(kvf)
}

fn parse_import(input: Pair<Rule>) -> String {
    input
        .as_str()
        .trim_start_matches("#base")
        .trim_matches(|p: char| p == '"' || p.is_ascii_whitespace())
        .to_string()
}

fn parse_section(input: Pairs<Rule>) -> Vec<KeyValue> {
    let mut kvs = Vec::new();

    for pair in input {
        if let Rule::keyvalue = pair.as_rule() {
            kvs.push(parse_single_kv(pair));
        }
    }

    kvs
}

fn parse_single_kv(input: Pair<Rule>) -> KeyValue {
    let mut kv = KeyValue::default();

    for pair in input.into_inner() {
        match pair.as_rule() {
            Rule::key => kv.key = pair.as_str().trim_matches('"').to_string(),
            Rule::value => kv.value = Value::Value(pair.as_str().trim_matches('"').to_string()),
            Rule::section => kv.value = Value::Section(parse_section(pair.into_inner())),
            _ => (),
        }
    }

    kv
}

#[derive(Debug)]
pub enum ParseFileError {
    ReadFileError(std::io::Error),
    ReadUtf8Error(FromUtf8Error),
    ParseKeyValueError(Box<pest::error::Error<Rule>>),
}

pub fn parse_file(path: &str) -> Result<Vec<KeyValue>, ParseFileError> {
    let base_path = Path::new(path).parent().unwrap().to_str().unwrap();

    let mut paths: VecDeque<String> = Default::default();
    paths.push_back(path.to_string());

    let mut res: Vec<KeyValue> = vec![];

    while let Some(current_path) = paths.pop_front() {
        let mut file = parse_file_impl(&current_path)?;

        res.append(&mut file.kvs);

        for new_path in file.imports {
            let full_new_path = format!("{}/{}", base_path, new_path);
            paths.push_back(full_new_path);
        }
    }

    Ok(res)
}

fn parse_file_impl(path: &str) -> Result<KeyValueFile, ParseFileError> {
    let file = String::from_utf8(fs::read(path).map_err(ParseFileError::ReadFileError)?)
        .map_err(ParseFileError::ReadUtf8Error)?;

    parse_input(&file).map_err(ParseFileError::ParseKeyValueError)
}

pub trait KVDesereliaze {
    fn from_key_value(input: KeyValue) -> Self;
}
