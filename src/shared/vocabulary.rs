use std::{collections::HashMap, fmt::Display, fs::File, io::BufReader, marker::PhantomData};

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

    pub fn get_token(&self) -> String {
        self.token.clone()
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
    pub fn from_json(path: &str) -> Result<Vocabulary<ReadOnly>, VocabError> {
        let end = path.split(".").last();
        if end.is_none() || end.unwrap() != "json" {
            return Err(VocabError::new("File is not a JSON file."));
        }
        let file = match File::open(path) {
            Ok(f) => f,
            Err(err) => {
                return Err(VocabError::new(format!("Failed to open file: {}", err).as_str()));
            }
        };

        let reader = BufReader::new(file);

        let raw_map: HashMap<String, Vec<String>> = match serde_json::from_reader(reader) {
            Ok(m) => m,
            Err(err) => {
                return Err(VocabError::new(format!("Failed to parse JSON: {}", err).as_str()));
            }
        };

        let tokens: Vec<Token> = raw_map.iter().map(|(k, v)| {
            let pair = if v.len() == 2 { Some((v[0].clone(), v[1].clone())) } else { None };
            Token::new(k.clone(), pair, None)
        }).collect();

        Ok(Vocabulary {
            state: PhantomData,
            tokens,
        })
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
