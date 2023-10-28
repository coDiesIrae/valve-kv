use serde::Deserialize;
use valve_kv::deserializer::{from_file, from_str};

#[test]
fn single_field_de() {
    #[derive(Debug, Deserialize, PartialEq)]
    struct Test {
        a: String,
    }

    let input = r#""a" "hello""#;

    let res = from_str::<Test>(input).unwrap();

    assert_eq!(
        res,
        Test {
            a: "hello".to_string()
        }
    );
}

#[test]
fn multi_field_de() {
    #[derive(Debug, Deserialize, PartialEq)]
    struct Test {
        a: String,
        b: String,
    }

    let input = r#""a" "hello"
"b" "world""#;

    let res = from_str::<Test>(input).unwrap();

    assert_eq!(
        res,
        Test {
            a: "hello".to_string(),
            b: "world".to_string(),
        }
    );
}

#[test]
fn seq_de() {
    #[derive(Debug, Deserialize, PartialEq)]
    struct Test {
        a: Vec<String>,
    }

    let input = r#"
    "a"
    {
        "0" "hello"
        "1" "world"
    }
    "#;

    let res = from_str::<Test>(input).unwrap();

    assert_eq!(
        res,
        Test {
            a: vec!["hello".to_string(), "world".to_string()],
        }
    );
}

#[test]
fn wrong_order_seq_de() {
    #[derive(Debug, Deserialize, PartialEq)]
    struct Test {
        a: Vec<String>,
    }

    let input = r#"
    "a"
    {
        "1" "world"
        "0" "hello"
    }
    "#;

    let res = from_str::<Test>(input).unwrap();

    assert_eq!(
        res,
        Test {
            a: vec!["hello".to_string(), "world".to_string()],
        }
    );
}

#[test]
fn map_de() {
    #[derive(Debug, Deserialize, PartialEq)]
    struct Test {
        a: std::collections::HashMap<String, String>,
    }

    let input = r#"
    "a"
    {
        "hello" "world"
        "foo" "bar"
    }
    "#;

    let res = from_str::<Test>(input).unwrap();

    let mut map = std::collections::HashMap::new();
    map.insert("hello".to_string(), "world".to_string());
    map.insert("foo".to_string(), "bar".to_string());

    assert_eq!(res, Test { a: map });
}

#[test]
fn nested_struct_de() {
    #[derive(Debug, Deserialize, PartialEq)]
    struct Test {
        a: String,
        b: Nested,
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct Nested {
        c: String,
        d: String,
    }

    let input = r#"
    "a" "hello"
    "b"
    {
        "c" "world"
        "d" "foo"
    }
    "#;

    let res = from_str::<Test>(input).unwrap();

    assert_eq!(
        res,
        Test {
            a: "hello".to_string(),
            b: Nested {
                c: "world".to_string(),
                d: "foo".to_string(),
            },
        }
    );
}

#[test]
fn bool_de() {
    #[derive(Debug, Deserialize, PartialEq)]
    struct Test {
        a: bool,
        b: bool,
    }

    let input = r#"
    "a" "1"
    "b" "0"
    "#;

    let res = from_str::<Test>(input).unwrap();

    assert_eq!(res, Test { a: true, b: false });
}

#[test]
fn struct_map_de() {
    #[derive(Debug, Deserialize, PartialEq)]
    struct Test {
        a: std::collections::HashMap<String, Nested>,
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct Nested {
        b: String,
        c: String,
    }

    let input = r#"
    "a"
    {
        "hello"
        {
            "b" "world"
            "c" "foo"
        }
        "foo"
        {
            "b" "bar"
            "c" "baz"
        }
    }
    "#;

    let res = from_str::<Test>(input).unwrap();

    let mut map = std::collections::HashMap::new();
    map.insert(
        "hello".to_string(),
        Nested {
            b: "world".to_string(),
            c: "foo".to_string(),
        },
    );
    map.insert(
        "foo".to_string(),
        Nested {
            b: "bar".to_string(),
            c: "baz".to_string(),
        },
    );

    assert_eq!(res, Test { a: map });
}

#[test]
fn file_de() {
    #[derive(Debug, Deserialize, PartialEq)]
    struct Test {
        a: String,
        b: String,
        c: String,
        d: String,
    }

    let res = from_file::<Test>("./tests/test_kvs/base.kv").unwrap();

    assert_eq!(
        res,
        Test {
            a: "hello".to_string(),
            b: "world".to_string(),
            c: "foo".to_string(),
            d: "bar".to_string(),
        }
    );
}

#[test]
fn enum_de() {
    #[derive(Debug, Deserialize, PartialEq)]
    enum Test {
        A,
        B,
        C,
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct TestStruct {
        a: Test,
        b: Test,
        c: Test,
    }

    let input = r#"
    "b" "A"
    "a" "C"
    "c" "B"
    "#;

    let res = from_str::<TestStruct>(input).unwrap();

    assert_eq!(
        res,
        TestStruct {
            a: Test::C,
            b: Test::A,
            c: Test::B,
        }
    );
}
