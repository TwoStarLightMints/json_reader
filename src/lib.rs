pub mod json_reader {

    #[derive(Debug, PartialEq, Eq)]
    pub enum JsonToken {
        JsonString(String),
        JsonNum(i64),
        JsonBool(bool),
        JsonArrBeg,
        JsonArrEnd,
        JsonObjBeg,
        JsonObjEnd,
        JsonKey,
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

                    tokens.push(JsonToken::JsonString(str_content.replace("\\", "")));
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
                ':' => {
                    tokens.push(JsonToken::JsonKey);
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

}

#[cfg(test)]
mod tests {
    use super::json_reader::*;

    #[test]
    fn reads_basic_json_string() {
        let json_string: String = String::from(r#""Hello, World!""#);
        assert_eq!(vec![JsonToken::JsonString(String::from("Hello, World!"))], tokenize_json_string(&json_string));
    }

    #[test]
    fn reads_json_string_w_escape_character() {
        let json_string: String = String::from(r#""Hell\"o, World!""#);
        assert_eq!(vec![JsonToken::JsonString(String::from("Hell\"o, World!"))], tokenize_json_string(&json_string));
    }

    #[test]
    fn reads_basic_json_number() {
        let json_string: String = String::from(r#"123"#);
        assert_eq!(vec![JsonToken::JsonNum(123)], tokenize_json_string(&json_string));
    }

    #[test]
    fn reads_basic_json_number_and_string() {
        let json_string: String = String::from(r#""Hello, World!" 123"#);
        assert_eq!(vec![JsonToken::JsonString(String::from("Hello, World!")), JsonToken::JsonNum(123)], tokenize_json_string(&json_string));
    }

    #[test]
    fn reads_basic_json_bool() {
        let json_string: String = String::from("true");
        assert_eq!(vec![JsonToken::JsonBool(true)], tokenize_json_string(&json_string));
    }

    #[test]
    fn reads_basic_json_obj_w_bool() {
        let json_string: String = String::from("{ true }");
        assert_eq!(vec![JsonToken::JsonObjBeg, JsonToken::JsonBool(true), JsonToken::JsonObjEnd], tokenize_json_string(&json_string));
    }
    #[test]
    fn reads_basic_json_arr_w_bool() {
        let json_string: String = String::from("[ true ]");
        assert_eq!(vec![JsonToken::JsonArrBeg, JsonToken::JsonBool(true), JsonToken::JsonArrEnd], tokenize_json_string(&json_string));
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
            JsonToken::JsonString(String::from("hello")), JsonToken::JsonKey, JsonToken::JsonString(String::from("world")),
            JsonToken::JsonString(String::from("bruh")), JsonToken::JsonKey, JsonToken::JsonBool(true),
            JsonToken::JsonString(String::from("arr")), JsonToken::JsonKey,
                JsonToken::JsonArrBeg,
                    JsonToken::JsonString(String::from("true")), JsonToken::JsonBool(true), JsonToken::JsonNum(123),
                JsonToken::JsonArrEnd,
            JsonToken::JsonString(String::from("obj")), JsonToken::JsonKey,
                JsonToken::JsonObjBeg,
                    JsonToken::JsonString(String::from("hello")),
                    JsonToken::JsonNum(123),
                JsonToken::JsonObjEnd,
            JsonToken::JsonObjEnd], tokenize_json_string(&json_string));
    }

    #[test]
    fn properly_parse_nested_obj() {
        let json_string = String::from(r#"{{"hello"}}"#);
        assert_eq!(vec![JsonToken::JsonObjBeg, JsonToken::JsonObjBeg, JsonToken::JsonString(String::from("hello")), JsonToken::JsonObjEnd, JsonToken::JsonObjEnd], tokenize_json_string(&json_string));
    }
}