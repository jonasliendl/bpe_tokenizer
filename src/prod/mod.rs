// TODO: how to deal with multiple languages?

use crate::shared::vocabulary::{ReadOnly, Token, Vocabulary};

pub struct Tokenizer {
    vocabulary: Vocabulary<ReadOnly>,
}

impl Tokenizer {
    pub fn new(vocabulary: Vocabulary<ReadOnly>) -> Self {
        Tokenizer { vocabulary }
    }

    pub fn tokenize(&self, text: String) -> Vec<usize> {
        let mut result: Vec<usize> = Vec::new();

        let binding = text.replace("\n", "\n ");
        let words = binding.split_whitespace();

        for word in words {
            match self.vocabulary.get_tokens().iter().find(|x| x.get_token() == word) {
                Some(token) => {
                    result.push(token.get_token_id());
                }
                None => {
                    let sub_tokens = self.tokenize_using_merge(word.to_string());
                    result.extend(sub_tokens);
                }
            };
        }

        result
    }

    fn tokenize_using_merge(&self, text: String) -> Vec<usize> {
        let tokens: Vec<usize> = text.as_bytes()
            .iter()
            .map(|x| *x as usize)
            .collect();
        let mut result: Vec<usize> = Vec::new();

        let binding = self.vocabulary.get_tokens();
        let merge_pairs: Vec<&Token> = binding.iter().filter(|x| x.get_pair().is_some()).collect();

        let mut idx = 0;

        while idx < tokens.len() {
            if tokens.len() > 0 && idx != tokens.len() - 1 {
                let pair = (tokens[idx].clone(), tokens[idx + 1].clone());
                match merge_pairs.iter().find(|x| x.get_pair().eq(&Some(pair.clone()))) {
                    Some(tkn) => {
                        result.push(tkn.get_token_id());
                        idx += 2;
                    },
                    None => {
                        result.push(tokens[idx].clone());
                        idx += 1;
                    }
                };
            } else if idx == tokens.len() - 1 {
                result.push(tokens[idx].clone());
                idx += 1;
            }
        }

        result
    }
}
