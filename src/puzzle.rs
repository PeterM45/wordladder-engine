use crate::graph::WordGraph;
use anyhow::{Result, anyhow};
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Puzzle {
    pub start: String,
    pub end: String,
    pub path: Vec<String>,
    pub difficulty: Difficulty,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl Puzzle {
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

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

pub struct PuzzleGenerator {
    graph: WordGraph,
}

impl PuzzleGenerator {
    pub fn new(graph: WordGraph) -> Self {
        Self { graph }
    }

    pub fn generate_puzzle(&self, start: &str, end: &str) -> Option<Puzzle> {
        self.graph
            .find_shortest_path(start, end)
            .map(|path| Puzzle::new(start.to_string(), end.to_string(), path))
    }

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

    fn matches_difficulty(&self, puzzle: &Puzzle, target: &Difficulty) -> bool {
        matches!(
            (puzzle.difficulty, target),
            (Difficulty::Easy, Difficulty::Easy)
                | (Difficulty::Medium, Difficulty::Medium)
                | (Difficulty::Hard, Difficulty::Hard)
        )
    }

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
