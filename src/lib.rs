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

#[derive(Debug, PartialEq, Clone)]
pub enum JsonToken {
    JsonKey(String),
    JsonString(String),
    JsonNum(f64),
    JsonBool(bool),
    JsonObj(HashMap<String, JsonToken>),
    JsonArr(Vec<JsonToken>),
    JsonArrBeg,
    JsonArrEnd,
    JsonObjBeg,
    JsonObjEnd,
    JsonInvalid,
}

impl JsonToken {
    pub fn is_key(&self) -> bool {
        match self {
            JsonToken::JsonKey(_) => {
                return true;
            }
            _ => {
                return false;
            }
        }
    }

    pub fn is_value(&self) -> bool {
        match self {
            JsonToken::JsonKey(_)
            | JsonToken::JsonArrBeg
            | JsonToken::JsonArrEnd
            | JsonToken::JsonObjBeg
            | JsonToken::JsonObjEnd
            | JsonToken::JsonInvalid => false,
            _ => true,
        }
    }

    pub fn as_str(&self) -> Result<&str, InvalidJsonUnwrap> {
        match self {
            JsonToken::JsonString(str) => Ok(str as &str),
            _ => Err(InvalidJsonUnwrap),
        }
    }

    pub fn as_f64(&self) -> Result<f64, InvalidJsonUnwrap> {
        match self {
            JsonToken::JsonNum(num) => Ok(*num),
            _ => Err(InvalidJsonUnwrap),
        }
    }

    pub fn as_bool(&self) -> bool {
        match self {
            JsonToken::JsonBool(bin) => *bin,
            _ => unreachable!(),
        }
    }

    pub fn as_map(&self) -> Result<HashMap<String, JsonToken>, InvalidJsonUnwrap> {
        match self {
            JsonToken::JsonObj(map) => Ok(map.clone()),
            _ => Err(InvalidJsonUnwrap),
        }
    }

    pub fn as_vec(&self) -> Result<Vec<JsonToken>, InvalidJsonUnwrap> {
        match self {
            JsonToken::JsonArr(vector) => Ok(vector.clone()),
            _ => Err(InvalidJsonUnwrap),
        }
    }
}

fn tokenize_json_string(json_string: &String) -> Vec<JsonToken> {
    let mut char_inds = json_string.char_indices().peekable();
    let mut tokens: Vec<JsonToken> = Vec::new();

    while let Some((_pos, ch)) = char_inds.next() {
        match ch {
            // Object parsing
            '{' => {
                tokens.push(JsonToken::JsonObjBeg);
            }
            '}' => {
                tokens.push(JsonToken::JsonObjEnd);
            }
            // Array parsing
            '[' => {
                tokens.push(JsonToken::JsonArrBeg);
            }
            ']' => {
                tokens.push(JsonToken::JsonArrEnd);
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
                        tokens.push(JsonToken::JsonKey(str_content.replace("\\", "")));
                        continue;
                    } else if *ch == ',' || *ch == '{' || *ch == '}' || *ch == '[' || *ch == ']' {
                        tokens.push(JsonToken::JsonString(str_content.replace("\\", "")));
                        continue;
                    }
                }

                while let Some((_pos, _ch)) = char_inds.next() {
                    match char_inds.peek() {
                        Some((_pos, c)) => {
                            if *c == ':' {
                                tokens.push(JsonToken::JsonKey(str_content.replace("\\", "")));
                                break;
                            } else if *c == ',' || *c == '{' || *c == '}' || *c == '[' || *c == ']'
                            {
                                tokens.push(JsonToken::JsonString(str_content.replace("\\", "")));
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
                    if ch.is_numeric() || ch == '.' {
                        number.push(ch);
                    }

                    match char_inds.peek() {
                        Some((_pos, c)) => {
                            if !c.is_numeric() {
                                break;
                            }
                        }
                        None => (),
                    }
                }

                tokens.push(JsonToken::JsonNum(number.parse::<f64>().unwrap()));
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
                    tokens.push(JsonToken::JsonBool(true));
                }
                if value == falth {
                    tokens.push(JsonToken::JsonBool(false));
                }
            }
            _ => (),
        }
    }

    return tokens;
}

