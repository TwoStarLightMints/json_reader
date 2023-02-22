pub mod json_reader {

    use std::collections::HashMap;

    #[derive(Debug, PartialEq, Eq)]
    pub enum JsonToken {
        JsonKey(String),
        JsonString(String),
        JsonNum(i64),
        JsonBool(bool),
        JsonArr(Vec<JsonToken>),
        JsonArrBeg,
        JsonArrEnd,
        JsonObj(HashMap<String, JsonToken>),
        JsonObjBeg,
        JsonObjEnd,
        JsonInvalid,
    }

    pub struct JsonMap {
        map: HashMap<String, JsonMap>,
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
                    
                    if let Some((_pos, ch)) = char_inds.peek() {
                        if *ch == ':' {
                            tokens.push(JsonToken::JsonKey(str_content.replace("\\", "")));
                            continue;
                        } else if *ch == ',' || *ch == '{' || *ch == '}' {
                            tokens.push(JsonToken::JsonString(str_content.replace("\\", "")));
                            continue;
                        }
                    }

                    while let Some((_pos, _ch)) = char_inds.next() {
                        match char_inds.peek() {
                                Some((_pos, c)) => { if *c == ':' {
                                    tokens.push(JsonToken::JsonKey(str_content.replace("\\", "")));
                                    break;
                                } else if *c == ',' || *c == '{' || *c == '}' {
                                    tokens.push(JsonToken::JsonString(str_content.replace("\\", "")));
                                    break;
                                }
                            }
                            None => (),
                        }
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

    pub fn build_json_array(json_token_vec: Vec<JsonToken>) -> Vec<JsonToken> {
        let mut token_iter = json_token_vec.iter();
        let mut cleaned_vec: Vec<JsonToken> = Vec::new();

        while let Some(item) = token_iter.next() {
            match item {
                JsonToken::JsonArrBeg => {
                    let new_arr: Vec<JsonToken> = token_iter
                        .by_ref()
                        .take_while(|i| **i != JsonToken::JsonArrEnd)
                        .map(|i| {
                            match i {
                                JsonToken::JsonKey(key) => { JsonToken::JsonKey(key.clone()) }
                                JsonToken::JsonString(str) => { JsonToken::JsonString(str.clone()) }
                                JsonToken::JsonNum(num) => { JsonToken::JsonNum(*num) }
                                JsonToken::JsonBool(bin) => { JsonToken::JsonBool(*bin) }
                                JsonToken::JsonObjBeg => { JsonToken::JsonObjBeg }
                                JsonToken::JsonObjEnd => { JsonToken::JsonObjEnd }
                                _ => { JsonToken::JsonInvalid }
                            }
                        })
                        .collect();

                    cleaned_vec.push(JsonToken::JsonArr(new_arr));
                }
                JsonToken::JsonKey(key) => { cleaned_vec.push(JsonToken::JsonKey(key.clone())); }
                JsonToken::JsonString(str) => { cleaned_vec.push(JsonToken::JsonString(str.clone())); }
                JsonToken::JsonNum(num) => { cleaned_vec.push(JsonToken::JsonNum(*num)); }
                JsonToken::JsonBool(bin) => { cleaned_vec.push(JsonToken::JsonBool(*bin)); }
                JsonToken::JsonObjBeg => { cleaned_vec.push(JsonToken::JsonObjBeg); }
                JsonToken::JsonObjEnd => { cleaned_vec.push(JsonToken::JsonObjEnd); }
                _ => ()
            }
        }

        return cleaned_vec;
    }

    pub fn from_json_tokens_to_data_struct(json_token_vec: Vec<JsonToken>) -> HashMap<String, JsonToken> {
        let mut token_iter = json_token_vec.iter();
        
        // while let Some(item) = token_iter.next() {
        //     match item {
        //         JsonToken::JsonObjBeg => {
        //             // let obj: HashMap<String, JsonToken> = HashMap::new();

        //             // from_json_tokens_to_data_struct(
        //             //     token_iter
        //             //         .by_ref()
        //             //         .map(|t| {

        //             //             if let JsonToken::JsonKey(key) = t { return JsonToken::JsonKey(key.clone()); }
        //             //             if let JsonToken::JsonString(str) = t { return JsonToken::JsonString(str.clone()); }
        //             //             if let JsonToken::JsonNum(num) = t { return JsonToken::JsonNum(*num); }
        //             //             if let JsonToken::JsonBool(bin) = t { return JsonToken::JsonBool(*bin); }
        //             //             if let JsonToken::JsonArrBeg = t { return JsonToken::JsonArrBeg; }
        //             //             if let JsonToken::JsonArrEnd = t { return JsonToken::JsonArrEnd; }
        //             //             if let JsonToken::JsonObjBeg = t { return JsonToken::JsonObjBeg; }
        //             //             if let JsonToken::JsonObjEnd = t { return JsonToken::JsonObjEnd; }
        //             //             else { return JsonToken::JsonInvalid; }
        //             //         })
        //             //         .collect::<Vec<JsonToken>>()
        //             // );
        //         }
        //         JsonToken::JsonArrBeg => {}
        //         _ => {}
        //     }
        // }

        // ------------------------------------------------------------------------------------------------------------------------------------------------------------------
        let mut thing: HashMap<String, JsonToken> = HashMap::new();
        thing.insert(String::from("bruh"), JsonToken::JsonArrBeg);
        return thing;
    }

}