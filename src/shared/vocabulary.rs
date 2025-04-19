use std::{fmt::Display, marker::PhantomData};

use crate::shared::error::VocabError;

#[derive(Clone, Debug)]
pub struct Token {
    token: String,
    pair: Option<(String, String)>,
    occurrences: usize,
}

impl Token {
    pub fn new(token: String, pair: Option<(String, String)>, occurrence: Option<usize>) -> Self {
        Token { token, pair, occurrences: occurrence.unwrap_or(1) }
    }

    pub fn increase_occurrence(&mut self, occurrence: Option<usize>) {
        if let Some(occ) = occurrence {
            self.occurrences += occ;
        } else {
            self.occurrences += 1;
        }
    }

    pub fn get_token(&self) -> &String {
        &self.token
    }

    pub fn get_occurrence(&self) -> usize {
        self.occurrences
    }

    pub fn get_pair(&self) -> Option<(String, String)> {
        self.pair.clone()
    }
}

#[derive(Clone, Debug)]
pub struct Edit;

#[derive(Clone, Debug)]
pub struct ReadOnly;

#[derive(Clone, Debug)]
pub struct Vocabulary<S> {
    state: PhantomData<S>,
    tokens: Vec<Token>,
}

impl<S> Vocabulary<S> {
    pub fn new() -> Vocabulary<Edit> {
        Vocabulary {
            tokens: Vec::new(),
            state: PhantomData,
        }
    }

    pub fn token_count(&self) -> usize {
        self.tokens.len()
    }

    pub fn get_tokens(&self) -> Vec<Token> {
        self.tokens.clone()
    }
}

impl Vocabulary<Edit> {
    pub fn add(&mut self, token: Token) -> Result<(), VocabError> {
        let tokens: Vec<String> = self.tokens.iter().map(|x| x.token.clone()).collect();
        if tokens.contains(&token.token) {
            return Err(VocabError::new(format!("Token {} is already part of the vocabulary.", token.token).as_str()));
        }

        self.tokens.push(token);

        Ok(())
    }

    pub fn append(&mut self, token_list: Vec<Token>) {
        let tokens: Vec<String> = self.tokens.iter().map(|x| x.token.clone()).collect();
        for token in token_list {
            if !tokens.contains(&token.token) {
                self.tokens.push(token);
            }
        }
    }
}

impl Vocabulary<ReadOnly> {
    pub fn from_json() -> Result<Vocabulary<ReadOnly>, VocabError> {
        todo!()
    }
}

impl<S> Display for Vocabulary<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        for token in self.tokens.iter() {
            if token.pair.is_some() {
                result.push_str(&format!("{} ({}, {})\n", token.token, token.pair.as_ref().unwrap().0, token.pair.as_ref().unwrap().1));
            } else {
                result.push_str(&format!("{}\n", token.token));
            }
        }
        write!(f, "{}", result)
    }
}
