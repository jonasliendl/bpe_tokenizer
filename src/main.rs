mod training;
mod prod;
mod utils;

use log::LevelFilter;
use training::{loader::{Closed, LoaderOptions, TextLoader}, Initialized};
use utils::export::{ExportHandler, ExportTypes};

use crate::training::Training;
use crate::utils::log::SimpleLogger;

const LOGGER: SimpleLogger = SimpleLogger;

fn main() {
    if log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info)).is_err() {
        println!("Failed to set logger");
        return;
    }

    let path = "data/en.txt";

    let loader: TextLoader<Closed> = TextLoader::new(LoaderOptions::TXT, path);

    let trainer: Training<Initialized> = Training::new(loader);

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

    let vocab = result_merge.get_vocabulary();

    let export_handler = ExportHandler::new(ExportTypes::Text);
    
    match export_handler.export_vocabulary(vocab, "tokens/en_first.txt") {
        Ok(_) => {},
        Err(err) => {
            log::error!("{}", err.msg);
            return;
        }
    };
}
