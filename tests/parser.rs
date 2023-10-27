use valve_kv::{
    kv::{KeyValue, KeyValueFile, Value},
    parser::parse_input,
};

#[test]
fn parse_single_kv() {
    let input = r#""key" "value""#;

    let kv = parse_input(input).unwrap();

    assert_eq!(
        kv,
        vec![KeyValue {
            key: "key".to_string(),
            value: Value::Value("value".to_string())
        }]
        .into()
    )
}

#[test]
fn parse_multiple_kv() {
    let input = r#"
    "key1" "value1"
    "key2" "value2"
    "#;

    let kv = parse_input(input).unwrap();

    assert_eq!(
        kv,
        vec![
            KeyValue {
                key: "key1".to_string(),
                value: Value::Value("value1".to_string())
            },
            KeyValue {
                key: "key2".to_string(),
                value: Value::Value("value2".to_string())
            }
        ]
        .into()
    )
}

#[test]
fn parse_nested_kv() {
    let input = r#"
    "key"
    {
        "key_nested" "value"
    }
    "#;

    let kv = parse_input(input).unwrap();

    assert_eq!(
        kv,
        vec![KeyValue {
            key: "key".to_string(),
            value: Value::Section(vec![KeyValue {
                key: "key_nested".to_string(),
                value: Value::Value("value".to_string())
            }])
        },]
        .into()
    )
}

#[test]
fn parse_multiple_nested_kv() {
    let input = r#"
    "key1"
    {
        "key1_nested" "value1"
    }
    "key2"
    {
        "key2_nested" "value2"
    }
    "#;

    let kv = parse_input(input).unwrap();

    assert_eq!(
        kv,
        vec![
            KeyValue {
                key: "key1".to_string(),
                value: Value::Section(vec![KeyValue {
                    key: "key1_nested".to_string(),
                    value: Value::Value("value1".to_string())
                }])
            },
            KeyValue {
                key: "key2".to_string(),
                value: Value::Section(vec![KeyValue {
                    key: "key2_nested".to_string(),
                    value: Value::Value("value2".to_string())
                }])
            }
        ]
        .into()
    )
}

#[test]
fn parse_nested_nested() {
    let input = r#"
    "key_outer"
    {
        "key_nested1"
        {
            "key_nested_nested1" "value1"
        }
        "key_nested2"
        {
            "key_nested_nested2" "value2"
        }
    }
    "#;

    let kv = parse_input(input).unwrap();

    assert_eq!(
        kv,
        vec![KeyValue {
            key: "key_outer".to_string(),
            value: Value::Section(vec![
                KeyValue {
                    key: "key_nested1".to_string(),
                    value: Value::Section(vec![KeyValue {
                        key: "key_nested_nested1".to_string(),
                        value: Value::Value("value1".to_string())
                    },])
                },
                KeyValue {
                    key: "key_nested2".to_string(),
                    value: Value::Section(vec![KeyValue {
                        key: "key_nested_nested2".to_string(),
                        value: Value::Value("value2".to_string())
                    },])
                }
            ])
        },]
        .into()
    )
}

#[test]
fn parse_imports() {
    let input = r#"
    #base "import1"
    #base "import2"
    "#;

    let kv = parse_input(input).unwrap();

    assert_eq!(
        kv,
        KeyValueFile {
            kvs: vec![],
            imports: vec!["import1".to_string(), "import2".to_string()]
        }
    )
}
