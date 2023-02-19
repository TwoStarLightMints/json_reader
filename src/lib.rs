#[derive(Debug, PartialEq, Eq)]
pub enum JsonToken {
    JsonString(String),
}

pub fn tokenize_json_string(json_string: &String) -> Vec<JsonToken> {
    let mut char_inds = json_string.char_indices();
    let mut tokens: Vec<JsonToken> = Vec::new();
    let mut in_string = false;

    while let Some((pos, ch)) = char_inds.next() {
        
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