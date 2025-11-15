use regex::Regex;

struct Reader {
    input: String,
    position: usize,
    tokens: Vec<String>,
}

impl Reader {
    fn new(input: String) -> Self {
        Self {
            input: input,
            position: 0,
            tokens: tokenize(&input),
        }
    }
    fn peek(&self) -> Option<String> {
        if self.position >= self.tokens.len() {
            return None;
        }
        Some(self.tokens[self.position].clone())
    }
    fn next(&mut self) -> String {
        if self.position < self.tokens.len() {
            let output = self.tokens[self.position].clone();
            self.position = self.position+1;
            output
        } else {"".to_string()}

    }
}



fn tokenize(input: &str) -> Vec<String> {
    let tokens_regex = Regex::new(r#"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"#).unwrap();
    let tokens_discard = Regex::new(r#"[\s,]*"#);

    let mut slice = &input[..];
    let mut output = Vec::new();

    while slice.len()>0 {
        let find_token = tokens_regex.find(slice);
        let token = match find_token {
            Some(token) => token,
            None => break,
        };

        if !tokens_discard.is_match(token.as_str()) {
            output.push(token.as_str().to_string());
        }
        slice = &slice[token.end()..];
    }

    output
}
