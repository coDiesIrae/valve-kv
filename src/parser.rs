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
