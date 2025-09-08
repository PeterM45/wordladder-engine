use anyhow::Result;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

#[derive(Debug, Clone)]
pub struct WordGraph {
    graph: HashMap<String, Vec<String>>,
    words: HashSet<String>,
    base_words: HashSet<String>,
}

impl WordGraph {
    pub fn new() -> Self {
        Self {
            graph: HashMap::new(),
            words: HashSet::new(),
            base_words: HashSet::new(),
        }
    }

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

    pub fn load_base_words(&mut self, path: &str) -> Result<()> {
        let content = fs::read_to_string(path)?;
        self.base_words = content
            .lines()
            .map(|line| line.trim().to_lowercase())
            .filter(|word| !word.is_empty() && word.chars().all(|c| c.is_alphabetic()))
            .collect();
        Ok(())
    }

    fn build_graph(&mut self) {
        let word_list: Vec<String> = self.words.iter().cloned().collect();
        for word in &word_list {
            let neighbors = self.generate_neighbors(word);
            self.graph.insert(word.clone(), neighbors);
        }
    }

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

    pub fn get_words(&self) -> &HashSet<String> {
        &self.words
    }

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
