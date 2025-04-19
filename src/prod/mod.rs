// TODO: how to deal with multiple languages?

pub struct Tokenizer {
    tokens: Vec<String>,
}

impl Tokenizer {
    pub fn new(tokens: Vec<String>) -> Self {
        Tokenizer { tokens }
    }

    pub fn tokenize(&self, text: String) -> Vec<String> {
        let mut result = Vec::new();

        result
    }
}
