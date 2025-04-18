use std::fmt::Display;

use crate::utils::error::VocabError;

#[derive(Clone, Debug)]
pub struct Vocabulary {
    tokens: Vec<String>,
}

impl Vocabulary {
    pub fn new() -> Self {
        Vocabulary {
            tokens: Vec::new(),
        }
    }

    pub fn add(&mut self, token: String) -> Result<(), VocabError> {
        if self.tokens.contains(&token) {
            return Err(VocabError::new(format!("Token {} is already part of the vocabulary.", token).as_str()));
        }

        self.tokens.push(token);

        Ok(())
    }

    pub fn append(&mut self, token_list: Vec<String>) {
        for token in token_list {
            if !self.tokens.contains(&token) {
                self.tokens.push(token);
            }
        }
    }

    pub fn token_count(&self) -> usize {
        self.tokens.len()
    }

    pub fn get_tokens(&self) -> &Vec<String> {
        &self.tokens
    }
}

impl Display for Vocabulary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        for token in &self.tokens {
            result.push_str(&format!("{}\n", token));
        }
        write!(f, "{}", result)
    }
}
