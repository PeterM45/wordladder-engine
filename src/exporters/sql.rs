//! # SQL Export Module
//!
//! This module provides functionality to export word ladder puzzles to SQL format
//! for integration with mobile applications using SQLite databases.
//!
//! ## Features
//!
//! - **SQL Generation**: Creates valid SQLite-compatible INSERT statements
//! - **Batch Processing**: Groups INSERTs for optimal performance
//! - **ID Generation**: Creates unique puzzle IDs in word1_word2_counter format
//! - **Schema Creation**: Optional CREATE TABLE statements
//! - **SQL Injection Prevention**: Proper escaping of string values
//!
//! ## Usage
//!
//! ```rust
//! use wordladder_engine::exporters::sql::SqlExporter;
//!
//! let mut exporter = SqlExporter::new()
//!     .with_batch_size(100)
//!     .with_include_schema(true);
//!
//! let puzzles = vec![/* puzzle data */];
//! let sql = exporter.export_puzzles(&puzzles).unwrap();
//!
//! // Write to file
//! std::fs::write("puzzles.sql", sql).unwrap();
//! ```

use crate::puzzle::{Difficulty, Puzzle};
use anyhow::Result;
use std::collections::HashMap;
use std::collections::HashSet;

/// Configuration for SQL export functionality.
///
/// This struct contains settings that control how puzzles are exported to SQL format,
/// including batch size for INSERT statements and whether to include schema creation.
#[derive(Debug, Clone)]
pub struct SqlExportConfig {
    /// Number of INSERT statements to batch together for performance
    pub batch_size: usize,
    /// Whether to include CREATE TABLE statement at the beginning
    pub include_schema: bool,
    /// Whether to include comments in the SQL output
    pub include_comments: bool,
}

impl Default for SqlExportConfig {
    fn default() -> Self {
        Self {
            batch_size: 100,
            include_schema: true,
            include_comments: true,
        }
    }
}

/// SQL exporter for word ladder puzzles.
///
/// The `SqlExporter` handles the conversion of puzzle data to SQLite-compatible
/// SQL statements, with optimizations for bulk insertion and proper data escaping.
#[derive(Debug)]
pub struct SqlExporter {
    config: SqlExportConfig,
    id_counter: HashMap<String, usize>,
}

impl SqlExporter {
    /// Creates a new SQL exporter with default configuration.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wordladder_engine::exporters::sql::SqlExporter;
    ///
    /// let exporter = SqlExporter::new();
    /// ```
    pub fn new() -> Self {
        Self {
            config: SqlExportConfig::default(),
            id_counter: HashMap::new(),
        }
    }

    /// Creates a new SQL exporter with custom configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration for the exporter
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wordladder_engine::exporters::sql::{SqlExporter, SqlExportConfig};
    ///
    /// let config = SqlExportConfig {
    ///     batch_size: 50,
    ///     include_schema: false,
    ///     include_comments: true,
    /// };
    /// let exporter = SqlExporter::with_config(config);
    /// ```
    pub fn with_config(config: SqlExportConfig) -> Self {
        Self {
            config,
            id_counter: HashMap::new(),
        }
    }

    /// Sets the batch size for INSERT statements.
    ///
    /// # Arguments
    ///
    /// * `batch_size` - Number of records per INSERT statement
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wordladder_engine::exporters::sql::SqlExporter;
    ///
    /// let exporter = SqlExporter::new().with_batch_size(50);
    /// ```
    pub fn with_batch_size(mut self, batch_size: usize) -> Self {
        self.config.batch_size = batch_size;
        self
    }

    /// Sets whether to include CREATE TABLE schema.
    ///
    /// # Arguments
    ///
    /// * `include_schema` - Whether to include schema creation
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wordladder_engine::exporters::sql::SqlExporter;
    ///
    /// let exporter = SqlExporter::new().with_include_schema(true);
    /// ```
    pub fn with_include_schema(mut self, include_schema: bool) -> Self {
        self.config.include_schema = include_schema;
        self
    }

