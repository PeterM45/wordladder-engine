//! # Puzzle Generation and Validation
//!
//! This module handles puzzle generation, difficulty assessment, and validation
//! for word ladder puzzles. It provides the core logic for creating puzzles of
//! varying difficulty levels and verifying user solutions.
//!
//! ## Key Components
//!
//! - **Puzzle Structure**: Represents a complete word ladder with start, end, path, and difficulty
//! - **Difficulty Levels**: Easy (3-4 steps), Medium (5-7 steps), Hard (8+ steps)
//! - **Puzzle Generator**: Creates puzzles using random word selection and path finding
//! - **Validation**: Verifies that puzzle solutions are valid word ladders
//!
//! ## Usage
//!
//! ```rust
//! use wordladder_engine::puzzle::{PuzzleGenerator, Difficulty};
//!
//! // Create generator with loaded graph
//! let generator = PuzzleGenerator::new(graph);
//!
//! // Generate a single puzzle
//! if let Some(puzzle) = generator.generate_puzzle("cat", "dog") {
//!     println!("Difficulty: {:?}", puzzle.difficulty);
//! }
//!
//! // Generate batch of puzzles
//! let puzzles = generator.generate_batch(10, Difficulty::Medium);
//!
//! // Verify a solution
//! let is_valid = generator.verify_puzzle("cat,cot,cog,dog")?;
//! # Ok::<(), anyhow::Error>(())
//! ```

use crate::graph::WordGraph;
use anyhow::{Result, anyhow};
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a complete word ladder puzzle with its solution path and difficulty.
///
/// A puzzle consists of a starting word, ending word, the complete path between them,
/// and an automatically calculated difficulty level based on the number of steps.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Puzzle {
    /// The starting word of the puzzle
    pub start: String,
    /// The ending word of the puzzle
    pub end: String,
    /// The complete path from start to end, including all intermediate words
    pub path: Vec<String>,
    /// The difficulty level of this puzzle based on path length
    pub difficulty: Difficulty,
}

/// Represents the difficulty level of a word ladder puzzle.
///
/// Difficulty is determined by the number of steps (word changes) required:
/// - Easy: 3-4 steps (4-5 total words in path)
/// - Medium: 5-7 steps (6-8 total words in path)
/// - Hard: 8+ steps (9+ total words in path)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Difficulty {
    /// Easy puzzles: 3-4 word changes (4-5 words total)
    Easy,
    /// Medium puzzles: 5-7 word changes (6-8 words total)
    Medium,
    /// Hard puzzles: 8+ word changes (9+ words total)
    Hard,
}

impl Puzzle {
    /// Creates a new puzzle with the given parameters.
    ///
    /// The difficulty is automatically calculated based on the path length.
    ///
    /// # Arguments
    ///
    /// * `start` - Starting word
    /// * `end` - Ending word
    /// * `path` - Complete path including start and end words
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wordladder_engine::puzzle::{Puzzle, Difficulty};
    ///
    /// let path = vec!["cat".to_string(), "cot".to_string(), "cog".to_string(), "dog".to_string()];
    /// let puzzle = Puzzle::new("cat".to_string(), "dog".to_string(), path);
    /// assert!(matches!(puzzle.difficulty, Difficulty::Easy));
    /// ```
    pub fn new(start: String, end: String, path: Vec<String>) -> Self {
        let len = path.len() - 1; // number of steps
        let difficulty = match len {
            3..=4 => Difficulty::Easy,
            5..=7 => Difficulty::Medium,
            _ => Difficulty::Hard,
        };
        Self {
            start,
            end,
            path,
            difficulty,
        }
    }

    /// Serializes the puzzle to a JSON string.
    ///
    /// # Returns
    ///
    /// A pretty-printed JSON string representation of the puzzle.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wordladder_engine::puzzle::Puzzle;
    ///
    /// let puzzle = Puzzle::new(
    ///     "cat".to_string(),
    ///     "dog".to_string(),
    ///     vec!["cat".to_string(), "dog".to_string()]
    /// );
    ///
    /// let json = puzzle.to_json().unwrap();
    /// println!("{}", json);
    /// ```
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

/// Generator for creating word ladder puzzles with various difficulty levels.
///
/// The `PuzzleGenerator` uses a loaded `WordGraph` to create puzzles by:
/// 1. Selecting random start and end words from the base words
/// 2. Finding the shortest path between them
/// 3. Filtering by desired difficulty level
/// 4. Ensuring puzzles meet quality criteria
pub struct PuzzleGenerator {
    /// The word graph containing dictionary and base words
    graph: WordGraph,
}

impl PuzzleGenerator {
    /// Creates a new puzzle generator with the given word graph.
    ///
    /// # Arguments
    ///
    /// * `graph` - A word graph with loaded dictionary and base words
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wordladder_engine::{graph::WordGraph, puzzle::PuzzleGenerator};
    ///
    /// let mut graph = WordGraph::new();
    /// // ... load dictionary and base words ...
    /// # graph.load_dictionary("data/dictionary.txt").ok();
    /// # graph.load_base_words("data/base_words.txt").ok();
    ///
    /// let generator = PuzzleGenerator::new(graph);
    /// ```
    pub fn new(graph: WordGraph) -> Self {
        Self { graph }
    }

