use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct LoaderError {
    pub msg: String
}

impl LoaderError {
    pub fn new(msg: &str) -> Self {
        LoaderError {
            msg: msg.to_string(),
        }
    }
}

impl Display for LoaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LoaderError: {}", self.msg)
    }
}

impl Error for LoaderError {}

#[derive(Debug)]
pub struct VocabError {
    pub msg: String,
}

impl VocabError {
    pub fn new(msg: &str) -> Self {
        VocabError { msg: msg.to_string() }
    }
}

impl Display for VocabError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "VocabError: {}", self.msg)
    }
}

impl Error for VocabError {}

#[derive(Debug)]
pub struct TrainingError {
    pub msg: String,
}

impl TrainingError {
    pub fn new(msg: &str) -> Self {
        TrainingError { msg: msg.to_string() }
    }
}

impl Display for TrainingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TrainingError: {}", self.msg)
    }
}

impl Error for TrainingError {}

#[derive(Debug)]
pub struct ExportError {
    pub msg: String,
}

impl ExportError {
    pub fn new(msg: &str) -> Self {
        ExportError { msg: msg.to_string() }
    }
}

impl Display for ExportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ExportError: {}", self.msg)
    }
}

impl Error for ExportError {}
