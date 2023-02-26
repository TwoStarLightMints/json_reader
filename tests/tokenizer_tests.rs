use json_reader::json_reader::*;

#[test]
fn reads_basic_json_string_w_key() {
    let json_string: String = String::from(r#"{"string": "Hello, World!"}"#);
    assert_eq!(vec![JsonToken::JsonObjBeg, JsonToken::JsonKey(String::from("string")), JsonToken::JsonString(String::from("Hello, World!")), JsonToken::JsonObjEnd], tokenize_json_string(&json_string));
}

#[test]
fn reads_json_string_w_escape_character() {
    let json_string: String = String::from(r#"{"string": "Hell\"o, World!"}"#);
    assert_eq!(vec![JsonToken::JsonObjBeg, JsonToken::JsonKey(String::from("string")), JsonToken::JsonString(String::from("Hell\"o, World!")), JsonToken::JsonObjEnd], tokenize_json_string(&json_string));
}

#[test]
fn reads_basic_json_number() {
    let json_string: String = String::from(r#"{"number": 123}"#);
    assert_eq!(vec![JsonToken::JsonObjBeg, JsonToken::JsonKey(String::from("number")), JsonToken::JsonNum(123), JsonToken::JsonObjEnd], tokenize_json_string(&json_string));
}

#[test]
fn reads_basic_json_bool() {
    let json_string: String = String::from(r#"{"boolean": true}"#);
    assert_eq!(vec![JsonToken::JsonObjBeg, JsonToken::JsonKey(String::from("boolean")), JsonToken::JsonBool(true), JsonToken::JsonObjEnd], tokenize_json_string(&json_string));
}

#[test]
fn reads_basic_json_arr_w_bool() {
    let json_string: String = String::from(r#"{"key":[ true ]}"#);
    assert_eq!(vec![JsonToken::JsonObjBeg, JsonToken::JsonKey(String::from("key")), JsonToken::JsonArrBeg, JsonToken::JsonBool(true), JsonToken::JsonArrEnd, JsonToken::JsonObjEnd], tokenize_json_string(&json_string));
}

#[test]
fn reads_basic_json_string_full_parse() {
    let json_string: String = String::from(r#"{
        "hello": "world",
        "bruh": true,
        "arr": ["true", true , 123]
        "obj": {"hello", 123}
    }"#);
    assert_eq!(vec![
        JsonToken::JsonObjBeg,
        JsonToken::JsonKey(String::from("hello")), JsonToken::JsonString(String::from("world")),
        JsonToken::JsonKey(String::from("bruh")), JsonToken::JsonBool(true),
        JsonToken::JsonKey(String::from("arr")),
            JsonToken::JsonArrBeg,
                JsonToken::JsonString(String::from("true")), JsonToken::JsonBool(true), JsonToken::JsonNum(123),
            JsonToken::JsonArrEnd,
        JsonToken::JsonKey(String::from("obj")),
            JsonToken::JsonObjBeg,
                JsonToken::JsonString(String::from("hello")),
                JsonToken::JsonNum(123),
            JsonToken::JsonObjEnd,
        JsonToken::JsonObjEnd], tokenize_json_string(&json_string));
}

#[test]
fn properly_parse_nested_obj() {
    let json_string = String::from(r#"{"object": {"hello": "world"}}"#);
    assert_eq!(vec![JsonToken::JsonObjBeg, JsonToken::JsonKey(String::from("object")), JsonToken::JsonObjBeg, JsonToken::JsonKey(String::from("hello")), JsonToken::JsonString(String::from("world")), JsonToken::JsonObjEnd, JsonToken::JsonObjEnd], tokenize_json_string(&json_string));
}

#[test]
fn properly_parse_nested_arr() {
    let json_string = String::from(r#"{ "nested-array": "does it work?", "arr": [123, [321, true]] }"#);
    assert_eq!(vec![JsonToken::JsonObjBeg, JsonToken::JsonKey(String::from("arr")), JsonToken::JsonArrBeg, JsonToken::JsonNum(123), JsonToken::JsonArrBeg, JsonToken::JsonNum(321), JsonToken::JsonBool(true), JsonToken::JsonArrEnd, JsonToken::JsonArrEnd, JsonToken::JsonObjEnd], tokenize_json_string(&json_string));
}

#[test]
fn detects_key() {
    let key = JsonToken::JsonKey(String::from("This is a key!"));
    assert_eq!(true, key.is_key());
}