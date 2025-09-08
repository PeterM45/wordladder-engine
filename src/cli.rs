//! # Command-Line Interface
//!
//! This module implements the command-line interface for the word ladder engine.
//! It defines the CLI structure, command parsing, and execution logic.
//!
//! ## Commands
//!
//! The application supports three main commands:
//!
//! - `generate`: Generate puzzles (bulk or single)
//! - `batch`: Generate multiple puzzles to a file
//! - `verify`: Verify puzzle sequence validity
//!
//! ## Configuration Integration
//!
//! The CLI integrates with the configuration system to provide sensible defaults
//! while allowing users to override settings via command-line arguments.
//!
//! ## Usage Examples
//!
//! ```bash
//! // Generate bulk puzzles with defaults
//! wordladder-engine generate
//!
//! // Generate single puzzle with custom words
//! wordladder-engine generate --start cat --end dog
//!
//! // Generate batch with specific parameters
//! wordladder-engine batch --count 50 --difficulty medium --output puzzles.txt
//!
//! // Verify a puzzle solution
//! wordladder-engine verify --puzzle "cat,cot,cog,dog"
//! ```

use crate::config::Config;
use crate::graph::WordGraph;
use crate::puzzle::{Difficulty, PuzzleGenerator};
use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};

/// Main CLI structure for the word ladder engine.
///
/// This struct defines the top-level command-line interface and uses clap's
/// derive macros for automatic argument parsing and help generation.
#[derive(Parser)]
#[command(name = "wordladder-engine")]
#[command(about = "A CLI tool for generating word ladder puzzles")]
pub struct Cli {
    /// The subcommand to execute
    #[command(subcommand)]
    pub command: Commands,
}

/// Enumeration of available commands.
///
/// Each variant represents a different operation the application can perform,
/// with associated arguments for that specific command.
#[derive(Subcommand)]
pub enum Commands {
    /// Generate puzzles (bulk or single with arguments)
    ///
    /// This command can either:
    /// - Generate bulk puzzles for all difficulty levels (when no specific words provided)
    /// - Generate a single puzzle between specified start/end words
    /// - Output results in text or JSON format
    Generate {
        /// Path to dictionary file (defaults to config value)
        #[arg(short, long, default_value = "data/dictionary.txt")]
        dict: PathBuf,
        /// Path to base words file (defaults to config value)
        #[arg(short = 'b', long, default_value = "data/base_words.txt")]
        base_words: PathBuf,
        /// Starting word (optional, will pick random if not provided)
        #[arg(short, long)]
        start: Option<String>,
        /// Ending word (optional, will pick random if not provided)
        #[arg(short, long)]
        end: Option<String>,
        /// Output as JSON instead of text
        #[arg(long)]
        json: bool,
    },
    /// Generate multiple puzzles of specified difficulty to a file
    ///
    /// Creates a batch of puzzles with consistent difficulty and saves them
    /// to a text file. Useful for generating puzzle sets for games or challenges.
    Batch {
        /// Path to dictionary file (defaults to config value)
        #[arg(short, long, default_value = "data/dictionary.txt")]
        dict: PathBuf,
        /// Path to base words file (defaults to config value)
        #[arg(short = 'b', long, default_value = "data/base_words.txt")]
        base_words: PathBuf,
        /// Number of puzzles to generate
        #[arg(short, long, default_value = "10")]
        count: usize,
        /// Difficulty level (easy, medium, hard)
        #[arg(short, long, default_value = "medium")]
        difficulty: String,
        /// Output file path for the generated puzzles
        #[arg(short, long)]
        output: PathBuf,
    },
    /// Verify that a puzzle sequence is valid
    ///
    /// Checks whether a comma-separated sequence of words forms a valid
    /// word ladder where each consecutive pair differs by exactly one letter.
    Verify {
        /// Path to dictionary file (defaults to config value)
        #[arg(short, long, default_value = "data/dictionary.txt")]
        dict: PathBuf,
        /// Path to base words file (defaults to config value)
        #[arg(short = 'b', long, default_value = "data/base_words.txt")]
        base_words: PathBuf,
        /// Puzzle as comma-separated words (e.g., "cat,cot,cog,dog")
        #[arg(short, long)]
        puzzle: String,
    },
}

