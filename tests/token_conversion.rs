use std::collections::HashMap;
use json_reader::json_reader::*;

#[test]
fn converts_tokens_to_hashmap_containers_following_each_other() {
    let json_token_vec: Vec<JsonToken> = vec![
        JsonToken::JsonObjBeg,
            JsonToken::JsonKey(String::from("key1")), JsonToken::JsonString(String::from("string")),
            JsonToken::JsonKey(String::from("key2")), JsonToken::JsonBool(true),
            JsonToken::JsonKey(String::from("key3")),
                JsonToken::JsonArrBeg,
                    JsonToken::JsonString(String::from("thing1")),
                    JsonToken::JsonString(String::from("thing2")),
                JsonToken::JsonArrEnd,
            JsonToken::JsonKey(String::from("key4")),
                JsonToken::JsonObjBeg,
                    JsonToken::JsonKey(String::from("key1")), JsonToken::JsonString(String::from("full test")),
                JsonToken::JsonObjEnd,
        JsonToken::JsonObjEnd
        ];

    let mut json_map: HashMap<String, JsonToken> = HashMap::new();
    let mut inner_map: HashMap<String, JsonToken> = HashMap::new();
    inner_map.insert(String::from("key1"), JsonToken::JsonString(String::from("full test")));
    json_map.insert(String::from("key1"), JsonToken::JsonString(String::from("string")));
    json_map.insert(String::from("key2"), JsonToken::JsonBool(true));
    json_map.insert(String::from("key3"), JsonToken::JsonArr(vec![JsonToken::JsonString(String::from("thing1")), JsonToken::JsonString(String::from("thing2"))]));
    json_map.insert(String::from("key4"), JsonToken::JsonObj(inner_map));

    let created_map = match from_json_tokens_to_json_object(&json_token_vec).unwrap() {
        JsonToken::JsonObj(map) => map,
        _ => HashMap::new(),
    };

    assert_eq!(json_map, created_map);
}

#[test]
fn converts_not_nested_tokens_to_hashmap() {
    let json_token_vec: Vec<JsonToken> = vec![
        JsonToken::JsonObjBeg,
            JsonToken::JsonKey(String::from("hello")), JsonToken::JsonString(String::from("world")),
            JsonToken::JsonKey(String::from("num")), JsonToken::JsonNum(123),
            JsonToken::JsonKey(String::from("bool")), JsonToken::JsonBool(true),
        JsonToken::JsonObjEnd,
    ];

    let mut json_map: HashMap<String, JsonToken> = HashMap::new();
    json_map.insert(String::from("hello"), JsonToken::JsonString(String::from("world")));
    json_map.insert(String::from("num"), JsonToken::JsonNum(123));
    json_map.insert(String::from("bool"), JsonToken::JsonBool(true));

    let created_map = match from_json_tokens_to_json_object(&json_token_vec).unwrap() {
        JsonToken::JsonObj(map) => map,
        _ => HashMap::new(),
    };

    assert_eq!(json_map, created_map);
}

#[test]
fn converts_tokens_w_array_to_hashmap() {
    let json_token_vec: Vec<JsonToken> = vec![
        JsonToken::JsonObjBeg,
            JsonToken::JsonKey(String::from("hello")), JsonToken::JsonString(String::from("world")),
            JsonToken::JsonKey(String::from("num")), JsonToken::JsonNum(123),
            JsonToken::JsonKey(String::from("bool")), JsonToken::JsonBool(true),
            JsonToken::JsonKey(String::from("array")),
                JsonToken::JsonArrBeg,
                    JsonToken::JsonNum(123),
                    JsonToken::JsonBool(false),
                JsonToken::JsonArrEnd,
        JsonToken::JsonObjEnd,
    ];

    let mut json_map: HashMap<String, JsonToken> = HashMap::new();
    json_map.insert(String::from("hello"), JsonToken::JsonString(String::from("world")));
    json_map.insert(String::from("num"), JsonToken::JsonNum(123));
    json_map.insert(String::from("bool"), JsonToken::JsonBool(true));
    json_map.insert(String::from("array"), JsonToken::JsonArr(vec![JsonToken::JsonNum(123), JsonToken::JsonBool(false)]));

    let created_map = match from_json_tokens_to_json_object(&json_token_vec).unwrap() {
        JsonToken::JsonObj(map) => map,
        _ => HashMap::new(),
    };

    assert_eq!(json_map, created_map);
}

#[test]
fn converts_tokens_w_nested_array_to_hashmap() {
    let json_token_vec: Vec<JsonToken> = vec![
        JsonToken::JsonObjBeg,
            JsonToken::JsonKey(String::from("hello")), JsonToken::JsonString(String::from("world")),
            JsonToken::JsonKey(String::from("num")), JsonToken::JsonNum(123),
            JsonToken::JsonKey(String::from("bool")), JsonToken::JsonBool(true),
            JsonToken::JsonKey(String::from("array")),
                JsonToken::JsonArrBeg,
                    JsonToken::JsonNum(123),
                    JsonToken::JsonBool(false),
                    JsonToken::JsonArrBeg,
                        JsonToken::JsonNum(321),
                        JsonToken::JsonBool(false),
                    JsonToken::JsonArrEnd,
                JsonToken::JsonArrEnd,
        JsonToken::JsonObjEnd,
    ];

    let mut json_map: HashMap<String, JsonToken> = HashMap::new();
    json_map.insert(String::from("hello"), JsonToken::JsonString(String::from("world")));
    json_map.insert(String::from("num"), JsonToken::JsonNum(123));
    json_map.insert(String::from("bool"), JsonToken::JsonBool(true));
    json_map.insert(String::from("array"), JsonToken::JsonArr(vec![JsonToken::JsonNum(123), JsonToken::JsonBool(false), JsonToken::JsonArr(vec![JsonToken::JsonNum(321), JsonToken::JsonBool(false)])]));

    let created_map = match from_json_tokens_to_json_object(&json_token_vec).unwrap() {
        JsonToken::JsonObj(map) => map,
        _ => HashMap::new(),
    };

    assert_eq!(json_map, created_map);
}

#[test]
fn converts_tokens_w_nested_object_to_hashmap() {
    let json_token_vec: Vec<JsonToken> = vec![
        JsonToken::JsonObjBeg,
            JsonToken::JsonKey(String::from("hello")), JsonToken::JsonString(String::from("world")),
            JsonToken::JsonKey(String::from("num")), JsonToken::JsonNum(123),
            JsonToken::JsonKey(String::from("bool")), JsonToken::JsonBool(true),
            JsonToken::JsonKey(String::from("obj")),
                JsonToken::JsonObjBeg,
                    JsonToken::JsonKey(String::from("key1")), JsonToken::JsonString(String::from("nested obj test")),
                JsonToken::JsonObjEnd,
        JsonToken::JsonObjEnd,
    ];

    let mut nested_map: HashMap<String, JsonToken> = HashMap::new();
    nested_map.insert(String::from("key1"), JsonToken::JsonString(String::from("nested obj test")));

    let mut json_map: HashMap<String, JsonToken> = HashMap::new();
    json_map.insert(String::from("hello"), JsonToken::JsonString(String::from("world")));
    json_map.insert(String::from("num"), JsonToken::JsonNum(123));
    json_map.insert(String::from("bool"), JsonToken::JsonBool(true));
    json_map.insert(String::from("obj"), JsonToken::JsonObj(nested_map));

    let created_map = match from_json_tokens_to_json_object(&json_token_vec).unwrap() {
        JsonToken::JsonObj(map) => map,
        _ => HashMap::new(),
    };

    assert_eq!(json_map, created_map);
}