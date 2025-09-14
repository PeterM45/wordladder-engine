//! # Word Ladder Engine
//!
//! A high-performance Rust library for generating and solving word ladder puzzles.
//! This library provides efficient algorithms for finding shortest paths between words
//! and generating puzzles of varying difficulty levels.
//!
//! ## Architecture
//!
//! The library is organized into several key modules:
//! - `config`: Configuration management and defaults
//! - `graph`: Word graph construction and BFS path finding
//! - `puzzle`: Puzzle generation, validation, and difficulty assessment
//! - `cli`: Command-line interface for the application
//! - `exporters`: Export functionality for different formats (SQL, etc.)
//!
//! ## Key Features
//!
//! - **Efficient Word Graph**: Uses adjacency list representation with BFS for optimal path finding
//! - **Configurable Difficulty**: Easy (2-3 steps), Medium (4-5 steps), Hard (6-10 steps)
//! - **Dual Dictionary System**: Separate dictionaries for path finding and puzzle endpoints
//! - **Dictionary Export**: Export dictionary to SQL for O(log n) mobile lookups
//! - **Async File I/O**: Fast loading of large dictionary files
//! - **Comprehensive Error Handling**: Robust error handling with detailed messages
//! - **Multiple Export Formats**: Support for text, JSON, and SQL export formats
//! - **Mobile Integration**: Direct SQL export for React Native/SQLite applications
//!
//! ## Example
//!
//! ```rust
//! use wordladder_engine::{config::Config, graph::WordGraph, puzzle::PuzzleGenerator};
//!
//! // Create a word graph
//! let mut graph = WordGraph::new();
//! graph.load_dictionary("data/dictionary.txt").unwrap();
//! graph.load_base_words("data/base_words.txt").unwrap();
//!
//! // Create puzzle generator
//! let generator = PuzzleGenerator::new(graph);
//!
//! // Generate a puzzle
//! if let Some(puzzle) = generator.generate_puzzle("cat", "dog") {
//!     println!("Found path: {:?}", puzzle.path);
//! }
//! ```

pub mod cli;
pub mod config;
pub mod exporters;
pub mod graph;
pub mod puzzle;
