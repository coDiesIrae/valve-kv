use valve_kv::{
    kv::{KeyValue, Value},
    serializer::serialize_kv,
};

#[test]
fn serialize_single_kv() {
    let input = KeyValue {
        key: "key".to_string(),
        value: Value::Value("value".to_string()),
    };

    let output = serialize_kv(input);

    assert_eq!(output, r#""key" "value""#)
}

#[test]
fn serialize_nested_kv() {
    let input = KeyValue {
        key: "key".to_string(),
        value: Value::Section(vec![
            KeyValue {
                key: "key_nested1".to_string(),
                value: Value::Value("value1".to_string()),
            },
            KeyValue {
                key: "key_nested2".to_string(),
                value: Value::Value("value2".to_string()),
            },
        ]),
    };

    let output = serialize_kv(input);

    assert_eq!(
        output,
        "\"key\"\n{\n\t\"key_nested1\" \"value1\"\n\t\"key_nested2\" \"value2\"\n}"
    )
}
