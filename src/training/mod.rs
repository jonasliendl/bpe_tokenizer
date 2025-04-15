pub mod vocabulary;
pub mod word;
pub mod loader;

use std::{collections::HashMap, marker::PhantomData};

use deunicode::deunicode;
use loader::{Closed, TextLoader};
use vocabulary::Vocabulary;
use word::Word;

use crate::utils::error::TrainingError;

pub struct DoneReading;
pub struct Finished;
pub struct Initialized;

pub struct Training<S> {
    state: PhantomData<S>,
    loader: TextLoader<Closed>,
    vocabulary: Vocabulary,
    word_data: HashMap<String, Word>,
}

impl<S> Training<S> {
    pub fn new(loader: TextLoader<Closed>) -> Self {
        Training {
            state: PhantomData,
            loader,
            vocabulary: Vocabulary::new(),
            word_data: HashMap::new(),
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
        let mut vocab: Vocabulary = self.vocabulary;

        while let Some(line) = loader.read_line() {
            let ln = match line {
                Ok(l) => deunicode(l.as_str()),
                Err(e) => {
                    return Err(TrainingError::new(e.to_string().as_str()));
                }
            };

            for word in ln.split_whitespace() {
                match words.get_mut(word) {
                    Some(wrd) => {
                        wrd.increase_occurence();
                    },
                    None => {
                        let parsed_word = Word::new(word, None);
                        vocab.append(parsed_word.letters.clone());
                        words.insert(word.to_string(), parsed_word);
                    }
                }
            }
        }

        let loader = loader.close();

        Ok(Training {
            state: PhantomData,
            loader,
            vocabulary: vocab,
            word_data: words,
        })
    }
}

impl Training<DoneReading> {
    pub fn start_merge(mut self) -> Result<Training<Finished>, TrainingError> {
        const TOKEN_LIMIT: usize = 10000;

        let mut combinations: HashMap<(String, String), usize> = HashMap::new();
        let mut words = self.word_data;

        let mut prev_token_count = 1;

        while TOKEN_LIMIT > self.vocabulary.token_count() && prev_token_count != self.vocabulary.token_count() {
            let progress = (self.vocabulary.token_count() as f32 / TOKEN_LIMIT as f32)*100.0;
            log::info!("Progress: {:.2}%", progress);
            prev_token_count = self.vocabulary.token_count();
            combinations.clear();
            for word in &words {
                if word.0.chars().count() > 1 {
                    for i in 1..word.1.letters.len() - 1 {
                        let letters = &word.1.letters[i-1..i+1];
                        match combinations.get_mut(&(letters[0].to_string(), letters[1].to_string())) {
                            Some(occurence)  => {
                                *occurence += word.1.occurence_count;
                            },
                            None => {
                                combinations.insert((letters[0].to_string(), letters[1].to_string()), word.1.occurence_count);
                            },
                        }
                    }
                }
            }

            match combinations.iter().max_by(|x, y| x.1.cmp(y.1)) {
                Some(item) => {
                    match self.vocabulary.add(vec![item.0.0.clone(), item.0.1.clone()].join("")) {
                        Ok(_) => {},
                        Err(e) => {
                            log::error!("Error adding {} to vocabulary: {}", vec![item.0.0.clone(), item.0.1.clone()].join(""), e);
                        }
                    };
                    words.iter_mut().for_each(|x| {
                        if x.0.contains(vec![item.0.0.clone(), item.0.1.clone()].join("").as_str()) && x.0.chars().count() > 1 {
                            x.1.merge_letters((item.0.0.clone(), item.0.1.clone()));
                        }
                    });
                },
                None => {},
            };

            combinations.clear();
        }

        Ok(Training {
            state: PhantomData,
            loader: self.loader,
            vocabulary: self.vocabulary,
            word_data: words,
        })
    }
}

impl Training<Finished> {
    pub fn get_vocabulary(&self) -> Vocabulary {
        self.vocabulary.clone()
    }
}
