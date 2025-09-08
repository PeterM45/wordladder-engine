use crate::config::Config;
use crate::graph::WordGraph;
use crate::puzzle::{Difficulty, PuzzleGenerator};
use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(name = "wordladder-engine")]
#[command(about = "A CLI tool for generating word ladder puzzles")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate a single puzzle
    Generate {
        /// Path to dictionary file
        #[arg(short, long, default_value = "data/dictionary.txt")]
        dict: PathBuf,
        /// Path to base words file
        #[arg(short = 'b', long, default_value = "data/base_words.txt")]
        base_words: PathBuf,
        /// Starting word (optional, will pick random if not provided)
        #[arg(short, long)]
        start: Option<String>,
        /// Ending word (optional, will pick random if not provided)
        #[arg(short, long)]
        end: Option<String>,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Generate a batch of puzzles
    Batch {
        /// Path to dictionary file
        #[arg(short, long, default_value = "data/dictionary.txt")]
        dict: PathBuf,
        /// Path to base words file
        #[arg(short = 'b', long, default_value = "data/base_words.txt")]
        base_words: PathBuf,
        /// Number of puzzles to generate
        #[arg(short, long, default_value = "10")]
        count: usize,
        /// Difficulty level
        #[arg(short, long, default_value = "medium")]
        difficulty: String,
        /// Output file for puzzles (required)
        #[arg(short, long)]
        output: PathBuf,
    },
    /// Verify a puzzle
    Verify {
        /// Path to dictionary file
        #[arg(short, long, default_value = "data/dictionary.txt")]
        dict: PathBuf,
        /// Path to base words file
        #[arg(short = 'b', long, default_value = "data/base_words.txt")]
        base_words: PathBuf,
        /// Puzzle as comma-separated words
        #[arg(short, long)]
        puzzle: String,
    },
}

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

fn load_generator(dict: &Path, base_words: &Path) -> Result<PuzzleGenerator> {
    let mut graph = WordGraph::new();
    graph.load_dictionary(dict.to_str().unwrap())?;
    graph.load_base_words(base_words.to_str().unwrap())?;
    Ok(PuzzleGenerator::new(graph))
}

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
