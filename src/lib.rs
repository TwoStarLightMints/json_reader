pub mod json_reader {

    use std::collections::HashMap;

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub enum JsonToken {
        JsonKey(String),
        JsonString(String),
        JsonNum(i64),
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
                JsonToken::JsonKey(_) => { return true; }
                _ => { return false; }
            }
        }

        pub fn is_value(&self) -> bool {
            match self {
                JsonToken::JsonKey(_) | JsonToken::JsonArrBeg | JsonToken::JsonArrEnd | JsonToken::JsonObjBeg | JsonToken::JsonObjEnd | JsonToken::JsonInvalid => false,
                _ => true,
            }
        }

        pub fn as_str(&self) -> &str {
            match self {
                JsonToken::JsonString(str) => { str as &str }
                _ => unreachable!(),
            }
        }

        pub fn as_int(&self) -> i64 {
            match self {
                JsonToken::JsonNum(num) => { *num }
                _ => unreachable!(),
            }
        }

        pub fn as_bool(&self) -> bool {
            match self {
                JsonToken::JsonBool(bin) => { *bin }
                _ => unreachable!(),
            }
        }

        pub fn as_map(&self) -> HashMap<String, JsonToken> {
            match self {
                JsonToken::JsonObj(map) => { map.clone() }
                _ => unreachable!(),
            }
        }

        pub fn as_vec(&self) -> Vec<JsonToken> {
            match self {
                JsonToken::JsonArr(vector) => { vector.clone() }
                _ => unreachable!(),
            }
        }
    }

    pub fn tokenize_json_string(json_string: &String) -> Vec<JsonToken> {
        let mut char_inds = json_string.char_indices().peekable();
        let mut tokens: Vec<JsonToken> = Vec::new();

        while let Some((_pos, ch)) = char_inds.next() {
            println!("{}", &ch);
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
                                } else if *c == ',' || *c == '{' || *c == '}' || *c == '[' || *c == ']' {
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



    pub fn from_json_tokens_to_json_array(json_token_vec: &Vec<JsonToken>) -> Result<JsonToken, String> {
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

        if arr_inds.len() % 2 != 0 || obj_inds.len() % 2 != 0 {
            return Err(String::from("Invalid"));
        }

        let mut nesting = 0;

        for i in arr_inds[0]..arr_inds[arr_inds.len()-1] {
            
            if i == 0 || i == arr_inds[arr_inds.len()-1] { continue; }

            if json_token_vec[i].is_value() && nesting <= 0 {
                new_vec.push(json_token_vec[i].clone());
            } else {
                match json_token_vec[i] {
                    JsonToken::JsonArrBeg => {
                        new_vec.push(from_json_tokens_to_json_array(&json_token_vec[arr_inds[1]..=arr_inds[arr_inds.len()-2]].to_vec()).unwrap());
                        nesting += 1;
                    }
                    JsonToken::JsonObjBeg => {
                        new_vec.push(from_json_tokens_to_json_object(&json_token_vec[obj_inds[0]..=obj_inds[obj_inds.len()-1]].to_vec()).unwrap());
                        nesting += 1;
                    }
                    JsonToken::JsonArrEnd => { nesting -= 1; }
                    JsonToken::JsonObjEnd => { nesting -= 1; }
                    _ => (),
                }
            }
        }

        return Ok(JsonToken::JsonArr(new_vec));
    }

    pub fn from_json_tokens_to_json_object(json_token_vec: &Vec<JsonToken>) -> Result<JsonToken, String> {
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

        if arr_inds.len() % 2 != 0 || obj_inds.len() % 2 != 0 {
            return Err(String::from("Invalid"));
        }

        let mut nesting = 0;
            
        for i in obj_inds[0]..obj_inds[obj_inds.len()-1] {
            
            if i == 0 || i == obj_inds[obj_inds.len()-1] { continue; }

            let mut key = String::new();
            if let JsonToken::JsonKey(val) = &json_token_vec[i] { key = val.clone(); }

            if json_token_vec[i].is_key() && nesting <= 0 {
                if json_token_vec[i+1].is_value() {
                    new_map.insert(key.clone(), json_token_vec[i+1].clone());
                } else {
                    match json_token_vec[i+1] {
                        JsonToken::JsonArrBeg => {
                            new_map.insert(key.clone(), from_json_tokens_to_json_array(&json_token_vec[arr_inds[0]..=arr_inds[arr_inds.len()-1]].to_vec()).unwrap());
                            nesting += 1;
                        }
                        JsonToken::JsonObjBeg => {
                            new_map.insert(key.clone(), from_json_tokens_to_json_object(&json_token_vec[obj_inds[1]..=obj_inds[obj_inds.len()-2]].to_vec()).unwrap());
                            nesting += 1;
                        }
                        _ => (),
                    }
                }
            } else if nesting > 0 {
                match json_token_vec[i] {
                    JsonToken::JsonArrEnd => { nesting -= 1; }
                    JsonToken::JsonObjEnd => { nesting -= 1; }
                    _ => (),
                }
            }
        }

        return Ok(JsonToken::JsonObj(new_map));
    }

    pub fn from_json_string(json_string: &String) -> HashMap<String, JsonToken> {
        let token_vec = tokenize_json_string(&json_string);
        if let JsonToken::JsonObj(map) = from_json_tokens_to_json_object(&token_vec).unwrap() { return map; } else { return HashMap::new(); }
    }

}