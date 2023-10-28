use serde::Serialize;
use valve_kv::serializer::to_string;

#[test]
fn struct_ser() {
    #[derive(Serialize)]
    struct Test {
        int: u32,
        seq1: Vec<&'static str>,
        seq2: Vec<&'static str>,
    }

    let test = Test {
        int: 42,
        seq1: vec!["a", "b", "c"],
        seq2: vec!["d", "e", "f"],
    };

    let expected = r#"
{
  "int" "42"
  "seq1" 
  {
    "0" "a"
    "1" "b"
    "2" "c"
  }
  "seq2" 
  {
    "0" "d"
    "1" "e"
    "2" "f"
  }
}
"#
    .trim();

    let res = to_string(&test).unwrap();

    assert_eq!(res, expected);
}

#[test]
fn map_ser() {
    use std::collections::HashMap;

    let mut map = HashMap::new();
    map.insert("a", "b");
    map.insert("c", "d");

    let expected1 = r#"
{
  "a" "b"
  "c" "d"
}
"#
    .trim();

    let expected2 = r#"
{
  "c" "d"
  "a" "b"
}
"#
    .trim();

    let res = to_string(&map).unwrap();

    assert!(res == expected1 || res == expected2);
}

#[test]
fn nested_ser() {
    #[derive(Serialize)]
    struct Test {
        int: u32,
        seq1: Vec<&'static str>,
        seq2: Vec<&'static str>,
    }

    #[derive(Serialize)]
    struct Test2 {
        int: u32,
        seq1: Vec<&'static str>,
        seq2: Vec<&'static str>,
        test: Test,
    }

    let test = Test {
        int: 42,
        seq1: vec!["a", "b", "c"],
        seq2: vec!["d", "e", "f"],
    };

    let test2 = Test2 {
        int: 42,
        seq1: vec!["a", "b", "c"],
        seq2: vec!["d", "e", "f"],
        test,
    };

    let expected = r#"
{
  "int" "42"
  "seq1" 
  {
    "0" "a"
    "1" "b"
    "2" "c"
  }
  "seq2" 
  {
    "0" "d"
    "1" "e"
    "2" "f"
  }
  "test" 
  {
    "int" "42"
    "seq1" 
    {
      "0" "a"
      "1" "b"
      "2" "c"
    }
    "seq2" 
    {
      "0" "d"
      "1" "e"
      "2" "f"
    }
  }
}
    "#
    .trim();

    let res = to_string(&test2).unwrap();

    assert_eq!(res, expected);
}

#[test]
fn enum_ser() {
    #[derive(Serialize)]
    enum TestEnum {
        A,
    }

    #[derive(Serialize)]
    struct Test {
        a: TestEnum,
    }

    let expected = r#"
{
  "a" "A"
}
"#
    .trim();

    let res = to_string(&Test { a: TestEnum::A }).unwrap();

    assert_eq!(res, expected);
}
