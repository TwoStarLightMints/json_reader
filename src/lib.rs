use std::{collections::HashMap, fmt};

#[derive(Debug, Clone)]
pub struct InvalidJsonUnwrap;

impl fmt::Display for InvalidJsonUnwrap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid unwrap of JsonToken")
    }
}

#[derive(Debug, Clone)]
pub struct InvalidJson;

impl fmt::Display for InvalidJson {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Json string has invalid syntax")
    }
}

// ====================================================
//
// New token implementation
//
// ====================================================

trait JsonToken {
    fn is_key(&self) -> bool;
    fn is_seperator(&self) -> bool;
    fn is_container(&self) -> bool;
    fn is_value(&self) -> bool;
}

struct JsonKey {
    // This will only ever contain a string so may as well be a struct
    key: String,
}

impl JsonKey {
    fn new(key: String) -> Self {
        Self { key }
    }
}

impl JsonToken for JsonKey {
    fn is_key(&self) -> bool {
        true
    }
    fn is_seperator(&self) -> bool {
        false
    }
    fn is_container(&self) -> bool {
        false
    }
    fn is_value(&self) -> bool {
        false
    }
}

enum JsonSeperator {
    // Used to demarcate the beginning and end of statements in JSON
    JsonArrBeg,
    JsonArrEnd,
    JsonObjBeg,
    JsonObjEnd,
    JsonComma,
}

impl JsonToken for JsonSeperator {
    fn is_key(&self) -> bool {
        false
    }
    fn is_seperator(&self) -> bool {
        true
    }
    fn is_container(&self) -> bool {
        false
    }
    fn is_value(&self) -> bool {
        false
    }
}

enum JsonContainer {
    // This will be the container for all JSON containers
    JsonObj(HashMap<JsonKey, Box<dyn JsonToken>>),
    JsonArr(Vec<Box<dyn JsonToken>>),
}

impl JsonToken for JsonContainer {
    fn is_key(&self) -> bool {
        false
    }
    fn is_seperator(&self) -> bool {
        false
    }
    fn is_container(&self) -> bool {
        true
    }
    fn is_value(&self) -> bool {
        false
    }
}

enum JsonValue {
    // Wrapper for values in that come from JSON
    JString(String),
    JsonNum(i64),
    JsonBool(bool),
}

impl JsonToken for JsonValue {
    fn is_key(&self) -> bool {
        false
    }
    fn is_seperator(&self) -> bool {
        false
    }
    fn is_container(&self) -> bool {
        false
    }
    fn is_value(&self) -> bool {
        true
    }
}

// ====================================================
//
// New token implementation
//
// ====================================================

// ====================================================
//
// Old token implementation
//
// ====================================================

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum JsonTokenOld {
    JsonKey(String),
    JsonString(String),
    JsonNum(i64),
    JsonBool(bool),
    JsonObj(HashMap<String, JsonTokenOld>),
    JsonArr(Vec<JsonTokenOld>),
    JsonArrBeg,
    JsonArrEnd,
    JsonObjBeg,
    JsonObjEnd,
    JsonInvalid,
}

impl JsonTokenOld {
    pub fn is_key(&self) -> bool {
        match self {
            JsonTokenOld::JsonKey(_) => {
                return true;
            }
            _ => {
                return false;
            }
        }
    }

    pub fn is_value(&self) -> bool {
        match self {
            JsonTokenOld::JsonKey(_)
            | JsonTokenOld::JsonArrBeg
            | JsonTokenOld::JsonArrEnd
            | JsonTokenOld::JsonObjBeg
            | JsonTokenOld::JsonObjEnd
            | JsonTokenOld::JsonInvalid => false,
            _ => true,
        }
    }

    pub fn as_str(&self) -> Result<&str, InvalidJsonUnwrap> {
        match self {
            JsonTokenOld::JsonString(str) => Ok(str as &str),
            _ => Err(InvalidJsonUnwrap),
        }
    }

    pub fn as_int(&self) -> Result<i64, InvalidJsonUnwrap> {
        match self {
            JsonTokenOld::JsonNum(num) => Ok(*num),
            _ => Err(InvalidJsonUnwrap),
        }
    }