    /// Generates a single puzzle between the specified start and end words.
    ///
    /// # Arguments
    ///
    /// * `start` - Starting word
    /// * `end` - Ending word
    ///
    /// # Returns
    ///
    /// Returns `Some(puzzle)` if a path exists between the words, `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wordladder_engine::puzzle::PuzzleGenerator;
    ///
    /// // Assuming generator is set up...
    /// # let generator = PuzzleGenerator::new(wordladder_engine::graph::WordGraph::new());
    ///
    /// if let Some(puzzle) = generator.generate_puzzle("cat", "dog") {
    ///     println!("Found puzzle with {} steps", puzzle.path.len() - 1);
    /// }
    /// ```
    pub fn generate_puzzle(&self, start: &str, end: &str) -> Option<Puzzle> {
        self.graph
            .find_shortest_path(start, end)
            .map(|path| Puzzle::new(start.to_string(), end.to_string(), path))
    }

    /// Generates a batch of puzzles with the specified difficulty level.
    ///
    /// This method creates multiple puzzles by randomly selecting word pairs
    /// and filtering for the desired difficulty. It ensures that generated
    /// puzzles are valid and meet the difficulty criteria.
    ///
    /// # Arguments
    ///
    /// * `count` - Number of puzzles to generate
    /// * `difficulty` - Desired difficulty level
    ///
    /// # Returns
    ///
    /// A vector of generated puzzles. May contain fewer than requested if
    /// sufficient valid puzzles cannot be found.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wordladder_engine::puzzle::{PuzzleGenerator, Difficulty};
    ///
    /// // Assuming generator is set up...
    /// # let generator = PuzzleGenerator::new(wordladder_engine::graph::WordGraph::new());
    ///
    /// let puzzles = generator.generate_batch(10, Difficulty::Medium);
    /// println!("Generated {} puzzles", puzzles.len());
    /// ```
    pub fn generate_batch(&self, count: usize, difficulty: Difficulty) -> Vec<Puzzle> {
        let by_length = self.get_valid_base_words_by_length();
        if by_length.is_empty() {
            return Vec::new();
        }

        // Find lengths with at least 2 words
        let valid_lengths: Vec<usize> = by_length
            .iter()
            .filter(|(_, words)| words.len() >= 2)
            .map(|(&len, _)| len)
            .collect();

        if valid_lengths.is_empty() {
            return Vec::new();
        }

        let mut rng = thread_rng();
        let mut puzzles = Vec::new();

        while puzzles.len() < count {
            let chosen_length = valid_lengths.choose(&mut rng).unwrap();
            let words = by_length.get(chosen_length).unwrap();

            let start = words.choose(&mut rng).unwrap().clone();
            let mut end = words.choose(&mut rng).unwrap().clone();
            while end == start {
                end = words.choose(&mut rng).unwrap().clone();
            }

            if let Some(puzzle) = self
                .generate_puzzle(&start, &end)
                .filter(|p| self.matches_difficulty(p, &difficulty))
            {
                puzzles.push(puzzle);
            }
        }
        puzzles
    }

    /// Groups valid base words by their length for efficient random selection.
    ///
    /// This method filters base words to ensure they exist in the dictionary
    /// and groups them by word length. This enables efficient random selection
    /// of words with matching lengths for puzzle generation.
    ///
    /// # Returns
    ///
    /// A HashMap mapping word lengths to vectors of valid words of that length.
    fn get_valid_base_words_by_length(&self) -> HashMap<usize, Vec<String>> {
        let base_words: Vec<String> = self.graph.get_base_words().iter().cloned().collect();
        if base_words.is_empty() {
            return HashMap::new();
        }

        // Filter base words to only include those in the dictionary
        let valid_words: Vec<String> = base_words
            .into_iter()
            .filter(|word| self.graph.get_words().contains(word))
            .collect();

        if valid_words.len() < 2 {
            return HashMap::new();
        }

        // Group by length
        let mut by_length: HashMap<usize, Vec<String>> = HashMap::new();
        for word in valid_words {
            by_length.entry(word.len()).or_default().push(word);
        }

        by_length
    }

    /// Checks if a puzzle matches the specified difficulty level.
    ///
    /// # Arguments
    ///
    /// * `puzzle` - The puzzle to check
    /// * `target` - The target difficulty level
    ///
    /// # Returns
    ///
    /// `true` if the puzzle matches the difficulty, `false` otherwise
    fn matches_difficulty(&self, puzzle: &Puzzle, target: &Difficulty) -> bool {
        matches!(
            (puzzle.difficulty, target),
            (Difficulty::Easy, Difficulty::Easy)
                | (Difficulty::Medium, Difficulty::Medium)
                | (Difficulty::Hard, Difficulty::Hard)
        )
    }

