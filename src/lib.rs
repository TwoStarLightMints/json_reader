#[derive(Debug, PartialEq, Eq)]
enum JsonToken {
    JsonString(String),
}

pub fn tokenize_json_string() -> Vec<JsonToken> {

}

#[cfg(test)]
mod tests {
    use super::*;
}