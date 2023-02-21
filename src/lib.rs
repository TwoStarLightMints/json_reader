pub mod json_reader {

    use std::collections::HashMap;

    #[derive(Debug, PartialEq, Eq)]
    pub enum JsonToken {
        JsonString(String),
        JsonNum(i64),
        JsonBool(bool),
        JsonArrBeg,
        JsonArrEnd,
        JsonObjBeg,
        JsonObjEnd,
        JsonKey(String),
    }

    pub fn tokenize_json_string(json_string: &String) -> Vec<JsonToken> {
        let mut char_inds = json_string.char_indices().peekable();
        let mut tokens: Vec<JsonToken> = Vec::new();

        while let Some((_pos, ch)) = char_inds.next() {
            match ch {
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
                        .map(|(_pos, c)| { c })
                        .collect();
                    
                    let mut delim: char = ' ';
                    let _ = char_inds
                        .by_ref()
                        .take_while(|(_pos, c)| {
                            println!("ENTERED IN GENERAL");
                            if *c != ':' && *c != ',' && *c != '{' && *c != '}' {
                                return true;
                            } else {
                                delim = *c;
                                return false;
                            }
                        })
                        .map(|(_pos, c)| { c })
                        .collect::<String>();

                    if delim == ':' || last_matched == ':' {
                        tokens.push(JsonToken::JsonKey(str_content.replace("\\", "")));
                    } else {
                        tokens.push(JsonToken::JsonString(str_content.replace("\\", "")));
                    }
                }
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
                // Number parsing
                c if c.is_numeric() => {
                    let mut number: String = String::from(c);
                    while let Some((_pos, ch)) = char_inds.next() {
                        number.push(ch);

                        match char_inds.peek() {
                            Some((_pos, c)) => { if !c.is_numeric() {break;} }
                            None => (),
                        }
                    }

                    tokens.push(JsonToken::JsonNum(number.parse::<i64>().unwrap()));
                }
                // Boolean parsing
                c if c.is_alphabetic() => {
                    let mut value: String = String::from(c);

                    while let Some((_pos, ch)) = char_inds.next() {
                        value.push(ch);

                        match char_inds.peek() {
                            Some((_pos, c)) => { if c.is_ascii_punctuation() || *c == ' ' {break;} }
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

    pub fn from_json_tokens_to_data_struct(json_token_vec: Vec<JsonToken>) -> HashMap<String, JsonToken> {
        let mut token_iter = json_token_vec.iter().peekable();
        
        while let Some(token) = token_iter.next() {
            match token {
                JsonToken::JsonObjBeg => {
                    let mut new_vec: Vec<JsonToken> = Vec::new();
                    token_iter.clone().for_each(|t| {
                        match t {
                            JsonToken::JsonKey(str) => {new_vec.push(JsonToken::JsonKey(str.clone()));}
                            JsonToken::JsonString(str) => {new_vec.push(JsonToken::JsonString(str.clone()));}
                            JsonToken::JsonObjBeg => {new_vec.push(JsonToken::JsonObjBeg);}
                            JsonToken::JsonObjEnd => {new_vec.push(JsonToken::JsonObjEnd);}
                            JsonToken::JsonArrBeg => {new_vec.push(JsonToken::JsonArrBeg);}
                            JsonToken::JsonArrEnd => {new_vec.push(JsonToken::JsonArrEnd);}
                            JsonToken::JsonBool(val) => {new_vec.push(JsonToken::JsonBool(*val));}
                            JsonToken::JsonNum(num) => {new_vec.push(JsonToken::JsonNum(*num));}
                        }
                    });

                }
                _ => (),
            }
        }

        let mut thing: HashMap<String, JsonToken> = HashMap::new();
        thing.insert(String::from("bruh"), JsonToken::JsonArrBeg);
        return thing;
    }

}