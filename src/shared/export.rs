use std::collections::HashMap;
use std::fs::File;
use std::io::BufWriter;

use crate::shared::vocabulary::Vocabulary;

use super::error::ExportError;

pub enum ExportTypes {
    JSON,
}

pub struct ExportHandler {
    export_type: ExportTypes,
}

impl ExportHandler {
    pub fn new(export_type: ExportTypes) -> Self {
        ExportHandler {
            export_type,
        }
    }

    pub fn export_vocabulary<S>(&self, vocabulary: Vocabulary<S>, path: &str) -> Result<(), ExportError> {
        match self.export_type {
            ExportTypes::JSON => {
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

                let mut raw_map: HashMap<String, Vec<String>> = HashMap::new();

                for token in vocabulary.get_tokens() {
                    let pair = match token.get_pair() {
                        Some(pair) => vec![pair.0, pair.1],
                        None => Vec::new(),
                    };
                    raw_map.insert(token.get_token().to_string(), pair);
                }

                match serde_json::to_writer_pretty(writer, &raw_map) {
                    Ok(_) => {},
                    Err(err) => {
                        return Err(ExportError::new(format!("Failed to write JSON: {}", err).as_str()));
                    }
                };
            }
        }
        Ok(())
    }
}
