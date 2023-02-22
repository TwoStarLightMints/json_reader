use json_reader::json_reader::*;

#[test]
fn convert_array_tokens_to_vectors() {
    let json_token_vec: Vec<JsonToken> = vec![JsonToken::JsonObjBeg, JsonToken::JsonKey(String::from("hello")), JsonToken::JsonArrBeg, JsonToken::JsonNum(123), JsonToken::JsonBool(true), JsonToken::JsonArrEnd, JsonToken::JsonObjEnd];
    assert_eq!(vec![JsonToken::JsonObjBeg, JsonToken::JsonKey(String::from("hello")), JsonToken::JsonArr(vec![JsonToken::JsonNum(123), JsonToken::JsonBool(true)]), JsonToken::JsonObjEnd], build_json_array(json_token_vec));
}