/// Main CLI execution function.
///
/// This function handles the parsed CLI arguments and dispatches to the
/// appropriate command handler. It integrates with the configuration system
/// to provide sensible defaults while respecting user overrides.
///
/// # Arguments
///
/// * `cli` - The parsed command-line arguments
///
/// # Returns
///
/// Returns `Ok(())` on successful execution, or an error if something fails.
///
/// # Examples
///
/// ```rust
/// use wordladder_engine::cli::{Cli, run};
/// use clap::Parser;
///
/// // Parse arguments and run
/// let cli = Cli::parse();
/// run(cli)?;
/// # Ok::<(), anyhow::Error>(())
/// ```
pub fn run(cli: Cli) -> Result<()> {
    let config = Config::default();

    match cli.command {
        Commands::Generate {
            dict,
            base_words,
            start,
            end,
            json,
        } => {
            let dict_path = if dict == PathBuf::from("data/dictionary.txt") {
                config.dictionary_path.clone()
            } else {
                dict
            };
            let base_words_path = if base_words == PathBuf::from("data/base_words.txt") {
                config.base_words_path.clone()
            } else {
                base_words
            };

            let generator = load_generator(dict_path.as_path(), base_words_path.as_path())?;

            // If no specific arguments provided, generate bulk puzzles
            if start.is_none() && end.is_none() && !json {
                generate_bulk_puzzles(&generator, &config)?;
            } else {
                let (start_word, end_word) = if let (Some(s), Some(e)) = (start, end) {
                    (s.to_lowercase(), e.to_lowercase())
                } else {
                    generator.pick_random_words()?
                };

                if let Some(puzzle) = generator.generate_puzzle(&start_word, &end_word) {
                    if json {
                        println!("{}", puzzle.to_json()?);
                    } else {
                        println!("Start: {}", puzzle.start);
                        println!("End: {}", puzzle.end);
                        println!("Path: {}", puzzle.path.join(" -> "));
                        println!("Difficulty: {:?}", puzzle.difficulty);
                    }
                } else {
                    println!("No path found between {} and {}", start_word, end_word);
                }
            }
        }
        Commands::Batch {
            dict,
            base_words,
            count,
            difficulty,
            output,
        } => {
            let dict_path = if dict == PathBuf::from("data/dictionary.txt") {
                config.dictionary_path.clone()
            } else {
                dict
            };
            let base_words_path = if base_words == PathBuf::from("data/base_words.txt") {
                config.base_words_path.clone()
            } else {
                base_words
            };

            let generator = load_generator(dict_path.as_path(), base_words_path.as_path())?;

            let diff = match difficulty.as_str() {
                "easy" => Difficulty::Easy,
                "medium" => Difficulty::Medium,
                "hard" => Difficulty::Hard,
                _ => Difficulty::Medium,
            };

            let puzzles = generator.generate_batch(count, diff);
            let puzzle_count = puzzles.len();

            let mut output_content = String::new();
            for puzzle in puzzles {
                let solution = puzzle.path.join(" -> ");
                output_content.push_str(&format!(
                    "{} -> {}: {}\n",
                    puzzle.start, puzzle.end, solution
                ));
            }

            std::fs::write(&output, output_content)?;
            println!(
                "Generated {} puzzles and saved to {}",
                puzzle_count,
                output.display()
            );
        }
        Commands::Verify {
            dict,
            base_words,
            puzzle,
        } => {
            let dict_path = if dict == PathBuf::from("data/dictionary.txt") {
                config.dictionary_path.clone()
            } else {
                dict
            };
            let base_words_path = if base_words == PathBuf::from("data/base_words.txt") {
                config.base_words_path.clone()
            } else {
                base_words
            };

            let generator = load_generator(dict_path.as_path(), base_words_path.as_path())?;

            match generator.verify_puzzle(&puzzle) {
                Ok(true) => println!("Puzzle is valid"),
                Ok(false) => println!("Puzzle is invalid"),
                Err(e) => println!("Error: {}", e),
            }
        }
    }
    Ok(())
}

/// Loads and initializes a puzzle generator with the specified dictionary files.
///
/// This function creates a new `WordGraph`, loads the dictionary and base words,
/// and returns a configured `PuzzleGenerator` ready for use.
///
/// # Arguments
///
/// * `dict` - Path to the dictionary file
/// * `base_words` - Path to the base words file
///
/// # Returns
///
/// Returns a configured `PuzzleGenerator` or an error if file loading fails.
fn load_generator(dict: &Path, base_words: &Path) -> Result<PuzzleGenerator> {
    let mut graph = WordGraph::new();
    graph.load_dictionary(dict.to_str().unwrap())?;
    graph.load_base_words(base_words.to_str().unwrap())?;
    Ok(PuzzleGenerator::new(graph))
}

/// Generates bulk puzzles for all difficulty levels and saves them to files.
///
/// This function creates three output files (easy.txt, medium.txt, hard.txt)
/// in the configured output directory, each containing the specified number
/// of puzzles for that difficulty level.
///
/// # Arguments
///
/// * `generator` - The puzzle generator to use
/// * `config` - Configuration containing output settings
///
/// # Returns
///
/// Returns `Ok(())` on success, or an error if file operations fail.
fn generate_bulk_puzzles(generator: &PuzzleGenerator, config: &Config) -> Result<()> {
    use std::fs;

    // Create output directory if it doesn't exist
    fs::create_dir_all(&config.output_dir)?;

    let difficulties = vec![
        (Difficulty::Easy, "easy"),
        (Difficulty::Medium, "medium"),
        (Difficulty::Hard, "hard"),
    ];

    for (difficulty, filename) in difficulties {
        let puzzles = generator.generate_batch(config.bulk_puzzle_count, difficulty);
        let puzzle_count = puzzles.len();

        let mut output_content = String::new();
        for puzzle in puzzles {
            let solution = puzzle.path.join(" -> ");
            output_content.push_str(&format!(
                "{} -> {}: {}\n",
                puzzle.start, puzzle.end, solution
            ));
        }

        let output_path = config.output_dir.join(format!("{}.txt", filename));
        fs::write(&output_path, output_content)?;
        println!(
            "Generated {} {} puzzles in {}",
            puzzle_count,
            filename,
            output_path.display()
        );
    }

    Ok(())
}