    pub fn as_bool(&self) -> bool {
        match self {
            JsonTokenOld::JsonBool(bin) => *bin,
            _ => unreachable!(),
        }
    }

    pub fn as_map(&self) -> Result<HashMap<String, JsonTokenOld>, InvalidJsonUnwrap> {
        match self {
            JsonTokenOld::JsonObj(map) => Ok(map.clone()),
            _ => Err(InvalidJsonUnwrap),
        }
    }

    pub fn as_vec(&self) -> Result<Vec<JsonTokenOld>, InvalidJsonUnwrap> {
        match self {
            JsonTokenOld::JsonArr(vector) => Ok(vector.clone()),
            _ => Err(InvalidJsonUnwrap),
        }
    }
}

// ====================================================
//
// Old token implementation
//
// ====================================================

fn tokenize_json_string(json_string: &String) -> Vec<JsonTokenOld> {
    let mut char_inds = json_string.char_indices().peekable();
    let mut tokens: Vec<JsonTokenOld> = Vec::new();

    while let Some((_pos, ch)) = char_inds.next() {
        match ch {
            // Object parsing
            '{' => {
                tokens.push(JsonTokenOld::JsonObjBeg);
            }
            '}' => {
                tokens.push(JsonTokenOld::JsonObjEnd);
            }
            // Array parsing
            '[' => {
                tokens.push(JsonTokenOld::JsonArrBeg);
            }
            ']' => {
                tokens.push(JsonTokenOld::JsonArrEnd);
            }
            // String parsing
            '"' => {
                let mut last_matched: char = ch;
                let str_content: String = char_inds
                    .by_ref()
                    .take_while(|(_pos, c)| {
                        if *c != '"' || last_matched == '\\' {
                            last_matched = *c;
                            return true;
                        }
                        false
                    })
                    .map(|(_pos, c)| c)
                    .collect();

                if let Some((_pos, ch)) = char_inds.peek() {
                    if *ch == ':' {
                        tokens.push(JsonTokenOld::JsonKey(str_content.replace("\\", "")));
                        continue;
                    } else if *ch == ',' || *ch == '{' || *ch == '}' || *ch == '[' || *ch == ']' {
                        tokens.push(JsonTokenOld::JsonString(str_content.replace("\\", "")));
                        continue;
                    }
                }

                while let Some((_pos, _ch)) = char_inds.next() {
                    match char_inds.peek() {
                        Some((_pos, c)) => {
                            if *c == ':' {
                                tokens.push(JsonTokenOld::JsonKey(str_content.replace("\\", "")));
                                break;
                            } else if *c == ',' || *c == '{' || *c == '}' || *c == '[' || *c == ']'
                            {
                                tokens
                                    .push(JsonTokenOld::JsonString(str_content.replace("\\", "")));
                                break;
                            }
                        }
                        None => (),
                    }
                }
            }

            // Number parsing
            c if c.is_numeric() => {
                let mut number: String = String::from(c);
                while let Some((_pos, ch)) = char_inds.next() {
                    number.push(ch);

                    match char_inds.peek() {
                        Some((_pos, c)) => {
                            if !c.is_numeric() {
                                break;
                            }
                        }
                        None => (),
                    }
                }

                tokens.push(JsonTokenOld::JsonNum(number.parse::<i64>().unwrap()));
            }
            // Boolean parsing
            c if c.is_alphabetic() => {
                let mut value: String = String::from(c);

                while let Some((_pos, ch)) = char_inds.next() {
                    value.push(ch);

                    match char_inds.peek() {
                        Some((_pos, c)) => {
                            if c.is_ascii_punctuation() || *c == ' ' {
                                break;
                            }
                        }
                        None => (),
                    }
                }

                let truth = String::from("true");
                let falth = String::from("false");

                if value == truth {
                    tokens.push(JsonTokenOld::JsonBool(true));
                }
                if value == falth {
                    tokens.push(JsonTokenOld::JsonBool(false));
                }
            }
            _ => (),
        }
    }

    return tokens;
}