    /// Verifies that a puzzle solution is valid.
    ///
    /// This method checks that:
    /// 1. The puzzle contains at least 2 words
    /// 2. Each consecutive pair of words differs by exactly one letter
    ///
    /// # Arguments
    ///
    /// * `puzzle_str` - Comma-separated string of words (e.g., "cat,cot,cog,dog")
    ///
    /// # Returns
    ///
    /// Returns `Ok(true)` if valid, `Ok(false)` if invalid, or an error for malformed input.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wordladder_engine::puzzle::PuzzleGenerator;
    ///
    /// // Assuming generator is set up...
    /// # let generator = PuzzleGenerator::new(wordladder_engine::graph::WordGraph::new());
    ///
    /// match generator.verify_puzzle("cat,cot,cog,dog") {
    ///     Ok(true) => println!("Valid puzzle!"),
    ///     Ok(false) => println!("Invalid puzzle"),
    ///     Err(e) => println!("Error: {}", e),
    /// }
    /// ```
    pub fn verify_puzzle(&self, puzzle_str: &str) -> Result<bool, String> {
        let words: Vec<String> = puzzle_str
            .split(',')
            .map(|s| s.trim().to_lowercase())
            .collect();

        if words.len() < 2 {
            return Err("Puzzle must have at least 2 words".to_string());
        }

        for i in 0..words.len() - 1 {
            if !self.are_neighbors(&words[i], &words[i + 1]) {
                return Ok(false);
            }
        }
        Ok(true)
    }

    /// Checks if two words are valid neighbors (differ by exactly one letter).
    ///
    /// # Arguments
    ///
    /// * `word1` - First word
    /// * `word2` - Second word
    ///
    /// # Returns
    ///
    /// `true` if the words differ by exactly one letter and have the same length
    fn are_neighbors(&self, word1: &str, word2: &str) -> bool {
        if word1.len() != word2.len() {
            return false;
        }

        let mut diff_count = 0;
        for (c1, c2) in word1.chars().zip(word2.chars()) {
            if c1 != c2 {
                diff_count += 1;
                if diff_count > 1 {
                    return false;
                }
            }
        }
        diff_count == 1
    }

    /// Selects a random pair of base words for puzzle generation.
    ///
    /// This method randomly selects two different words of the same length
    /// from the available base words, ensuring they can be used as puzzle endpoints.
    ///
    /// # Returns
    ///
    /// Returns `Ok((start, end))` with two random words, or an error if insufficient words are available.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wordladder_engine::puzzle::PuzzleGenerator;
    ///
    /// // Assuming generator is set up with base words...
    /// # let generator = PuzzleGenerator::new(wordladder_engine::graph::WordGraph::new());
    ///
    /// match generator.pick_random_words() {
    ///     Ok((start, end)) => println!("Selected: {} -> {}", start, end),
    ///     Err(e) => println!("Error: {}", e),
    /// }
    /// ```
    pub fn pick_random_words(&self) -> Result<(String, String)> {
        let by_length = self.get_valid_base_words_by_length();
        if by_length.is_empty() {
            return Err(anyhow!("No base words loaded"));
        }

        // Find lengths with at least 2 words
        let valid_lengths: Vec<usize> = by_length
            .iter()
            .filter(|(_, words)| words.len() >= 2)
            .map(|(&len, _)| len)
            .collect();
        if valid_lengths.is_empty() {
            return Err(anyhow!("No word lengths with at least 2 valid base words"));
        }

        let mut rng = thread_rng();
        let chosen_length = valid_lengths.choose(&mut rng).unwrap();
        let words = by_length.get(chosen_length).unwrap();

        let start = words.choose(&mut rng).unwrap().clone();
        let mut end = words.choose(&mut rng).unwrap().clone();
        while end == start {
            end = words.choose(&mut rng).unwrap().clone();
        }

        Ok((start, end))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::WordGraph;

    #[test]
    fn test_verify_puzzle() {
        let mut graph = WordGraph::new();
        let dict_content = "cat\ndog\ncog\ncot\n";
        std::fs::write("test_dict3.txt", dict_content).unwrap();
        graph.load_dictionary("test_dict3.txt").unwrap();
        std::fs::remove_file("test_dict3.txt").unwrap();

        let generator = PuzzleGenerator::new(graph);
        assert!(generator.verify_puzzle("cat,cot,cog,dog").unwrap());
        assert!(!generator.verify_puzzle("cat,dog").unwrap());
    }

    #[test]
    fn test_puzzle_difficulty() {
        let puzzle = Puzzle::new(
            "a".to_string(),
            "b".to_string(),
            vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "d".to_string(),
            ],
        );
        assert!(matches!(puzzle.difficulty, Difficulty::Easy));
    }
}