    /// Exports a collection of puzzles to SQL format.
    ///
    /// This method generates a complete SQL script containing:
    /// 1. Optional CREATE TABLE statement
    /// 2. Batched INSERT statements for all puzzles
    ///
    /// # Arguments
    ///
    /// * `puzzles` - Vector of puzzles to export
    ///
    /// # Returns
    ///
    /// A string containing the complete SQL script.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wordladder_engine::exporters::sql::SqlExporter;
    /// use wordladder_engine::puzzle::Puzzle;
    ///
    /// let mut exporter = SqlExporter::new();
    /// let puzzles = vec![/* puzzle data */];
    /// let sql = exporter.export_puzzles(&puzzles).unwrap();
    /// ```
    pub fn export_puzzles(&mut self, puzzles: &[Puzzle]) -> Result<String> {
        let mut sql = String::new();

        // Add schema if requested
        if self.config.include_schema {
            sql.push_str(&self.generate_schema());
            sql.push('\n');
        }

        // Add comments if requested
        if self.config.include_comments {
            sql.push_str(&format!("-- Generated {} puzzles\n", puzzles.len()));
            sql.push('\n');
        }

        // Generate INSERT statements in batches
        for chunk in puzzles.chunks(self.config.batch_size) {
            sql.push_str(&self.generate_batch_insert(chunk));
            sql.push('\n');
        }

        Ok(sql)
    }

    /// Generates the CREATE TABLE statement for the puzzles table.
    ///
    /// # Returns
    ///
    /// A string containing the CREATE TABLE SQL statement.
    fn generate_schema(&self) -> String {
        let mut schema = String::from(
            "-- Create puzzles table\n\
             CREATE TABLE IF NOT EXISTS puzzles (\n\
             \tid TEXT PRIMARY KEY,\n\
             \tstart_word TEXT NOT NULL,\n\
             \ttarget_word TEXT NOT NULL,\n\
             \tmin_steps INTEGER NOT NULL,\n\
             \tdifficulty TEXT NOT NULL\n\
             );",
        );

        if self.config.include_comments {
            schema.push_str("\n\n-- Indexes for better query performance\n");
            schema.push_str(
                "CREATE INDEX IF NOT EXISTS idx_puzzles_difficulty ON puzzles(difficulty);\n",
            );
            schema
                .push_str("CREATE INDEX IF NOT EXISTS idx_puzzles_steps ON puzzles(min_steps);\n");
        }

        schema
    }

    /// Generates a batched INSERT statement for a chunk of puzzles.
    ///
    /// # Arguments
    ///
    /// * `puzzles` - Slice of puzzles to insert
    ///
    /// # Returns
    ///
    /// A string containing the INSERT SQL statement.
    fn generate_batch_insert(&mut self, puzzles: &[Puzzle]) -> String {
        if puzzles.is_empty() {
            return String::new();
        }

        let mut sql = String::from(
            "INSERT INTO puzzles (id, start_word, target_word, min_steps, difficulty) VALUES\n",
        );

        for (i, puzzle) in puzzles.iter().enumerate() {
            let id = self.generate_puzzle_id(puzzle);
            let start_word = self.escape_sql_string(&puzzle.start);
            let target_word = self.escape_sql_string(&puzzle.end);
            let min_steps = puzzle.path.len() - 1; // number of steps
            let difficulty = self.difficulty_to_string(puzzle.difficulty);

            sql.push_str(&format!(
                "\t('{}', '{}', '{}', {}, '{}')",
                id, start_word, target_word, min_steps, difficulty
            ));

            if i < puzzles.len() - 1 {
                sql.push_str(",\n");
            } else {
                sql.push(';');
            }
        }

        sql
    }

    /// Generates a unique ID for a puzzle in the format word1_word2_counter.
    ///
    /// # Arguments
    ///
    /// * `puzzle` - The puzzle to generate an ID for
    ///
    /// # Returns
    ///
    /// A unique string ID for the puzzle.
    fn generate_puzzle_id(&mut self, puzzle: &Puzzle) -> String {
        let base_id = format!("{}_{}", puzzle.start, puzzle.end);
        let counter = self.id_counter.entry(base_id.clone()).or_insert(0);
        *counter += 1;
        format!("{}_{:03}", base_id, counter)
    }