fn check_is_valid_json(json_token_vec: &Vec<JsonTokenOld>) -> bool {
    let mut num_arr_tokens = 0;
    let mut num_obj_tokens = 0;
    let mut num_vals = 0;
    let mut num_keys = 0;
    let iter = json_token_vec.iter();

    for token in iter {
        match token {
            JsonTokenOld::JsonKey(_) => {
                num_keys += 1;
            }
            JsonTokenOld::JsonString(_) => {
                num_vals += 1;
            }
            JsonTokenOld::JsonNum(_) => {
                num_vals += 1;
            }
            JsonTokenOld::JsonBool(_) => {
                num_vals += 1;
            }
            JsonTokenOld::JsonArrBeg => {
                num_arr_tokens += 1;
            }
            JsonTokenOld::JsonArrEnd => {
                num_arr_tokens += 1;
            }
            JsonTokenOld::JsonObjBeg => {
                num_obj_tokens += 1;
            }
            JsonTokenOld::JsonObjEnd => {
                num_obj_tokens += 1;
            }
            JsonTokenOld::JsonInvalid => {
                return false;
            }
            _ => (),
        }
    }

    // If either of these two values are not even, that means that there is an unclosed object or array
    if num_arr_tokens % 2 != 0 || num_obj_tokens % 2 != 0 {
        return false;
    }

    // If the number of keys is not greater than the number of values, arrays, and objects, then there is a key without a value (not fool proof, but better than nothing)
    if num_keys > num_vals + (num_obj_tokens - 2) + num_arr_tokens {
        return false;
    }

    return true;
}

fn from_json_tokens_to_json_array(json_token_vec: &Vec<JsonTokenOld>) -> JsonTokenOld {
    let mut new_vec: Vec<JsonTokenOld> = Vec::new();

    let mut arr_inds: Vec<usize> = Vec::new();
    let mut obj_inds: Vec<usize> = Vec::new();

    json_token_vec.iter().enumerate().for_each(|(ind, val)| {
        if *val == JsonTokenOld::JsonArrBeg || *val == JsonTokenOld::JsonArrEnd {
            arr_inds.push(ind);
        } else if *val == JsonTokenOld::JsonObjBeg || *val == JsonTokenOld::JsonObjEnd {
            obj_inds.push(ind);
        }
    });

    arr_inds.sort();
    obj_inds.sort();

    let mut nesting = 0;

    for i in arr_inds[0]..arr_inds[arr_inds.len() - 1] {
        if i == 0 || i == arr_inds[arr_inds.len() - 1] {
            continue;
        }

        if json_token_vec[i].is_value() && nesting <= 0 {
            new_vec.push(json_token_vec[i].clone());
        } else {
            match json_token_vec[i] {
                JsonTokenOld::JsonArrBeg => {
                    new_vec.push(from_json_tokens_to_json_array(
                        &json_token_vec[arr_inds[1]..=arr_inds[arr_inds.len() - 2]].to_vec(),
                    ));
                    nesting += 1;
                }
                JsonTokenOld::JsonObjBeg => {
                    new_vec.push(from_json_tokens_to_json_object(
                        &json_token_vec[obj_inds[0]..=obj_inds[obj_inds.len() - 1]].to_vec(),
                    ));
                    nesting += 1;
                }
                JsonTokenOld::JsonArrEnd => {
                    nesting -= 1;
                }
                JsonTokenOld::JsonObjEnd => {
                    nesting -= 1;
                }
                _ => (),
            }
        }
    }

    return JsonTokenOld::JsonArr(new_vec);
}

