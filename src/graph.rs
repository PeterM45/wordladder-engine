//! # Word Graph Implementation
//!
//! This module implements the core word graph data structure and BFS algorithm
//! for finding shortest paths between words in word ladder puzzles.
//!
//! ## Architecture
//!
//! The `WordGraph` uses an adjacency list representation where each word maps to
//! a list of words that differ by exactly one letter. This allows for efficient
//! BFS traversal to find shortest paths.
//!
//! ## Key Components
//!
//! - **Dictionary Words**: Full set of valid words for path finding
//! - **Base Words**: Curated words used as puzzle start/end points
//! - **Adjacency Graph**: Maps each word to its valid neighbors
//! - **BFS Algorithm**: Finds shortest paths between any two words
//!
//! ## Performance
//!
//! - **Space Complexity**: O(V + E) where V is words, E is word relationships
//! - **Time Complexity**: O(V + E) for BFS, O(1) average for neighbor lookup
//! - **Optimized for**: Dictionaries with 15k-25k words
//!
//! ## Example
//!
//! ```rust
//! use wordladder_engine::graph::WordGraph;
//!
//! let mut graph = WordGraph::new();
//! graph.load_dictionary("data/dictionary.txt")?;
//! graph.load_base_words("data/base_words.txt")?;
//!
//! // Find shortest path
//! if let Some(path) = graph.find_shortest_path("cat", "dog") {
//!     println!("Path: {:?}", path);
//! }
//! # Ok::<(), anyhow::Error>(())
//! ```

use anyhow::Result;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

/// Core data structure representing a graph of words connected by single-letter changes.
///
/// The `WordGraph` maintains three key data structures:
/// - `graph`: Adjacency list mapping words to their neighbors
/// - `words`: Set of all valid dictionary words
/// - `base_words`: Set of curated words for puzzle endpoints
///
/// This design allows efficient path finding while maintaining separation between
/// the full dictionary (for paths) and base words (for puzzle selection).
#[derive(Debug, Clone)]
pub struct WordGraph {
    /// Adjacency list: word -> list of words differing by one letter
    graph: HashMap<String, Vec<String>>,
    /// Set of all valid dictionary words for path finding
    words: HashSet<String>,
    /// Set of curated words used as puzzle start/end points
    base_words: HashSet<String>,
}

impl WordGraph {
    /// Creates a new empty word graph.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wordladder_engine::graph::WordGraph;
    ///
    /// let graph = WordGraph::new();
    /// assert!(graph.get_words().is_empty());
    /// ```
    pub fn new() -> Self {
        Self {
            graph: HashMap::new(),
            words: HashSet::new(),
            base_words: HashSet::new(),
        }
    }

    /// Loads dictionary words from a file and builds the word graph.
    ///
    /// This method reads a text file containing one word per line, filters for
    /// valid alphabetic words, and constructs the adjacency graph for efficient
    /// path finding.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the dictionary file
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if successful, or an error if the file cannot be read.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wordladder_engine::graph::WordGraph;
    ///
    /// let mut graph = WordGraph::new();
    /// graph.load_dictionary("data/dictionary.txt")?;
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn load_dictionary(&mut self, path: &str) -> Result<()> {
        let content = fs::read_to_string(path)?;
        let words: HashSet<String> = content
            .lines()
            .map(|line| line.trim().to_lowercase())
            .filter(|word| !word.is_empty() && word.chars().all(|c| c.is_alphabetic()))
            .collect();

        self.words = words;
        self.build_graph();
        Ok(())
    }

    /// Loads base words from a file for use as puzzle endpoints.
    ///
    /// Base words are a curated subset of dictionary words that are suitable
    /// for use as start and end points in puzzles. They should be common words
    /// that players are likely to know.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the base words file
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if successful, or an error if the file cannot be read.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wordladder_engine::graph::WordGraph;
    ///
    /// let mut graph = WordGraph::new();
    /// graph.load_base_words("data/base_words.txt")?;
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn load_base_words(&mut self, path: &str) -> Result<()> {
        let content = fs::read_to_string(path)?;
        self.base_words = content
            .lines()
            .map(|line| line.trim().to_lowercase())
            .filter(|word| !word.is_empty() && word.chars().all(|c| c.is_alphabetic()))
            .collect();
        Ok(())
    }

    /// Builds the adjacency graph from the loaded dictionary words.
    ///
    /// This method creates a graph where each word is connected to all words
    /// that differ by exactly one letter. The graph is stored as an adjacency
    /// list for efficient traversal during BFS.
    ///
    /// # Performance
    ///
    /// Time complexity: O(W * L * 26) where W is word count, L is word length
    fn build_graph(&mut self) {
        let word_list: Vec<String> = self.words.iter().cloned().collect();
        for word in &word_list {
            let neighbors = self.generate_neighbors(word);
            self.graph.insert(word.clone(), neighbors);
        }
    }

