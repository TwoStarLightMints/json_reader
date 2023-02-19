#[derive(Debug, PartialEq, Eq)]
pub enum JsonToken {
    JsonString(String),
}

pub fn tokenize_json_string(json_string: &String) -> Vec<JsonToken> {
    let mut tokens: Vec<JsonToken> = Vec::new();
    let mut in_string = false;

    for ch in json_string.chars() {
        match ch {
            '"' => {
                if in_string { in_string = false; } else { in_string = true; }
            }
            _ =>  {
                if in_string {
                    let buffer: String = json_string
                        .chars()
                        .take_while(|c| { *c != '"'})
                        .map(|c| { c }).collect();
                    tokens.push(JsonToken::JsonString(String::from(buffer)));
                }
            }
        }
    }

    return tokens;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_basic_json_string() {
        let json_string = String::from(r#""Hello, World!""#);
        assert_eq!(vec![JsonToken::JsonString(String::from("Hello, World!"))], tokenize_json_string(&json_string));
    }
}