    /// Converts a Difficulty enum to its string representation.
    ///
    /// # Arguments
    ///
    /// * `difficulty` - The difficulty level
    ///
    /// # Returns
    ///
    /// String representation of the difficulty.
    fn difficulty_to_string(&self, difficulty: Difficulty) -> &'static str {
        match difficulty {
            Difficulty::Easy => "easy",
            Difficulty::Medium => "medium",
            Difficulty::Hard => "hard",
        }
    }

    /// Escapes a string for safe SQL insertion.
    ///
    /// This method handles SQL injection prevention by escaping single quotes
    /// and other special characters that could be problematic in SQL strings.
    ///
    /// # Arguments
    ///
    /// * `s` - The string to escape
    ///
    /// # Returns
    ///
    /// An escaped version of the string safe for SQL insertion.
    fn escape_sql_string(&self, s: &str) -> String {
        s.replace('\'', "''") // Escape single quotes by doubling them
    }

    /// Exports puzzles with balanced difficulty distribution for mobile apps.
    ///
    /// This method creates a balanced set of puzzles with the specified distribution
    /// across difficulty levels, optimized for mobile game consumption.
    ///
    /// # Arguments
    ///
    /// * `puzzles` - All available puzzles to select from
    /// * `total_count` - Total number of puzzles to export
    /// * `easy_ratio` - Ratio of easy puzzles (0.0 to 1.0)
    /// * `medium_ratio` - Ratio of medium puzzles (0.0 to 1.0)
    /// * `hard_ratio` - Ratio of hard puzzles (0.0 to 1.0)
    ///
    /// # Returns
    ///
    /// A vector of selected puzzles with balanced difficulty distribution.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wordladder_engine::exporters::sql::SqlExporter;
    /// use wordladder_engine::puzzle::{Puzzle, Difficulty};
    ///
    /// let exporter = SqlExporter::new();
    /// let all_puzzles = vec![/* all available puzzles */];
    ///
    /// // Create balanced set: 40% easy, 40% medium, 20% hard
    /// let balanced = exporter.create_balanced_set(&all_puzzles, 1000, 0.4, 0.4, 0.2);
    /// ```
    pub fn create_balanced_set(
        &self,
        puzzles: &[Puzzle],
        total_count: usize,
        easy_ratio: f64,
        medium_ratio: f64,
        hard_ratio: f64,
    ) -> Vec<Puzzle> {
        // Group puzzles by difficulty
        let mut easy: Vec<&Puzzle> = puzzles
            .iter()
            .filter(|p| matches!(p.difficulty, Difficulty::Easy))
            .collect();
        let mut medium: Vec<&Puzzle> = puzzles
            .iter()
            .filter(|p| matches!(p.difficulty, Difficulty::Medium))
            .collect();
        let mut hard: Vec<&Puzzle> = puzzles
            .iter()
            .filter(|p| matches!(p.difficulty, Difficulty::Hard))
            .collect();

        // Shuffle each group for randomness
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();
        easy.shuffle(&mut rng);
        medium.shuffle(&mut rng);
        hard.shuffle(&mut rng);

        // Calculate counts for each difficulty
        let easy_count = (total_count as f64 * easy_ratio).round() as usize;
        let medium_count = (total_count as f64 * medium_ratio).round() as usize;
        let hard_count = (total_count as f64 * hard_ratio).round() as usize;

        // Adjust for rounding errors
        let actual_total = easy_count + medium_count + hard_count;
        let adjustment = total_count as isize - actual_total as isize;

        let (easy_count, medium_count, hard_count) = if adjustment > 0 {
            // Add extra to medium
            (easy_count, medium_count + adjustment as usize, hard_count)
        } else if adjustment < 0 {
            // Remove from hard if possible
            if hard_count > 0 {
                (
                    easy_count,
                    medium_count,
                    hard_count.saturating_sub((-adjustment) as usize),
                )
            } else if medium_count > 0 {
                (
                    easy_count,
                    medium_count.saturating_sub((-adjustment) as usize),
                    hard_count,
                )
            } else {
                (
                    easy_count.saturating_sub((-adjustment) as usize),
                    medium_count,
                    hard_count,
                )
            }
        } else {
            (easy_count, medium_count, hard_count)
        };

        // Select puzzles from each group, allowing duplicates if needed
        let mut selected = Vec::new();

        // Helper function to add puzzles of a specific difficulty
        let mut add_puzzles = |puzzles_of_type: &Vec<&Puzzle>, count: usize| {
            for i in 0..count {
                if !puzzles_of_type.is_empty() {
                    let index = i % puzzles_of_type.len();
                    selected.push((*puzzles_of_type[index]).clone());
                }
            }
        };

        add_puzzles(&easy, easy_count);
        add_puzzles(&medium, medium_count);
        add_puzzles(&hard, hard_count);

        // If we still don't have enough, fill with any available puzzles
        while selected.len() < total_count && !puzzles.is_empty() {
            let index = selected.len() % puzzles.len();
            selected.push(puzzles[index].clone());
        }

        selected
    }

    /// Exports dictionary words to SQL format for mobile database integration.
    ///
    /// This method generates SQL statements to create and populate a dictionary table
    /// with all valid words from the word graph. The table includes an index for
    /// efficient word lookups (O(log n) vs O(n) for text file scanning).
    ///
    /// # Arguments
    ///
    /// * `words` - The set of dictionary words to export
    ///
    /// # Returns
    ///
    /// A string containing the complete SQL script for the dictionary table.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wordladder_engine::exporters::sql::SqlExporter;
    /// use std::collections::HashSet;
    ///
    /// let mut exporter = SqlExporter::new();
    /// let words: HashSet<String> = ["cat", "dog", "bat"].iter().map(|s| s.to_string()).collect();
    /// let sql = exporter.export_dictionary(&words).unwrap();
    /// ```
    pub fn export_dictionary(&mut self, words: &HashSet<String>) -> Result<String> {
        let mut sql = String::new();

        // Add schema if requested
        if self.config.include_schema {
            sql.push_str(&self.generate_dictionary_schema());
            sql.push('\n');
        }

        // Add comments if requested
        if self.config.include_comments {
            sql.push_str(&format!("-- Generated {} dictionary words\n", words.len()));
            sql.push('\n');
        }

        // Generate INSERT statements in batches
        let word_list: Vec<&String> = words.iter().collect();
        for chunk in word_list.chunks(self.config.batch_size) {
            sql.push_str(&self.generate_dictionary_batch_insert(chunk));
            sql.push('\n');
        }

        Ok(sql)
    }

    /// Generates the CREATE TABLE statement for the dictionary table.
    ///
    /// # Returns
    ///
    /// A string containing the CREATE TABLE SQL statement for the dictionary.
    fn generate_dictionary_schema(&self) -> String {
        let mut schema = String::from(
            "-- Create dictionary table\n\
             CREATE TABLE IF NOT EXISTS dictionary (\n\
             \tword TEXT PRIMARY KEY,\n\
             \tlength INTEGER NOT NULL\n\
             );",
        );

        if self.config.include_comments {
            schema.push_str("\n\n-- Indexes for efficient word lookups\n");
            schema.push_str(
                "CREATE INDEX IF NOT EXISTS idx_dictionary_length ON dictionary(length);\n",
            );
        }

        schema
    }

    /// Generates a batched INSERT statement for a chunk of dictionary words.
    ///
    /// # Arguments
    ///
    /// * `words` - Slice of words to insert
    ///
    /// # Returns
    ///
    /// A string containing the INSERT SQL statement for the dictionary words.
    fn generate_dictionary_batch_insert(&self, words: &[&String]) -> String {
        if words.is_empty() {
            return String::new();
        }

        let mut sql = String::from("INSERT OR IGNORE INTO dictionary (word, length) VALUES\n");

        for (i, word) in words.iter().enumerate() {
            let escaped_word = self.escape_sql_string(word);
            let length = word.len();

            sql.push_str(&format!("\t('{}', {})", escaped_word, length));

            if i < words.len() - 1 {
                sql.push_str(",\n");
            } else {
                sql.push(';');
            }
        }

        sql
    }
}

