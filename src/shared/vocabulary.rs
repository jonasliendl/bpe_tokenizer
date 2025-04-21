use std::{collections::{BTreeMap, HashMap}, fmt::Display, fs::File, io::{BufReader, BufWriter}, marker::PhantomData};

use serde::{Deserialize, Serialize};

use crate::shared::error::VocabError;

use super::error::ExportError;

#[derive(Clone, Debug)]
pub struct Token {
    token: String,
    token_id: usize,
    pair: Option<(usize, usize)>,
    occurrences: usize,
}

#[derive(Serialize, Deserialize)]
struct TokenInfo {
    pub token: String,
    pub pair: Vec<usize>,
}

impl Token {
    pub fn new(token: String, token_id: usize, pair: Option<(usize, usize)>, occurrence: Option<usize>) -> Self {
        Token { token, token_id, pair, occurrences: occurrence.unwrap_or(1) }
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

    pub fn get_pair(&self) -> Option<(usize, usize)> {
        self.pair.clone()
    }

    pub fn get_token_id(&self) -> usize {
        self.token_id
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
    fn sort_tokens(&mut self) {
        self.tokens.sort_by(|a, b| a.token_id.cmp(&b.token_id));
    }

    fn initialize_letters(&mut self) {
        for i in 0..=255 {
            let parsed_token = String::from_utf8_lossy(&vec![i]).to_string();
            let token = Token::new(
                parsed_token,
                i as usize,
                None,
                None,
            );
            self.tokens.push(token);
        }
    }

    pub fn new() -> Vocabulary<Edit> {
        let mut vocab = Vocabulary {
            tokens: Vec::new(),
            state: PhantomData,
        };
        vocab.initialize_letters();
        vocab.sort_tokens();
        vocab
    }

    pub fn token_count(&self) -> usize {
        self.tokens.len()
    }

    pub fn get_tokens(&self) -> Vec<Token> {
        self.tokens.clone()
    }

    pub fn to_json(&self, path: &str) -> Result<(), ExportError> {
        if path.split(".").last() != Some("json") {
            return Err(ExportError::new("File extension must be .json"));
        }
        let file = match File::create(path) {
            Ok(f) => f,
            Err(err) => {
                return Err(ExportError::new(err.to_string().as_str()));
            }
        };

        let writer = BufWriter::new(file);

        let mut raw_map: HashMap<String, TokenInfo> = HashMap::new();

        for token in &self.tokens {
            let pair = match token.get_pair() {
                Some(pair) => vec![pair.0, pair.1],
                None => Vec::new(),
            };
            raw_map.insert(token.token_id.to_string(), TokenInfo { token: token.token.clone(), pair });
        }

        match serde_json::to_writer_pretty(writer, &raw_map) {
            Ok(_) => {},
            Err(err) => {
                return Err(ExportError::new(format!("Failed to write JSON: {}", err).as_str()));
            }
        };
        Ok(())
    }

    pub fn get_last_id(&self) -> usize {
        if self.tokens.is_empty() {
            return 0;
        }
        self.tokens.last().unwrap().token_id
    }

    pub fn find_token(&self, token_id: usize) -> Option<&Token> {
        self.tokens.binary_search_by(|x| x.token_id.cmp(&token_id)).ok().and_then(|idx| {
            if idx < self.tokens.len() {
                Some(&self.tokens[idx])
            } else {
                None
            }
        })
    }
}

impl Vocabulary<Edit> {
    pub fn add(&mut self, token: Token) -> Result<(), VocabError> {
        if self.tokens.iter().any(|x| x.pair == token.pair && x.token == token.token) {
            return Err(VocabError::new("Token already exists."));
        }

        self.tokens.push(token);

        Ok(())
    }

    pub fn append(&mut self, token_list: Vec<Token>) {
        for token in token_list {
            match self.add(token) {
                Ok(_) => {},
                Err(_) => {}
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

        let raw_map: HashMap<String, TokenInfo> = match serde_json::from_reader(reader) {
            Ok(m) => m,
            Err(err) => {
                return Err(VocabError::new(format!("Failed to parse JSON: {}", err).as_str()));
            }
        };

        let tokens: Vec<Token> = raw_map.iter().map(|(k, v)| {
            let pair = if v.pair.len() == 2 { Some((v.pair[0].clone(), v.pair[1].clone())) } else { None };
            let token_id = k.parse::<usize>().unwrap_or(0);
            Token::new(v.token.clone(), token_id, pair, None)
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
