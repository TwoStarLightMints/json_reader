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

        while arr_inds.len() > 0 || obj_inds.len() > 0 {

            let mut recorded = true; println!("INSIDE ARRAY PARSER  arrinds{:?} objinds{:?}", arr_inds, obj_inds);

            for i in arr_inds[0]..arr_inds[arr_inds.len()-1] {
                if i == 0 || i == arr_inds[arr_inds.len()-1] { continue; }

                let mut val = json_token_vec[i].clone();
                
                if val == JsonToken::JsonObjBeg || val == JsonToken::JsonArrBeg {
                    if !recorded {
                        if val == JsonToken::JsonObjBeg {
                            val = from_json_tokens_to_json_object(&json_token_vec[obj_inds[0]..obj_inds[obj_inds.len()-1]].to_vec()).unwrap();
                            recorded = true;
                        } else {
                            val = from_json_tokens_to_json_array(&json_token_vec[arr_inds[0]..arr_inds[arr_inds.len()-1]].to_vec()).unwrap();
                            recorded = true;
                        }
                    } else {
                        continue;
                    }
                }

                if val == JsonToken::JsonObjEnd && recorded {
                    if i == obj_inds[obj_inds.len()-2] {
                        recorded = false;
                    }
                } else if val == JsonToken::JsonArrEnd && recorded {
                    if i == arr_inds[arr_inds.len()-2] {
                        recorded = false;
                    }
                }

                new_vec.push(val);
            }

            if obj_inds.len() > 0 { obj_inds.remove(0); obj_inds.remove(obj_inds.len()-1); }
            if arr_inds.len() > 0 { arr_inds.remove(0); arr_inds.remove(arr_inds.len()-1); }
        }

        println!("RETURNED VECTOR: {:?}", &new_vec);
        return Ok(JsonToken::JsonArr(new_vec));
    }

//-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------//
//-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------//
//-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------//
//-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------//

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

        while arr_inds.len() > 0 || obj_inds.len() > 0 {

            let mut recorded = false;
            let mut is_obj = false;
            let mut is_arr = false;
            
            for i in obj_inds[0]..obj_inds[obj_inds.len()-1] {
                println!("CURRENT INDEX: {}", i);
                if i == 0 || i == obj_inds[obj_inds.len()-1] { continue; }

                let mut val = json_token_vec[i].clone();
                
                println!("arr {:?} : obj {:?}", arr_inds, obj_inds);
                if !recorded {
                    if val == JsonToken::JsonObjBeg && obj_inds.len() > 0 {
                        val = from_json_tokens_to_json_object(&json_token_vec[obj_inds[1]..=obj_inds[obj_inds.len()-2]].to_vec()).unwrap();
                        recorded = true;
                        is_obj = true;
                    } else if val == JsonToken::JsonArrBeg && arr_inds.len() > 0 {
                        println!("FOUND ARR {:?}", &json_token_vec[arr_inds[0]..=arr_inds[arr_inds.len()-1]]);
                        val = from_json_tokens_to_json_array(&json_token_vec[arr_inds[0]..=arr_inds[arr_inds.len()-1]].to_vec()).unwrap();
                        recorded = true;
                        is_arr = true;
                    }
                } else {
                    continue;
                }

                if val == JsonToken::JsonObjEnd && recorded {
                    if i == obj_inds[obj_inds.len()-2] {
                        recorded = false;
                    }
                } else if val == JsonToken::JsonArrEnd && recorded {
                    if i == arr_inds[arr_inds.len()-2] {
                        recorded = false; println!("FOUND END ARR");
                    }
                }

                let mut key = String::new();
                if let JsonToken::JsonKey(val) = &json_token_vec[i] { key = val.clone(); }

                println!("val {:?}", &val);
                if json_token_vec[i].is_key() { new_map.insert(key.clone(), json_token_vec[i+1].clone()); } else { new_map.insert(key.clone(), val); }
            }

            println!("obj len {}, is obj {}", obj_inds.len(), is_obj);

            if arr_inds.len() > 0 && is_arr { arr_inds.remove(0); arr_inds.remove(arr_inds.len()-1); println!("Removed array"); }
            if obj_inds.len() > 0 && is_obj { obj_inds.remove(0); obj_inds.remove(obj_inds.len()-1); println!("Removed object"); }
        }

        return Ok(JsonToken::JsonObj(new_map));
    }

}