pub fn from_json_tokens_to_json_object(json_token_vec: &Vec<JsonTokenOld>) -> JsonTokenOld {
    let mut new_map: HashMap<String, JsonTokenOld> = HashMap::new();

    let mut arr_inds: Vec<usize> = Vec::new();
    let mut obj_inds: Vec<usize> = Vec::new();

    json_token_vec.iter().enumerate().for_each(|(ind, val)| {
        if *val == JsonTokenOld::JsonArrBeg || *val == JsonTokenOld::JsonArrEnd {
            arr_inds.push(ind);
        } else if *val == JsonTokenOld::JsonObjBeg || *val == JsonTokenOld::JsonObjEnd {
            obj_inds.push(ind);
        }
    });

    arr_inds.sort();
    obj_inds.sort();

    let mut nesting = 0;

    for i in obj_inds[0]..obj_inds[obj_inds.len() - 1] {
        if i == 0 || i == obj_inds[obj_inds.len() - 1] {
            continue;
        }

        let mut key = String::new();
        if let JsonTokenOld::JsonKey(val) = &json_token_vec[i] {
            key = val.clone();
        }

        if json_token_vec[i].is_key() && nesting <= 0 {
            if json_token_vec[i + 1].is_value() {
                new_map.insert(key.clone(), json_token_vec[i + 1].clone());
            } else {
                match json_token_vec[i + 1] {
                    JsonTokenOld::JsonArrBeg => {
                        new_map.insert(
                            key.clone(),
                            from_json_tokens_to_json_array(
                                &json_token_vec[arr_inds[0]..=arr_inds[arr_inds.len() - 1]]
                                    .to_vec(),
                            ),
                        );
                        nesting += 1;
                    }
                    JsonTokenOld::JsonObjBeg => {
                        new_map.insert(
                            key.clone(),
                            from_json_tokens_to_json_object(
                                &json_token_vec[obj_inds[1]..=obj_inds[obj_inds.len() - 2]]
                                    .to_vec(),
                            ),
                        );
                        nesting += 1;
                    }
                    _ => (),
                }
            }
        } else if nesting > 0 {
            match json_token_vec[i] {
                JsonTokenOld::JsonArrEnd => {
                    nesting -= 1;
                }
                JsonTokenOld::JsonObjEnd => {
                    nesting -= 1;
                }
                _ => (),
            }
        }
    }

    return JsonTokenOld::JsonObj(new_map);
}

