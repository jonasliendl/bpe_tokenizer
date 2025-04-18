use std::fs::File;
use std::io::Write;

use crate::training::vocabulary::Vocabulary;

use super::error::ExportError;

pub enum ExportTypes {
    Text,
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

    pub fn export_vocabulary(&self, vocabulary: Vocabulary, path: &str) -> Result<(), ExportError> {
        match self.export_type {
            ExportTypes::Text => {
                let mut file = match File::create(path) {
                    Ok(f) => f,
                    Err(err) => {
                        return Err(ExportError::new(err.to_string().as_str()));
                    }
                };

                let tokens = vocabulary.get_tokens().clone();

                for token in tokens {
                    match writeln!(file, "{}", token) {
                        Ok(_) => {},
                        Err(err) => {
                            log::info!("Error while exporting tokens: {}", err.to_string());
                            return Err(ExportError::new(err.to_string().as_str()));
                        },
                    };
                }
            }
        }
        Ok(())
    }
}