impl Default for SqlExporter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::puzzle::{Difficulty, Puzzle};

    fn create_test_puzzle(
        start: &str,
        end: &str,
        path: Vec<String>,
        difficulty: Difficulty,
    ) -> Puzzle {
        Puzzle {
            start: start.to_string(),
            end: end.to_string(),
            path,
            difficulty,
        }
    }

    #[test]
    fn test_generate_puzzle_id() {
        let mut exporter = SqlExporter::new();
        let puzzle = create_test_puzzle(
            "cat",
            "dog",
            vec!["cat".to_string(), "dog".to_string()],
            Difficulty::Easy,
        );

        let id1 = exporter.generate_puzzle_id(&puzzle);
        let id2 = exporter.generate_puzzle_id(&puzzle);

        assert_eq!(id1, "cat_dog_001");
        assert_eq!(id2, "cat_dog_002");
    }

    #[test]
    fn test_escape_sql_string() {
        let exporter = SqlExporter::new();

        assert_eq!(exporter.escape_sql_string("normal"), "normal");
        assert_eq!(exporter.escape_sql_string("don't"), "don''t");
        assert_eq!(exporter.escape_sql_string("O'Connor"), "O''Connor");
    }

    #[test]
    fn test_difficulty_to_string() {
        let exporter = SqlExporter::new();

        assert_eq!(exporter.difficulty_to_string(Difficulty::Easy), "easy");
        assert_eq!(exporter.difficulty_to_string(Difficulty::Medium), "medium");
        assert_eq!(exporter.difficulty_to_string(Difficulty::Hard), "hard");
    }

    #[test]
    fn test_generate_batch_insert() {
        let mut exporter = SqlExporter::new();
        let puzzles = vec![create_test_puzzle(
            "cat",
            "dog",
            vec!["cat".to_string(), "cot".to_string(), "dog".to_string()],
            Difficulty::Easy,
        )];

        let sql = exporter.generate_batch_insert(&puzzles);
        assert!(sql.contains("INSERT INTO puzzles"));
        assert!(sql.contains("'cat_dog_001'"));
        assert!(sql.contains("'cat'"));
        assert!(sql.contains("'dog'"));
        assert!(sql.contains("2")); // min_steps
        assert!(sql.contains("'easy'"));
    }

    #[test]
    fn test_create_balanced_set() {
        let exporter = SqlExporter::new();
        let puzzles = vec![
            create_test_puzzle(
                "a",
                "b",
                vec!["a".to_string(), "b".to_string()],
                Difficulty::Easy,
            ),
            create_test_puzzle(
                "c",
                "d",
                vec!["c".to_string(), "d".to_string(), "e".to_string()],
                Difficulty::Easy,
            ),
            create_test_puzzle(
                "f",
                "g",
                vec![
                    "f".to_string(),
                    "g".to_string(),
                    "h".to_string(),
                    "i".to_string(),
                    "j".to_string(),
                    "k".to_string(),
                ],
                Difficulty::Medium,
            ),
            create_test_puzzle(
                "l",
                "m",
                vec![
                    "l".to_string(),
                    "m".to_string(),
                    "n".to_string(),
                    "o".to_string(),
                    "p".to_string(),
                    "q".to_string(),
                    "r".to_string(),
                    "s".to_string(),
                    "t".to_string(),
                ],
                Difficulty::Hard,
            ),
        ];

        let balanced = exporter.create_balanced_set(&puzzles, 10, 0.5, 0.3, 0.2);

        let easy_count = balanced
            .iter()
            .filter(|p| matches!(p.difficulty, Difficulty::Easy))
            .count();
        let medium_count = balanced
            .iter()
            .filter(|p| matches!(p.difficulty, Difficulty::Medium))
            .count();
        let hard_count = balanced
            .iter()
            .filter(|p| matches!(p.difficulty, Difficulty::Hard))
            .count();

        // Should have roughly the right distribution, but limited by available puzzles
        // We have 2 easy, 1 medium, 1 hard available
        // For 10 requested with 50%/30%/20% distribution, we expect:
        // - Easy: min(5, 2) = 2, but algorithm may duplicate to fill
        // - Medium: min(3, 1) = 1, but may duplicate
        // - Hard: min(2, 1) = 1, but may duplicate
        // Total should be 10, with remaining filled from available puzzles
        assert_eq!(balanced.len(), 10); // Should return exactly the requested count
        assert!(easy_count >= 1); // Should have at least some easy puzzles
        assert!(medium_count >= 1); // Should have at least some medium puzzles
        assert!(hard_count >= 1); // Should have at least some hard puzzles
    }

    #[test]
    fn test_export_dictionary() {
        let mut exporter = SqlExporter::new();
        let words: HashSet<String> = ["cat", "dog", "bat"]
            .iter()
            .map(|s| s.to_string())
            .collect();

        let sql = exporter.export_dictionary(&words).unwrap();

        // Check that the CREATE TABLE statement is present
        assert!(sql.contains("CREATE TABLE IF NOT EXISTS dictionary"));

        // Check that the INSERT statements are present for each word
        for word in &["cat", "dog", "bat"] {
            assert!(sql.contains(&format!("('{}', {})", word, word.len())));
        }

        // Check that the SQL ends with a semicolon
        assert!(sql.trim().ends_with(';'));
    }
}
