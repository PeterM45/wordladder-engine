use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub dictionary_path: PathBuf,
    pub base_words_path: PathBuf,
    pub output_dir: PathBuf,
    pub bulk_puzzle_count: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            dictionary_path: PathBuf::from("data/dictionary.txt"),
            base_words_path: PathBuf::from("data/base_words.txt"),
            output_dir: PathBuf::from("output"),
            bulk_puzzle_count: 100,
        }
    }
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_dictionary_path(mut self, path: PathBuf) -> Self {
        self.dictionary_path = path;
        self
    }

    pub fn with_base_words_path(mut self, path: PathBuf) -> Self {
        self.base_words_path = path;
        self
    }

    pub fn with_output_dir(mut self, path: PathBuf) -> Self {
        self.output_dir = path;
        self
    }

    pub fn with_bulk_puzzle_count(mut self, count: usize) -> Self {
        self.bulk_puzzle_count = count;
        self
    }
}
