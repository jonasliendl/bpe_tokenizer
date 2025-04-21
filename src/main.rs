mod training;
mod prod;
mod shared;

use log::LevelFilter;
use prod::Tokenizer;
use training::{loader::{Closed, LoaderOptions, TextLoader}, Initialized};
use shared::{export::{ExportHandler, ExportTypes}, vocabulary::Vocabulary};

use crate::training::Training;
use crate::shared::log::SimpleLogger;

const LOGGER: SimpleLogger = SimpleLogger;

fn main() {
    if log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info)).is_err() {
        println!("Failed to set logger");
        return;
    }

    // let path = "data/en.txt";

    // let loader: TextLoader<Closed> = TextLoader::new(LoaderOptions::TXT, path);

    // let trainer: Training<Initialized> = Training::new(loader);

    // let result_read = match trainer.start_training() {
    //     Ok(res) => res,
    //     Err(err) => {
    //         println!("Error: {}", err);
    //         return;
    //     }
    // };

    // let result_merge = match result_read.start_merge() {
    //     Ok(res) => res,
    //     Err(err) => {
    //         println!("Error: {}", err);
    //         return;
    //     }
    // };

    // let vocab = result_merge.get_vocabulary();

    // let export_handler = ExportHandler::new(ExportTypes::JSON);
    
    // match export_handler.export_vocabulary(vocab, "tokens/en_first.json") {
    //     Ok(_) => {},
    //     Err(err) => {
    //         log::error!("{}", err.msg);
    //         return;
    //     }
    // };

    let read_vocab = match Vocabulary::from_json("tokens/en_first.json") {
        Ok(vocab) => vocab,
        Err(err) => {
            log::error!("{}", err.msg);
            return;
        }
    };

    log::info!("Vocabulary size: {}", read_vocab.token_count());
    log::info!("Tokens: {}", read_vocab);

    let example_text = "This is a test text for the tokenizer. It should be able to handle this text correctly.";

    let tokenizer = Tokenizer::new(read_vocab);
    let tokens = tokenizer.tokenize(example_text.to_string());

    log::info!("Tokens: {:?}", tokens);
}
