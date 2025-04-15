use std::{fs::File, io::{self, BufRead, BufReader, Lines}, marker::PhantomData};

use crate::utils::error::LoaderError;

pub struct Closed;
pub struct Open;

pub enum LoaderOptions {
    TXT
}

pub struct TextLoader<S> {
    option: LoaderOptions,
    state: PhantomData<S>,
    path: String,
    lines: Option<Lines<BufReader<File>>>
}

impl<S> TextLoader<S> {
    pub fn new(option: LoaderOptions, path: &str) -> Self {
        TextLoader {
            option,
            state: PhantomData,
            path: path.to_string(),
            lines: None,
        }
    }
}

impl TextLoader<Open> {
    pub fn read_line(&mut self) -> Option<io::Result<String>> {
        self.lines.as_mut()?.next()
    }

    pub fn close(self) -> TextLoader<Closed> {
        TextLoader::new(self.option, self.path.as_str())
    }
}

impl TextLoader<Closed> {
    pub fn open(self) -> Result<TextLoader<Open>, LoaderError> {
        let file = match File::open(&self.path) {
            Ok(f) => f,
            Err(e) => {
                log::error!("Unable to open TXT file: {}", e);
                return Err(LoaderError::new(format!("Unable to open file: {}", e).as_str()));
            },
        };

        let buffer = BufReader::new(file);

        Ok(TextLoader {
            option: self.option,
            path: self.path,
            state: PhantomData,
            lines: Some(buffer.lines()),
        })
    }
}
