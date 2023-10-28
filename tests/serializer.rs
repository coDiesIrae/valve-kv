use serde::Serialize;
use valve_kv::serializer::to_string;

#[test]
fn test_struct() {
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
"#
    .trim();

    let res = to_string(&test).unwrap();

    assert_eq!(res, expected);
}
