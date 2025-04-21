pub mod word;
pub mod loader;

use std::{collections::HashMap, marker::PhantomData};

use loader::{Closed, TextLoader};
use crate::shared::vocabulary::{Edit, Token, Vocabulary};
use word::Word;

use crate::shared::error::TrainingError;

pub struct DoneReading;
pub struct Finished;
pub struct Initialized;

pub struct Training<S> {
    state: PhantomData<S>,
    loader: TextLoader<Closed>,
    vocabulary: Vocabulary<Edit>,
    word_data: HashMap<String, Word>,
    token_limit: usize,
}

impl<S> Training<S> {
    pub fn new(loader: TextLoader<Closed>, token_limit: Option<usize>) -> Self {
        const TOKEN_LIMIT: usize = 10000;

        Training {
            state: PhantomData,
            loader,
            vocabulary: Vocabulary::<Edit>::new(),
            word_data: HashMap::new(),
            token_limit: token_limit.unwrap_or(TOKEN_LIMIT),
        }
    }
}

impl Training<Initialized> {
    pub fn start_training(self) -> Result<Training<DoneReading>, TrainingError> {
        let mut loader = match self.loader.open() {
            Ok(l) => l,
            Err(e) => {
                return Err(TrainingError::new(e.msg.as_str()));
            }
        };

        let mut words: HashMap<String, Word> = HashMap::new();

        while let Some(line) = loader.read_line() {
            let ln = match line {
                Ok(l) => l,
                Err(e) => {
                    return Err(TrainingError::new(e.to_string().as_str()));
                }
            };

            for word in ln.split_inclusive(' ') {
                match words.get_mut(word) {
                    Some(wrd) => {
                        wrd.increase_occurence();
                    },
                    None => {
                        let parsed_word = Word::new(word, None);
                        words.insert(word.to_string(), parsed_word);
                    }
                }
            }
        }

        let loader = loader.close();

        Ok(Training {
            state: PhantomData,
            loader,
            vocabulary: self.vocabulary,
            word_data: words,
            token_limit: self.token_limit,
        })
    }
}

impl Training<DoneReading> {
    pub fn start_merge(mut self) -> Result<Training<Finished>, TrainingError> {
        let mut tokens: Vec<Token> = Vec::new();

        let mut words = self.word_data;

        let mut prev_token_count = 1;

        while self.token_limit > self.vocabulary.token_count() && prev_token_count != self.vocabulary.token_count() {
            tokens.clear();
            prev_token_count = self.vocabulary.token_count();
            for word in &words {
                if word.0.chars().count() > 1 {
                    for i in 1..word.1.letters.len() - 1 {
                        let letters = &word.1.letters[i-1..i+1];
                        let merged_pair = (letters[0], letters[1]);
                        match tokens.iter_mut().find(|x| x.get_pair() == Some(merged_pair)) {
                            Some(token) => {
                                token.increase_occurrence(Some(word.1.occurence_count));
                            },
                            None => {
                                let first_token = match self.vocabulary.find_token(letters[0]) {
                                    Some(tkn) => tkn,
                                    None => {
                                        continue;
                                    }
                                };
                                let second_token = match self.vocabulary.find_token(letters[1]) {
                                    Some(tkn) => tkn,
                                    None => {
                                        continue;
                                    }
                                };
                                let token_id = self.vocabulary.get_last_id() + 1;
                                let new_token = vec![first_token.get_token(), second_token.get_token()].join("");
                                let token = Token::new(
                                    new_token,
                                    token_id,
                                    Some(merged_pair),
                                    Some(word.1.occurence_count)
                                );
                                tokens.push(token);
                            },
                        }
                    }
                }
            }

            match tokens.iter().max_by(|x, y| x.get_occurrence().cmp(&y.get_occurrence())) {
                Some(item) => {
                    match self.vocabulary.add(item.clone()) {
                        Ok(_) => {},
                        Err(_) => {
                            continue;
                        }
                    };
                    words.iter_mut().for_each(|x| {
                        let pair = match item.get_pair() {
                            Some(p) => p,
                            None => {
                                return;
                            }
                        };
                        x.1.merge_letters(item.get_token_id(), pair);
                    });
                },
                None => {},
            };
        }

        Ok(Training {
            state: PhantomData,
            loader: self.loader,
            vocabulary: self.vocabulary,
            word_data: words,
            token_limit: self.token_limit,
        })
    }
}

impl Training<Finished> {
    pub fn get_vocabulary(&self) -> Vocabulary<Edit> {
        self.vocabulary.clone()
    }
}