    /// Generates all valid neighbors for a given word.
    ///
    /// A neighbor is a word that differs from the input by exactly one letter
    /// and exists in the dictionary. This method systematically tries changing
    /// each letter to every other letter in the alphabet.
    ///
    /// # Arguments
    ///
    /// * `word` - The word to find neighbors for
    ///
    /// # Returns
    ///
    /// A vector of neighboring words
    ///
    /// # Performance
    ///
    /// Time complexity: O(L * 26) where L is word length
    fn generate_neighbors(&self, word: &str) -> Vec<String> {
        let mut neighbors = Vec::new();
        let chars: Vec<char> = word.chars().collect();
        let alphabet = "abcdefghijklmnopqrstuvwxyz";

        for i in 0..chars.len() {
            for &c in alphabet.as_bytes() {
                let new_char = c as char;
                if new_char != chars[i] {
                    let mut new_word = chars.clone();
                    new_word[i] = new_char;
                    let new_word_str: String = new_word.into_iter().collect();
                    if self.words.contains(&new_word_str) {
                        neighbors.push(new_word_str);
                    }
                }
            }
        }
        neighbors
    }

    /// Finds the shortest path between two words using BFS.
    ///
    /// This method implements breadth-first search to find the shortest path
    /// between a start and end word. The path consists of words where each
    /// consecutive pair differs by exactly one letter.
    ///
    /// # Arguments
    ///
    /// * `start` - Starting word
    /// * `end` - Ending word
    ///
    /// # Returns
    ///
    /// Returns `Some(path)` if a path exists, `None` if no path is found.
    /// The path includes both start and end words.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wordladder_engine::graph::WordGraph;
    ///
    /// let mut graph = WordGraph::new();
    /// // ... load dictionary ...
    /// # graph.load_dictionary("data/dictionary.txt").ok();
    ///
    /// if let Some(path) = graph.find_shortest_path("cat", "dog") {
    ///     println!("Path: {:?}", path); // ["cat", "cot", "cog", "dog"]
    /// }
    /// ```
    ///
    /// # Performance
    ///
    /// Time complexity: O(V + E) where V is vertices (words), E is edges
    pub fn find_shortest_path(&self, start: &str, end: &str) -> Option<Vec<String>> {
        if start == end {
            return Some(vec![start.to_string()]);
        }

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut parent = HashMap::new();

        queue.push_back(start.to_string());
        visited.insert(start.to_string());

        while let Some(current) = queue.pop_front() {
            if let Some(neighbors) = self.graph.get(&current) {
                for neighbor in neighbors {
                    if !visited.contains(neighbor) {
                        visited.insert(neighbor.clone());
                        parent.insert(neighbor.clone(), current.clone());
                        if neighbor == end {
                            return Some(self.reconstruct_path(&parent, start, end));
                        }
                        queue.push_back(neighbor.clone());
                    }
                }
            }
        }
        None
    }

    /// Reconstructs the path from BFS parent pointers.
    ///
    /// This helper method traces back from the end word to the start word
    /// using the parent map built during BFS to reconstruct the complete path.
    ///
    /// # Arguments
    ///
    /// * `parent` - Map of child -> parent relationships from BFS
    /// * `start` - Starting word
    /// * `end` - Ending word
    ///
    /// # Returns
    ///
    /// The complete path from start to end
    fn reconstruct_path(
        &self,
        parent: &HashMap<String, String>,
        start: &str,
        end: &str,
    ) -> Vec<String> {
        let mut path = vec![end.to_string()];
        let mut current = end.to_string();

        while current != start {
            if let Some(prev) = parent.get(&current) {
                path.push(prev.clone());
                current = prev.clone();
            } else {
                break;
            }
        }
        path.reverse();
        path
    }

    /// Returns a reference to the set of dictionary words.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wordladder_engine::graph::WordGraph;
    ///
    /// let mut graph = WordGraph::new();
    /// // ... load dictionary ...
    /// # graph.load_dictionary("data/dictionary.txt").ok();
    ///
    /// let words = graph.get_words();
    /// println!("Dictionary contains {} words", words.len());
    /// ```
    pub fn get_words(&self) -> &HashSet<String> {
        &self.words
    }

    /// Returns a reference to the set of base words.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wordladder_engine::graph::WordGraph;
    ///
    /// let mut graph = WordGraph::new();
    /// // ... load base words ...
    /// # graph.load_base_words("data/base_words.txt").ok();
    ///
    /// let base_words = graph.get_base_words();
    /// println!("{} base words available", base_words.len());
    /// ```
    pub fn get_base_words(&self) -> &HashSet<String> {
        &self.base_words
    }
}

impl Default for WordGraph {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_dictionary() {
        let mut graph = WordGraph::new();
        // Create a temporary dictionary
        let dict_content = "cat\ndog\nbat\nrat\nmat\n";
        std::fs::write("test_dict.txt", dict_content).unwrap();
        graph.load_dictionary("test_dict.txt").unwrap();
        std::fs::remove_file("test_dict.txt").unwrap();

        assert!(graph.words.contains("cat"));
        assert!(graph.words.contains("dog"));
        assert_eq!(graph.words.len(), 5);
    }

    #[test]
    fn test_find_shortest_path() {
        let mut graph = WordGraph::new();
        let dict_content = "cat\ndog\ncog\ncot\n";
        std::fs::write("test_dict2.txt", dict_content).unwrap();
        graph.load_dictionary("test_dict2.txt").unwrap();
        std::fs::remove_file("test_dict2.txt").unwrap();

        let path = graph.find_shortest_path("cat", "dog");
        assert!(path.is_some());
        let path = path.unwrap();
        assert_eq!(path, vec!["cat", "cot", "cog", "dog"]);
    }
}
