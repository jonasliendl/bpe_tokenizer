use crate::shared::vocabulary::{ReadOnly, Vocabulary};

pub struct Decoder {
    vocabulary: Vocabulary<ReadOnly>,
}

impl Decoder {
    pub fn new(vocabulary: Vocabulary<ReadOnly>) -> Self {
        Decoder { vocabulary }
    }

    pub fn decode(&self, tokens: Vec<usize>) -> String {
        let mut result = String::new();

        for token in tokens {
            match self.vocabulary.get_tokens().iter().find(|x| x.get_token_id() == token) {
                Some(tkn) => {
                    result.push_str(&tkn.get_token());
                }
                None => {
                    result.push_str("<unknown>");
                }
            }
        }

        result.to_string()
    }
}