fn check_is_valid_json(json_token_vec: &Vec<JsonToken>) -> bool {
    let mut num_arr_tokens: usize = 0;
    let mut num_obj_tokens: usize = 0;
    let mut num_vals: usize = 0;
    let mut num_keys: usize = 0;
    let iter = json_token_vec.iter();

    for token in iter {
        match token {
            JsonToken::JsonKey(_) => {
                num_keys += 1;
            }
            JsonToken::JsonString(_) => {
                num_vals += 1;
            }
            JsonToken::JsonNum(_) => {
                num_vals += 1;
            }
            JsonToken::JsonBool(_) => {
                num_vals += 1;
            }
            JsonToken::JsonArrBeg => {
                num_arr_tokens += 1;
            }
            JsonToken::JsonArrEnd => {
                num_arr_tokens += 1;
            }
            JsonToken::JsonObjBeg => {
                num_obj_tokens += 1;
            }
            JsonToken::JsonObjEnd => {
                num_obj_tokens += 1;
            }
            JsonToken::JsonInvalid => {
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

fn from_tokens_to_datastructure(json_tokens: &Vec<JsonToken>) -> JsonToken {
    let start_ind = json_tokens
        .iter()
        .rposition(|e| *e == JsonToken::JsonArrBeg || *e == JsonToken::JsonObjBeg)
        .unwrap();

    if json_tokens[start_ind] == JsonToken::JsonArrBeg {
        let j_vec: Vec<JsonToken> = json_tokens
            .iter()
            .skip(start_ind + 1)
            .take_while(|e| **e != JsonToken::JsonArrEnd)
            .map(|e| e.clone())
            .collect();

        JsonToken::JsonArr(j_vec)
    } else {
        let j_vec: Vec<JsonToken> = json_tokens
            .iter()
            .skip(start_ind + 1)
            .take_while(|e| **e != JsonToken::JsonObjEnd)
            .map(|e| e.clone())
            .collect();

        let mut j_iter = j_vec.iter();
        let mut j_map: HashMap<String, JsonToken> = HashMap::new();

        while let Some(item) = j_iter.next() {
            let value = j_iter.next().unwrap();

            match item {
                JsonToken::JsonKey(key) => {
                    j_map.insert(key.clone(), value.clone());
                }
                _ => (),
            }
        }

        JsonToken::JsonObj(j_map)
    }
}

fn flatten_json_tokens(mut json_tokens: Vec<JsonToken>) -> HashMap<String, JsonToken> {
    while let Some(start) = json_tokens
        .iter()
        .rposition(|e| *e == JsonToken::JsonArrBeg || *e == JsonToken::JsonObjBeg)
    {
        let end = json_tokens
            .iter()
            .skip(start)
            .position(|e| *e == JsonToken::JsonArrEnd || *e == JsonToken::JsonObjEnd)
            .unwrap();

        let new_token = from_tokens_to_datastructure(&json_tokens);

        // Here, start + end is used because, using skip with the iterator above causes
        // the result of position to be in relation to the skip start. So, if one had a vector
        // [1, 2, 3, 4, 5, 6, 7, 8, 9] and used vec.iter().skip(1) and used position() to find
        // 9, the result would be 7, instead of 8.
        json_tokens.drain(start..=(start + end));
        json_tokens.insert(start, new_token);
    }

    json_tokens[0].as_map().unwrap()
}

pub fn from_json_string(json_string: &String) -> Result<HashMap<String, JsonToken>, InvalidJson> {
    let token_vec = tokenize_json_string(&json_string);

    if !check_is_valid_json(&token_vec) {
        return Err(InvalidJson);
    }

    Ok(flatten_json_tokens(token_vec))
}

fn json_tokens_to_json_array(json_token_vec: &Vec<JsonToken>) -> JsonToken {
    let mut new_vec: Vec<JsonToken> = Vec::new();

    let mut arr_inds: Vec<usize> = Vec::new();
    let mut obj_inds: Vec<usize> = Vec::new();

    json_token_vec.iter().enumerate().for_each(|(ind, val)| {
        if *val == JsonToken::JsonArrBeg || *val == JsonToken::JsonArrEnd {
            arr_inds.push(ind);
        } else if *val == JsonToken::JsonObjBeg || *val == JsonToken::JsonObjEnd {
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
                JsonToken::JsonArrBeg => {
                    new_vec.push(json_tokens_to_json_array(
                        &json_token_vec[arr_inds[1]..=arr_inds[arr_inds.len() - 2]].to_vec(),
                    ));
                    nesting += 1;
                }
                JsonToken::JsonObjBeg => {
                    new_vec.push(json_tokens_to_json_object(
                        &json_token_vec[obj_inds[0]..=obj_inds[obj_inds.len() - 1]].to_vec(),
                    ));
                    nesting += 1;
                }
                JsonToken::JsonArrEnd => {
                    nesting -= 1;
                }
                JsonToken::JsonObjEnd => {
                    nesting -= 1;
                }
                _ => (),
            }
        }
    }

    return JsonToken::JsonArr(new_vec);
}

pub fn json_tokens_to_json_object(json_token_vec: &Vec<JsonToken>) -> JsonToken {
    let mut new_map: HashMap<String, JsonToken> = HashMap::new();

    let mut arr_inds: Vec<usize> = Vec::new();
    let mut obj_inds: Vec<usize> = Vec::new();

    json_token_vec.iter().enumerate().for_each(|(ind, val)| {
        if *val == JsonToken::JsonArrBeg || *val == JsonToken::JsonArrEnd {
            arr_inds.push(ind);
        } else if *val == JsonToken::JsonObjBeg || *val == JsonToken::JsonObjEnd {
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
        if let JsonToken::JsonKey(val) = &json_token_vec[i] {
            key = val.clone();
        }

        if json_token_vec[i].is_key() && nesting <= 0 {
            if json_token_vec[i + 1].is_value() {
                new_map.insert(key.clone(), json_token_vec[i + 1].clone());
            } else {
                match json_token_vec[i + 1] {
                    JsonToken::JsonArrBeg => {
                        new_map.insert(
                            key.clone(),
                            json_tokens_to_json_array(
                                &json_token_vec[arr_inds[0]..=arr_inds[arr_inds.len() - 1]]
                                    .to_vec(),
                            ),
                        );
                        nesting += 1;
                    }
                    JsonToken::JsonObjBeg => {
                        new_map.insert(
                            key.clone(),
                            json_tokens_to_json_object(
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
                JsonToken::JsonArrEnd => {
                    nesting -= 1;
                }
                JsonToken::JsonObjEnd => {
                    nesting -= 1;
                }
                _ => (),
            }
        }
    }

    return JsonToken::JsonObj(new_map);
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
                JsonToken::JsonObjBeg,
                JsonToken::JsonKey(String::from("string")),
                JsonToken::JsonString(String::from("Hello, World!")),
                JsonToken::JsonObjEnd
            ],
            tokenize_json_string(&json_string)
        );
    }

    #[test]
    fn reads_json_string_w_escape_character() {
        let json_string: String = String::from(r#"{"string": "Hell\"o, World!"}"#);
        assert_eq!(
            vec![
                JsonToken::JsonObjBeg,
                JsonToken::JsonKey(String::from("string")),
                JsonToken::JsonString(String::from("Hell\"o, World!")),
                JsonToken::JsonObjEnd
            ],
            tokenize_json_string(&json_string)
        );
    }

    #[test]
    fn reads_basic_json_number() {
        let json_string: String = String::from(r#"{"number": 123}"#);
        assert_eq!(
            vec![
                JsonToken::JsonObjBeg,
                JsonToken::JsonKey(String::from("number")),
                JsonToken::JsonNum(123.0),
                JsonToken::JsonObjEnd
            ],
            tokenize_json_string(&json_string)
        );
    }

    #[test]
    fn reads_basic_json_bool() {
        let json_string: String = String::from(r#"{"boolean": true}"#);
        assert_eq!(
            vec![
                JsonToken::JsonObjBeg,
                JsonToken::JsonKey(String::from("boolean")),
                JsonToken::JsonBool(true),
                JsonToken::JsonObjEnd
            ],
            tokenize_json_string(&json_string)
        );
    }

    #[test]
    fn reads_basic_json_arr_w_bool() {
        let json_string: String = String::from(r#"{"key":[ true ]}"#);
        assert_eq!(
            vec![
                JsonToken::JsonObjBeg,
                JsonToken::JsonKey(String::from("key")),
                JsonToken::JsonArrBeg,
                JsonToken::JsonBool(true),
                JsonToken::JsonArrEnd,
                JsonToken::JsonObjEnd
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
                JsonToken::JsonObjBeg,
                JsonToken::JsonKey(String::from("hello")),
                JsonToken::JsonString(String::from("world")),
                JsonToken::JsonKey(String::from("bruh")),
                JsonToken::JsonBool(true),
                JsonToken::JsonKey(String::from("arr")),
                JsonToken::JsonArrBeg,
                JsonToken::JsonString(String::from("true")),
                JsonToken::JsonBool(true),
                JsonToken::JsonNum(123.0),
                JsonToken::JsonArrEnd,
                JsonToken::JsonKey(String::from("obj")),
                JsonToken::JsonObjBeg,
                JsonToken::JsonString(String::from("hello")),
                JsonToken::JsonNum(123.0),
                JsonToken::JsonObjEnd,
                JsonToken::JsonObjEnd
            ],
            tokenize_json_string(&json_string)
        );
    }

    #[test]
    fn properly_parse_nested_obj() {
        let json_string = String::from(r#"{"object": {"hello": "world"}}"#);
        assert_eq!(
            vec![
                JsonToken::JsonObjBeg,
                JsonToken::JsonKey(String::from("object")),
                JsonToken::JsonObjBeg,
                JsonToken::JsonKey(String::from("hello")),
                JsonToken::JsonString(String::from("world")),
                JsonToken::JsonObjEnd,
                JsonToken::JsonObjEnd
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
                JsonToken::JsonObjBeg,
                JsonToken::JsonKey(String::from("nested-array")),
                JsonToken::JsonString(String::from("does it work?")),
                JsonToken::JsonKey(String::from("arr")),
                JsonToken::JsonArrBeg,
                JsonToken::JsonNum(123.0),
                JsonToken::JsonArrBeg,
                JsonToken::JsonNum(321.0),
                JsonToken::JsonBool(true),
                JsonToken::JsonArrEnd,
                JsonToken::JsonArrEnd,
                JsonToken::JsonObjEnd
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
                JsonToken::JsonObjBeg,
                JsonToken::JsonKey(String::from("key1")),
                JsonToken::JsonString(String::from("Hello, World!")),
                JsonToken::JsonKey(String::from("key2")),
                JsonToken::JsonArrBeg,
                JsonToken::JsonNum(123.0),
                JsonToken::JsonBool(true),
                JsonToken::JsonArrBeg,
                JsonToken::JsonString(String::from("I am nested!")),
                JsonToken::JsonArrEnd,
                JsonToken::JsonArrEnd,
                JsonToken::JsonKey(String::from("key3")),
                JsonToken::JsonObjBeg,
                JsonToken::JsonKey(String::from("nested-obj")),
                JsonToken::JsonString(String::from("Hello up there!")),
                JsonToken::JsonObjEnd,
                JsonToken::JsonObjEnd
            ],
            tokenize_json_string(&json_string)
        );
    }

    #[test]
    fn detects_key() {
        let key = JsonToken::JsonKey(String::from("This is a key!"));
        assert_eq!(true, key.is_key());
    }

    #[test]
    fn converts_tokens_to_hashmap_containers_following_each_other() {
        let json_token_vec: Vec<JsonToken> = vec![
            JsonToken::JsonObjBeg,
            JsonToken::JsonKey(String::from("key1")),
            JsonToken::JsonString(String::from("string")),
            JsonToken::JsonKey(String::from("key2")),
            JsonToken::JsonBool(true),
            JsonToken::JsonKey(String::from("key3")),
            JsonToken::JsonArrBeg,
            JsonToken::JsonString(String::from("thing1")),
            JsonToken::JsonString(String::from("thing2")),
            JsonToken::JsonArrEnd,
            JsonToken::JsonKey(String::from("key4")),
            JsonToken::JsonObjBeg,
            JsonToken::JsonKey(String::from("key1")),
            JsonToken::JsonString(String::from("full test")),
            JsonToken::JsonObjEnd,
            JsonToken::JsonObjEnd,
        ];

        let mut json_map: HashMap<String, JsonToken> = HashMap::new();
        let mut inner_map: HashMap<String, JsonToken> = HashMap::new();
        inner_map.insert(
            String::from("key1"),
            JsonToken::JsonString(String::from("full test")),
        );
        json_map.insert(
            String::from("key1"),
            JsonToken::JsonString(String::from("string")),
        );
        json_map.insert(String::from("key2"), JsonToken::JsonBool(true));
        json_map.insert(
            String::from("key3"),
            JsonToken::JsonArr(vec![
                JsonToken::JsonString(String::from("thing1")),
                JsonToken::JsonString(String::from("thing2")),
            ]),
        );
        json_map.insert(String::from("key4"), JsonToken::JsonObj(inner_map));

        let created_map = match json_tokens_to_json_object(&json_token_vec) {
            JsonToken::JsonObj(map) => map,
            _ => HashMap::new(),
        };

        assert_eq!(json_map, created_map);
    }

    #[test]
    fn converts_not_nested_tokens_to_hashmap() {
        let json_token_vec: Vec<JsonToken> = vec![
            JsonToken::JsonObjBeg,
            JsonToken::JsonKey(String::from("hello")),
            JsonToken::JsonString(String::from("world")),
            JsonToken::JsonKey(String::from("num")),
            JsonToken::JsonNum(123.0),
            JsonToken::JsonKey(String::from("bool")),
            JsonToken::JsonBool(true),
            JsonToken::JsonObjEnd,
        ];

        let mut json_map: HashMap<String, JsonToken> = HashMap::new();
        json_map.insert(
            String::from("hello"),
            JsonToken::JsonString(String::from("world")),
        );
        json_map.insert(String::from("num"), JsonToken::JsonNum(123.0));
        json_map.insert(String::from("bool"), JsonToken::JsonBool(true));

        let created_map = match json_tokens_to_json_object(&json_token_vec) {
            JsonToken::JsonObj(map) => map,
            _ => HashMap::new(),
        };

        assert_eq!(json_map, created_map);
    }

    #[test]
    fn converts_tokens_w_array_to_hashmap() {
        let json_token_vec: Vec<JsonToken> = vec![
            JsonToken::JsonObjBeg,
            JsonToken::JsonKey(String::from("hello")),
            JsonToken::JsonString(String::from("world")),
            JsonToken::JsonKey(String::from("num")),
            JsonToken::JsonNum(123.0),
            JsonToken::JsonKey(String::from("bool")),
            JsonToken::JsonBool(true),
            JsonToken::JsonKey(String::from("array")),
            JsonToken::JsonArrBeg,
            JsonToken::JsonNum(123.0),
            JsonToken::JsonBool(false),
            JsonToken::JsonArrEnd,
            JsonToken::JsonObjEnd,
        ];

        let mut json_map: HashMap<String, JsonToken> = HashMap::new();
        json_map.insert(
            String::from("hello"),
            JsonToken::JsonString(String::from("world")),
        );
        json_map.insert(String::from("num"), JsonToken::JsonNum(123.0));
        json_map.insert(String::from("bool"), JsonToken::JsonBool(true));
        json_map.insert(
            String::from("array"),
            JsonToken::JsonArr(vec![JsonToken::JsonNum(123.0), JsonToken::JsonBool(false)]),
        );

        let created_map = match json_tokens_to_json_object(&json_token_vec) {
            JsonToken::JsonObj(map) => map,
            _ => HashMap::new(),
        };

        assert_eq!(json_map, created_map);
    }

    #[test]
    fn converts_tokens_w_nested_array_to_hashmap() {
        let json_token_vec: Vec<JsonToken> = vec![
            JsonToken::JsonObjBeg,
            JsonToken::JsonKey(String::from("hello")),
            JsonToken::JsonString(String::from("world")),
            JsonToken::JsonKey(String::from("num")),
            JsonToken::JsonNum(123.0),
            JsonToken::JsonKey(String::from("bool")),
            JsonToken::JsonBool(true),
            JsonToken::JsonKey(String::from("array")),
            JsonToken::JsonArrBeg,
            JsonToken::JsonNum(123.0),
            JsonToken::JsonBool(false),
            JsonToken::JsonArrBeg,
            JsonToken::JsonNum(321.0),
            JsonToken::JsonBool(false),
            JsonToken::JsonArrEnd,
            JsonToken::JsonArrEnd,
            JsonToken::JsonObjEnd,
        ];

        let mut json_map: HashMap<String, JsonToken> = HashMap::new();
        json_map.insert(
            String::from("hello"),
            JsonToken::JsonString(String::from("world")),
        );
        json_map.insert(String::from("num"), JsonToken::JsonNum(123.0));
        json_map.insert(String::from("bool"), JsonToken::JsonBool(true));
        json_map.insert(
            String::from("array"),
            JsonToken::JsonArr(vec![
                JsonToken::JsonNum(123.0),
                JsonToken::JsonBool(false),
                JsonToken::JsonArr(vec![JsonToken::JsonNum(321.0), JsonToken::JsonBool(false)]),
            ]),
        );

        let created_map = match json_tokens_to_json_object(&json_token_vec) {
            JsonToken::JsonObj(map) => map,
            _ => HashMap::new(),
        };

        assert_eq!(json_map, created_map);
    }

    #[test]
    fn converts_tokens_w_nested_object_to_hashmap() {
        let json_token_vec: Vec<JsonToken> = vec![
            JsonToken::JsonObjBeg,
            JsonToken::JsonKey(String::from("hello")),
            JsonToken::JsonString(String::from("world")),
            JsonToken::JsonKey(String::from("num")),
            JsonToken::JsonNum(123.0),
            JsonToken::JsonKey(String::from("bool")),
            JsonToken::JsonBool(true),
            JsonToken::JsonKey(String::from("obj")),
            JsonToken::JsonObjBeg,
            JsonToken::JsonKey(String::from("key1")),
            JsonToken::JsonString(String::from("nested obj test")),
            JsonToken::JsonObjEnd,
            JsonToken::JsonObjEnd,
        ];

        let mut nested_map: HashMap<String, JsonToken> = HashMap::new();
        nested_map.insert(
            String::from("key1"),
            JsonToken::JsonString(String::from("nested obj test")),
        );

        let mut json_map: HashMap<String, JsonToken> = HashMap::new();
        json_map.insert(
            String::from("hello"),
            JsonToken::JsonString(String::from("world")),
        );
        json_map.insert(String::from("num"), JsonToken::JsonNum(123.0));
        json_map.insert(String::from("bool"), JsonToken::JsonBool(true));
        json_map.insert(String::from("obj"), JsonToken::JsonObj(nested_map));

        let created_map = match json_tokens_to_json_object(&json_token_vec) {
            JsonToken::JsonObj(map) => map,
            _ => HashMap::new(),
        };

        assert_eq!(json_map, created_map);
    }
}
