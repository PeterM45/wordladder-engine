//! # Configuration Management
//!
//! This module provides centralized configuration management for the word ladder engine.
//! It defines default paths and settings that can be customized through the builder pattern.
//!
//! ## Default Configuration
//!
//! The default configuration uses these paths:
//! - Dictionary: `data/dictionary.txt`
//! - Base words: `data/base_words.txt`
//! - Output directory: `output/`
//! - Bulk puzzle count: 100 puzzles per difficulty
//!
//! ## Usage
//!
//! ```rust
//! use wordladder_engine::config::Config;
//!
//! // Use defaults
//! let config = Config::default();
//!
//! // Customize with builder pattern
//! let custom_config = Config::new()
//!     .with_dictionary_path("custom/dict.txt".into())
//!     .with_output_dir("results".into())
//!     .with_bulk_puzzle_count(50);
//! ```

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Central configuration structure for the word ladder engine.
///
/// This struct contains all configurable settings including file paths,
/// output directories, and generation parameters. It uses the builder pattern
/// for ergonomic configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Path to the dictionary file containing all valid words for path finding.
    /// This should be a text file with one word per line.
    pub dictionary_path: PathBuf,

    /// Path to the base words file containing curated words for puzzle endpoints.
    /// These words are used as start and end points for generated puzzles.
    pub base_words_path: PathBuf,

    /// Directory where generated puzzle files will be saved.
    /// This directory will be created if it doesn't exist.
    pub output_dir: PathBuf,

    /// Number of puzzles to generate for each difficulty level during bulk generation.
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
    /// Creates a new configuration with default values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wordladder_engine::config::Config;
    ///
    /// let config = Config::new();
    /// assert_eq!(config.bulk_puzzle_count, 100);
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the dictionary file path.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the dictionary file
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wordladder_engine::config::Config;
    ///
    /// let config = Config::new()
    ///     .with_dictionary_path("custom/dict.txt".into());
    /// ```
    pub fn with_dictionary_path(mut self, path: PathBuf) -> Self {
        self.dictionary_path = path;
        self
    }

    /// Sets the base words file path.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the base words file
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wordladder_engine::config::Config;
    ///
    /// let config = Config::new()
    ///     .with_base_words_path("custom/base.txt".into());
    /// ```
    pub fn with_base_words_path(mut self, path: PathBuf) -> Self {
        self.base_words_path = path;
        self
    }

    /// Sets the output directory for generated puzzles.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the output directory
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wordladder_engine::config::Config;
    ///
    /// let config = Config::new()
    ///     .with_output_dir("results".into());
    /// ```
    pub fn with_output_dir(mut self, path: PathBuf) -> Self {
        self.output_dir = path;
        self
    }

    /// Sets the number of puzzles to generate per difficulty level.
    ///
    /// # Arguments
    ///
    /// * `count` - Number of puzzles per difficulty level
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wordladder_engine::config::Config;
    ///
    /// let config = Config::new()
    ///     .with_bulk_puzzle_count(50);
    /// ```
    pub fn with_bulk_puzzle_count(mut self, count: usize) -> Self {
        self.bulk_puzzle_count = count;
        self
    }
}
