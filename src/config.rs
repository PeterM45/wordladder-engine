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
//! - SQL batch size: 100 records per INSERT
//! - Mobile difficulty distribution: 40% easy, 40% medium, 20% hard
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
//!     .with_sql_batch_size(50)
//!     .with_mobile_distribution(0.5, 0.3, 0.2);
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

    /// Number of SQL records to batch in each INSERT statement for performance.
    pub sql_batch_size: usize,

    /// Whether to include CREATE TABLE schema by default in SQL exports.
    pub include_schema_by_default: bool,

    /// Difficulty distribution for mobile-optimized puzzle generation.
    pub mobile_difficulty_distribution: DifficultyDistribution,
}

/// Difficulty distribution configuration for mobile puzzle generation.
///
/// This struct defines the ratios of easy, medium, and hard puzzles to generate
/// for mobile applications, ensuring a balanced gameplay experience.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DifficultyDistribution {
    /// Ratio of easy puzzles (0.0 to 1.0)
    pub easy: f64,
    /// Ratio of medium puzzles (0.0 to 1.0)
    pub medium: f64,
    /// Ratio of hard puzzles (0.0 to 1.0)
    pub hard: f64,
}

impl Default for DifficultyDistribution {
    fn default() -> Self {
        Self {
            easy: 0.4,   // 40% easy
            medium: 0.4, // 40% medium
            hard: 0.2,   // 20% hard
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            dictionary_path: PathBuf::from("data/dictionary.txt"),
            base_words_path: PathBuf::from("data/base_words.txt"),
            output_dir: PathBuf::from("output"),
            bulk_puzzle_count: 100,
            sql_batch_size: 100,
            include_schema_by_default: true,
            mobile_difficulty_distribution: DifficultyDistribution::default(),
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

    /// Sets the SQL batch size for INSERT statements.
    ///
    /// # Arguments
    ///
    /// * `batch_size` - Number of records per INSERT statement
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wordladder_engine::config::Config;
    ///
    /// let config = Config::new()
    ///     .with_sql_batch_size(50);
    /// ```
    pub fn with_sql_batch_size(mut self, batch_size: usize) -> Self {
        self.sql_batch_size = batch_size;
        self
    }

    /// Sets whether to include schema by default in SQL exports.
    ///
    /// # Arguments
    ///
    /// * `include_schema` - Whether to include CREATE TABLE by default
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wordladder_engine::config::Config;
    ///
    /// let config = Config::new()
    ///     .with_include_schema_by_default(false);
    /// ```
    pub fn with_include_schema_by_default(mut self, include_schema: bool) -> Self {
        self.include_schema_by_default = include_schema;
        self
    }

    /// Sets the mobile difficulty distribution.
    ///
    /// # Arguments
    ///
    /// * `easy` - Ratio of easy puzzles (0.0 to 1.0)
    /// * `medium` - Ratio of medium puzzles (0.0 to 1.0)
    /// * `hard` - Ratio of hard puzzles (0.0 to 1.0)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wordladder_engine::config::Config;
    ///
    /// let config = Config::new()
    ///     .with_mobile_distribution(0.5, 0.3, 0.2);
    /// ```
    pub fn with_mobile_distribution(mut self, easy: f64, medium: f64, hard: f64) -> Self {
        self.mobile_difficulty_distribution = DifficultyDistribution { easy, medium, hard };
        self
    }
}
