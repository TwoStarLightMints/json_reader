use std::collections::HashMap;
use json_reader::json_reader::*;

#[test]
fn converts_tokens_to_hashmap() {
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
                    JsonToken::JsonKey(String::from("key1")),
                    JsonToken::JsonString(String::from("string")),
                JsonToken::JsonObjEnd,

        JsonToken::JsonObjEnd
        ];

    let mut json_map: HashMap<String, JsonToken> = HashMap::new();
    let mut inner_map: HashMap<String, JsonToken> = HashMap::new();
    inner_map.insert(String::from("key1"), JsonToken::JsonString(String::from("string")));
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