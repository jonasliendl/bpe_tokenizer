mod training;
mod prod;
mod shared;
mod decode;

use log::LevelFilter;
use prod::Tokenizer;
use training::{loader::{Closed, LoaderOptions, TextLoader}, Initialized};
use shared::vocabulary::Vocabulary;

use crate::training::Training;
use crate::shared::log::SimpleLogger;

const LOGGER: SimpleLogger = SimpleLogger;

fn main() {
    if log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info)).is_err() {
        println!("Failed to set logger");
        return;
    }

    let path = "data/en.txt";

    let loader: TextLoader<Closed> = TextLoader::new(LoaderOptions::TXT, path);

    let trainer: Training<Initialized> = Training::new(loader, None);

    let training_start = chrono::Utc::now();
    let result_read = match trainer.start_training() {
        Ok(res) => res,
        Err(err) => {
            println!("Error: {}", err);
            return;
        }
    };

    let result_merge = match result_read.start_merge() {
        Ok(res) => res,
        Err(err) => {
            println!("Error: {}", err);
            return;
        }
    };
    let training_end = chrono::Utc::now();

    let duration = training_end - training_start;

    log::info!("Training completed in: {} seconds", duration.num_seconds());

    let vocab = result_merge.get_vocabulary();

    match vocab.to_json("tokens/en_first.json") {
        Ok(_) => {},
        Err(err) => {
            log::error!("{}", err.msg);
            return;
        }
    };

    let read_vocab = match Vocabulary::from_json("tokens/en_first.json") {
        Ok(vocab) => vocab,
        Err(err) => {
            log::error!("{}", err.msg);
            return;
        }
    };

    log::info!("Vocabulary size: {}", read_vocab.token_count());

    let example_text = "This is a test text for the tokenizer. It should be able to handle this text correctly.";
    log::info!("Example text: {}", example_text);

    let tokenization_start = chrono::Utc::now();
    let tokenizer = Tokenizer::new(read_vocab.clone());
    let tokens = tokenizer.tokenize(example_text.to_string());
    let tokenization_end = chrono::Utc::now();

    let tokenization_duration = tokenization_end - tokenization_start;

    log::info!("Tokenization completed in: {} seconds", tokenization_duration.num_seconds());

    log::info!("Tokens: {:?}", tokens);

    let decoder = decode::Decoder::new(read_vocab);
    let decoded_text = decoder.decode(tokens.clone());
    log::info!("Decoded text: {}", decoded_text);
}
