use std::{collections::VecDeque, fs, path::Path};

use pest::{
    iterators::{Pair, Pairs},
    Parser,
};
use pest_derive::Parser;

use crate::{
    error::Error,
    kv::{KeyValue, KeyValueFile, Value},
};

pub fn parse_file(path: &str) -> Result<Vec<KeyValue>, Error> {
    let base_path = Path::new(path)
        .parent()
        .and_then(|s| s.to_str())
        .unwrap_or("");

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

fn parse_file_impl(path: &str) -> Result<KeyValueFile, Error> {
    let file = String::from_utf8(fs::read(path).map_err(Error::ReadFileError)?)
        .map_err(Error::ReadUtf8Error)?;

    parse_input(&file)
}

#[derive(Parser)]
#[grammar = "src/kv.pest"]
pub struct KeyValueParser;

pub fn parse_input(input: &str) -> Result<KeyValueFile, Error> {
    let pairs = KeyValueParser::parse(Rule::file, input)
        .map_err(|e| Error::ParseKeyValueError(Box::new(e)))?;

    let mut kvf = KeyValueFile::default();

    for pair_outer in pairs {
        if let Rule::file = pair_outer.as_rule() {
            for pair in pair_outer.into_inner() {
                match pair.as_rule() {
                    Rule::import => kvf.imports.push(parse_import(pair)),
                    Rule::keyvalue => kvf.kvs.push(parse_single_kv(pair)),
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

fn parse_section(input: Pairs<Rule>) -> Vec<KeyValue> {
    let mut kvs = Vec::new();

    for pair in input {
        if let Rule::keyvalue = pair.as_rule() {
            kvs.push(parse_single_kv(pair));
        }
    }

    kvs
}