pub fn from_json_string(
    json_string: &String,
) -> Result<HashMap<String, JsonTokenOld>, InvalidJson> {
    let token_vec = tokenize_json_string(&json_string);

    if !check_is_valid_json(&token_vec) {
        return Err(InvalidJson);
    }

    match from_json_tokens_to_json_object(&token_vec).as_map() {
        Ok(map) => Ok(map),
        Err(_) => Err(InvalidJson),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn reads_basic_json_string_w_key() {
        let json_string: String = String::from(r#"{"string": "Hello, World!"}"#);
        assert_eq!(
            vec![
                JsonTokenOld::JsonObjBeg,
                JsonTokenOld::JsonKey(String::from("string")),
                JsonTokenOld::JsonString(String::from("Hello, World!")),
                JsonTokenOld::JsonObjEnd
            ],
            tokenize_json_string(&json_string)
        );
    }

    #[test]
    fn reads_json_string_w_escape_character() {
        let json_string: String = String::from(r#"{"string": "Hell\"o, World!"}"#);
        assert_eq!(
            vec![
                JsonTokenOld::JsonObjBeg,
                JsonTokenOld::JsonKey(String::from("string")),
                JsonTokenOld::JsonString(String::from("Hell\"o, World!")),
                JsonTokenOld::JsonObjEnd
            ],
            tokenize_json_string(&json_string)
        );
    }

    #[test]
    fn reads_basic_json_number() {
        let json_string: String = String::from(r#"{"number": 123}"#);
        assert_eq!(
            vec![
                JsonTokenOld::JsonObjBeg,
                JsonTokenOld::JsonKey(String::from("number")),
                JsonTokenOld::JsonNum(123),
                JsonTokenOld::JsonObjEnd
            ],
            tokenize_json_string(&json_string)
        );
    }

    #[test]
    fn reads_basic_json_bool() {
        let json_string: String = String::from(r#"{"boolean": true}"#);
        assert_eq!(
            vec![
                JsonTokenOld::JsonObjBeg,
                JsonTokenOld::JsonKey(String::from("boolean")),
                JsonTokenOld::JsonBool(true),
                JsonTokenOld::JsonObjEnd
            ],
            tokenize_json_string(&json_string)
        );
    }

    #[test]
    fn reads_basic_json_arr_w_bool() {
        let json_string: String = String::from(r#"{"key":[ true ]}"#);
        assert_eq!(
            vec![
                JsonTokenOld::JsonObjBeg,
                JsonTokenOld::JsonKey(String::from("key")),
                JsonTokenOld::JsonArrBeg,
                JsonTokenOld::JsonBool(true),
                JsonTokenOld::JsonArrEnd,
                JsonTokenOld::JsonObjEnd
            ],
            tokenize_json_string(&json_string)
        );
    }

    #[test]
    fn reads_basic_json_string_full_parse() {
        let json_string: String = String::from(
            r#"{
            "hello": "world",
            "bruh": true,
            "arr": ["true", true , 123]
            "obj": {"hello", 123}
        }"#,
        );
        assert_eq!(
            vec![
                JsonTokenOld::JsonObjBeg,
                JsonTokenOld::JsonKey(String::from("hello")),
                JsonTokenOld::JsonString(String::from("world")),
                JsonTokenOld::JsonKey(String::from("bruh")),
                JsonTokenOld::JsonBool(true),
                JsonTokenOld::JsonKey(String::from("arr")),
                JsonTokenOld::JsonArrBeg,
                JsonTokenOld::JsonString(String::from("true")),
                JsonTokenOld::JsonBool(true),
                JsonTokenOld::JsonNum(123),
                JsonTokenOld::JsonArrEnd,
                JsonTokenOld::JsonKey(String::from("obj")),
                JsonTokenOld::JsonObjBeg,
                JsonTokenOld::JsonString(String::from("hello")),
                JsonTokenOld::JsonNum(123),
                JsonTokenOld::JsonObjEnd,
                JsonTokenOld::JsonObjEnd
            ],
            tokenize_json_string(&json_string)
        );
    }

    #[test]
    fn properly_parse_nested_obj() {
        let json_string = String::from(r#"{"object": {"hello": "world"}}"#);
        assert_eq!(
            vec![
                JsonTokenOld::JsonObjBeg,
                JsonTokenOld::JsonKey(String::from("object")),
                JsonTokenOld::JsonObjBeg,
                JsonTokenOld::JsonKey(String::from("hello")),
                JsonTokenOld::JsonString(String::from("world")),
                JsonTokenOld::JsonObjEnd,
                JsonTokenOld::JsonObjEnd
            ],
            tokenize_json_string(&json_string)
        );
    }

    #[test]
    fn properly_parse_nested_arr() {
        let json_string =
            String::from(r#"{ "nested-array": "does it work?", "arr": [123, [321, true]] }"#);
        assert_eq!(
            vec![
                JsonTokenOld::JsonObjBeg,
                JsonTokenOld::JsonKey(String::from("nested-array")),
                JsonTokenOld::JsonString(String::from("does it work?")),
                JsonTokenOld::JsonKey(String::from("arr")),
                JsonTokenOld::JsonArrBeg,
                JsonTokenOld::JsonNum(123),
                JsonTokenOld::JsonArrBeg,
                JsonTokenOld::JsonNum(321),
                JsonTokenOld::JsonBool(true),
                JsonTokenOld::JsonArrEnd,
                JsonTokenOld::JsonArrEnd,
                JsonTokenOld::JsonObjEnd
            ],
            tokenize_json_string(&json_string)
        );
    }

    #[test]
    fn properly_parse_real_example() {
        let json_string = String::from(
            r#"{ "key1": "Hello, World!", "key2": [123, true, ["I am nested!"]], "key3": { "nested-obj": "Hello up there!"} }"#,
        );
        assert_eq!(
            vec![
                JsonTokenOld::JsonObjBeg,
                JsonTokenOld::JsonKey(String::from("key1")),
                JsonTokenOld::JsonString(String::from("Hello, World!")),
                JsonTokenOld::JsonKey(String::from("key2")),
                JsonTokenOld::JsonArrBeg,
                JsonTokenOld::JsonNum(123),
                JsonTokenOld::JsonBool(true),
                JsonTokenOld::JsonArrBeg,
                JsonTokenOld::JsonString(String::from("I am nested!")),
                JsonTokenOld::JsonArrEnd,
                JsonTokenOld::JsonArrEnd,
                JsonTokenOld::JsonKey(String::from("key3")),
                JsonTokenOld::JsonObjBeg,
                JsonTokenOld::JsonKey(String::from("nested-obj")),
                JsonTokenOld::JsonString(String::from("Hello up there!")),
                JsonTokenOld::JsonObjEnd,
                JsonTokenOld::JsonObjEnd
            ],
            tokenize_json_string(&json_string)
        );
    }

    #[test]
    fn detects_key() {
        let key = JsonTokenOld::JsonKey(String::from("This is a key!"));
        assert_eq!(true, key.is_key());
    }

    #[test]
    fn converts_tokens_to_hashmap_containers_following_each_other() {
        let json_token_vec: Vec<JsonTokenOld> = vec![
            JsonTokenOld::JsonObjBeg,
            JsonTokenOld::JsonKey(String::from("key1")),
            JsonTokenOld::JsonString(String::from("string")),
            JsonTokenOld::JsonKey(String::from("key2")),
            JsonTokenOld::JsonBool(true),
            JsonTokenOld::JsonKey(String::from("key3")),
            JsonTokenOld::JsonArrBeg,
            JsonTokenOld::JsonString(String::from("thing1")),
            JsonTokenOld::JsonString(String::from("thing2")),
            JsonTokenOld::JsonArrEnd,
            JsonTokenOld::JsonKey(String::from("key4")),
            JsonTokenOld::JsonObjBeg,
            JsonTokenOld::JsonKey(String::from("key1")),
            JsonTokenOld::JsonString(String::from("full test")),
            JsonTokenOld::JsonObjEnd,
            JsonTokenOld::JsonObjEnd,
        ];

        let mut json_map: HashMap<String, JsonTokenOld> = HashMap::new();
        let mut inner_map: HashMap<String, JsonTokenOld> = HashMap::new();
        inner_map.insert(
            String::from("key1"),
            JsonTokenOld::JsonString(String::from("full test")),
        );
        json_map.insert(
            String::from("key1"),
            JsonTokenOld::JsonString(String::from("string")),
        );
        json_map.insert(String::from("key2"), JsonTokenOld::JsonBool(true));
        json_map.insert(
            String::from("key3"),
            JsonTokenOld::JsonArr(vec![
                JsonTokenOld::JsonString(String::from("thing1")),
                JsonTokenOld::JsonString(String::from("thing2")),
            ]),
        );
        json_map.insert(String::from("key4"), JsonTokenOld::JsonObj(inner_map));

        let created_map = match from_json_tokens_to_json_object(&json_token_vec) {
            JsonTokenOld::JsonObj(map) => map,
            _ => HashMap::new(),
        };

        assert_eq!(json_map, created_map);
    }

    #[test]
    fn converts_not_nested_tokens_to_hashmap() {
        let json_token_vec: Vec<JsonTokenOld> = vec![
            JsonTokenOld::JsonObjBeg,
            JsonTokenOld::JsonKey(String::from("hello")),
            JsonTokenOld::JsonString(String::from("world")),
            JsonTokenOld::JsonKey(String::from("num")),
            JsonTokenOld::JsonNum(123),
            JsonTokenOld::JsonKey(String::from("bool")),
            JsonTokenOld::JsonBool(true),
            JsonTokenOld::JsonObjEnd,
        ];

        let mut json_map: HashMap<String, JsonTokenOld> = HashMap::new();
        json_map.insert(
            String::from("hello"),
            JsonTokenOld::JsonString(String::from("world")),
        );
        json_map.insert(String::from("num"), JsonTokenOld::JsonNum(123));
        json_map.insert(String::from("bool"), JsonTokenOld::JsonBool(true));

        let created_map = match from_json_tokens_to_json_object(&json_token_vec) {
            JsonTokenOld::JsonObj(map) => map,
            _ => HashMap::new(),
        };

        assert_eq!(json_map, created_map);
    }

    #[test]
    fn converts_tokens_w_array_to_hashmap() {
        let json_token_vec: Vec<JsonTokenOld> = vec![
            JsonTokenOld::JsonObjBeg,
            JsonTokenOld::JsonKey(String::from("hello")),
            JsonTokenOld::JsonString(String::from("world")),
            JsonTokenOld::JsonKey(String::from("num")),
            JsonTokenOld::JsonNum(123),
            JsonTokenOld::JsonKey(String::from("bool")),
            JsonTokenOld::JsonBool(true),
            JsonTokenOld::JsonKey(String::from("array")),
            JsonTokenOld::JsonArrBeg,
            JsonTokenOld::JsonNum(123),
            JsonTokenOld::JsonBool(false),
            JsonTokenOld::JsonArrEnd,
            JsonTokenOld::JsonObjEnd,
        ];

        let mut json_map: HashMap<String, JsonTokenOld> = HashMap::new();
        json_map.insert(
            String::from("hello"),
            JsonTokenOld::JsonString(String::from("world")),
        );
        json_map.insert(String::from("num"), JsonTokenOld::JsonNum(123));
        json_map.insert(String::from("bool"), JsonTokenOld::JsonBool(true));
        json_map.insert(
            String::from("array"),
            JsonTokenOld::JsonArr(vec![
                JsonTokenOld::JsonNum(123),
                JsonTokenOld::JsonBool(false),
            ]),
        );

        let created_map = match from_json_tokens_to_json_object(&json_token_vec) {
            JsonTokenOld::JsonObj(map) => map,
            _ => HashMap::new(),
        };

        assert_eq!(json_map, created_map);
    }

    #[test]
    fn converts_tokens_w_nested_array_to_hashmap() {
        let json_token_vec: Vec<JsonTokenOld> = vec![
            JsonTokenOld::JsonObjBeg,
            JsonTokenOld::JsonKey(String::from("hello")),
            JsonTokenOld::JsonString(String::from("world")),
            JsonTokenOld::JsonKey(String::from("num")),
            JsonTokenOld::JsonNum(123),
            JsonTokenOld::JsonKey(String::from("bool")),
            JsonTokenOld::JsonBool(true),
            JsonTokenOld::JsonKey(String::from("array")),
            JsonTokenOld::JsonArrBeg,
            JsonTokenOld::JsonNum(123),
            JsonTokenOld::JsonBool(false),
            JsonTokenOld::JsonArrBeg,
            JsonTokenOld::JsonNum(321),
            JsonTokenOld::JsonBool(false),
            JsonTokenOld::JsonArrEnd,
            JsonTokenOld::JsonArrEnd,
            JsonTokenOld::JsonObjEnd,
        ];

        let mut json_map: HashMap<String, JsonTokenOld> = HashMap::new();
        json_map.insert(
            String::from("hello"),
            JsonTokenOld::JsonString(String::from("world")),
        );
        json_map.insert(String::from("num"), JsonTokenOld::JsonNum(123));
        json_map.insert(String::from("bool"), JsonTokenOld::JsonBool(true));
        json_map.insert(
            String::from("array"),
            JsonTokenOld::JsonArr(vec![
                JsonTokenOld::JsonNum(123),
                JsonTokenOld::JsonBool(false),
                JsonTokenOld::JsonArr(vec![
                    JsonTokenOld::JsonNum(321),
                    JsonTokenOld::JsonBool(false),
                ]),
            ]),
        );

        let created_map = match from_json_tokens_to_json_object(&json_token_vec) {
            JsonTokenOld::JsonObj(map) => map,
            _ => HashMap::new(),
        };

        assert_eq!(json_map, created_map);
    }

    #[test]
    fn converts_tokens_w_nested_object_to_hashmap() {
        let json_token_vec: Vec<JsonTokenOld> = vec![
            JsonTokenOld::JsonObjBeg,
            JsonTokenOld::JsonKey(String::from("hello")),
            JsonTokenOld::JsonString(String::from("world")),
            JsonTokenOld::JsonKey(String::from("num")),
            JsonTokenOld::JsonNum(123),
            JsonTokenOld::JsonKey(String::from("bool")),
            JsonTokenOld::JsonBool(true),
            JsonTokenOld::JsonKey(String::from("obj")),
            JsonTokenOld::JsonObjBeg,
            JsonTokenOld::JsonKey(String::from("key1")),
            JsonTokenOld::JsonString(String::from("nested obj test")),
            JsonTokenOld::JsonObjEnd,
            JsonTokenOld::JsonObjEnd,
        ];

        let mut nested_map: HashMap<String, JsonTokenOld> = HashMap::new();
        nested_map.insert(
            String::from("key1"),
            JsonTokenOld::JsonString(String::from("nested obj test")),
        );

        let mut json_map: HashMap<String, JsonTokenOld> = HashMap::new();
        json_map.insert(
            String::from("hello"),
            JsonTokenOld::JsonString(String::from("world")),
        );
        json_map.insert(String::from("num"), JsonTokenOld::JsonNum(123));
        json_map.insert(String::from("bool"), JsonTokenOld::JsonBool(true));
        json_map.insert(String::from("obj"), JsonTokenOld::JsonObj(nested_map));

        let created_map = match from_json_tokens_to_json_object(&json_token_vec) {
            JsonTokenOld::JsonObj(map) => map,
            _ => HashMap::new(),
        };

        assert_eq!(json_map, created_map);
    }
}

fn testing_new_impl(json_string: &String) -> Vec<Box<dyn JsonToken>> {
    let mut char_inds = json_string.char_indices().peekable();
    let mut tokens: Vec<Box<dyn JsonToken>> = Vec::new();

    while let Some((_pos, ch)) = char_inds.next() {
        match ch {
            // Object parsing
            '{' => {
                tokens.push(Box::new(JsonSeperator::JsonObjBeg));
            }
            '}' => {
                tokens.push(Box::new(JsonSeperator::JsonObjEnd));
            }
            // Array parsing
            '[' => {
                tokens.push(Box::new(JsonSeperator::JsonArrBeg));
            }
            ']' => {
                tokens.push(Box::new(JsonSeperator::JsonArrEnd));
            }
            // String parsing
            '"' => {
                let mut last_matched: char = ch;
                let str_content: String = char_inds
                    .by_ref()
                    .take_while(|(_pos, c)| {
                        if *c != '"' || last_matched == '\\' {
                            last_matched = *c;
                            return true;
                        }
                        false
                    })
                    .map(|(_pos, c)| c)
                    .collect();

                if let Some((_pos, ch)) = char_inds.peek() {
                    if *ch == ':' {
                        tokens.push(Box::new(JsonKey::new(str_content.replace("\\", ""))));
                        continue;
                    } else if *ch == ',' || *ch == '{' || *ch == '}' || *ch == '[' || *ch == ']' {
                        tokens.push(Box::new(JsonValue::JString(str_content.replace("\\", ""))));
                        continue;
                    }
                }

                while let Some((_pos, _ch)) = char_inds.next() {
                    match char_inds.peek() {
                        Some((_pos, c)) => {
                            if *c == ':' {
                                tokens.push(Box::new(JsonKey::new(str_content.replace("\\", ""))));
                                break;
                            } else if *c == ',' || *c == '{' || *c == '}' || *c == '[' || *c == ']'
                            {
                                tokens.push(Box::new(JsonValue::JString(
                                    str_content.replace("\\", ""),
                                )));
                                break;
                            }
                        }
                        None => (),
                    }
                }
            }

            // Number parsing
            c if c.is_numeric() => {
                let mut number: String = String::from(c);
                while let Some((_pos, ch)) = char_inds.next() {
                    number.push(ch);

                    match char_inds.peek() {
                        Some((_pos, c)) => {
                            if !c.is_numeric() {
                                break;
                            }
                        }
                        None => (),
                    }
                }

                tokens.push(Box::new(JsonValue::JsonNum(number.parse::<i64>().unwrap())));
            }
            // Boolean parsing
            c if c.is_alphabetic() => {
                let mut value: String = String::from(c);

                while let Some((_pos, ch)) = char_inds.next() {
                    value.push(ch);

                    match char_inds.peek() {
                        Some((_pos, c)) => {
                            if c.is_ascii_punctuation() || *c == ' ' {
                                break;
                            }
                        }
                        None => (),
                    }
                }

                tokens.push(Box::new(JsonValue::JsonBool(value == "true".to_string())))
            }
            _ => (),
        }
    }

    tokens
}
