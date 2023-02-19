// TODO: Rework number parsing, it's stupid right now
#[derive(Debug, PartialEq, Eq)]
pub enum JsonToken {
    JsonString(String),
    JsonNum(i64),
    JsonBool(bool),
}

pub fn tokenize_json_string(json_string: &String) -> Vec<JsonToken> {
    let mut char_inds = json_string.char_indices();
    let mut tokens: Vec<JsonToken> = Vec::new();

    while let Some((_pos, ch)) = char_inds.next() {
        match ch {
            // String parsing
            '"' => {
                let str_content: String = char_inds
                    .by_ref()
                    .take_while(|(_pos, c)| *c != '"')
                    .map(|(_pos, c)| { c })
                    .collect();

                tokens.push(JsonToken::JsonString(str_content));
            }
            // Number parsing
            c if c.is_numeric() => {
                let mut first_digit = String::from(c);
                let num_content: String = char_inds
                    .by_ref()
                    .take_while(|(_pos, n)| { n.is_numeric() })
                    .map(|(_pos, n)| { n })
                    .collect();

                first_digit.push_str(num_content.as_str());

                tokens.push(JsonToken::JsonNum(first_digit.parse::<i64>().unwrap()));
            }
            // Boolean parsing
            c if c.is_alphabetic() => {
                let bool_content: String = char_inds
                    .by_ref()
                    .take_while(|(_pos, c)| { *c != ',' || *c != ' ' })
                    .map(|(_pos, c)| { c })
                    .collect();

                if bool_content == String::from("true") {
                    tokens.push(JsonToken::JsonBool(true));
                }
                if bool_content == String::from("false") {
                    tokens.push(JsonToken::JsonBool(false));
                }
            }
            _ => (),
        }
    }

    return tokens;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_basic_json_string() {
        let json_string: String = String::from(r#""Hello, World!""#);
        assert_eq!(vec![JsonToken::JsonString(String::from("Hello, World!"))], tokenize_json_string(&json_string));
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
}