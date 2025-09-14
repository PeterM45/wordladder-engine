//! # Command-Line Interface
//!
//! This module implements the command-line interface for the word ladder engine.
//! It defines the CLI structure, command parsing, and execution logic.
//!
//! ## Commands
//!
//! The application supports four main commands:
//!
//! - `generate`: Generate puzzles (bulk or single with arguments)
//! - `batch`: Generate multiple puzzles of specified difficulty to a file
//! - `generate-mobile`: Generate balanced puzzles optimized for mobile apps
//! - `verify`: Verify puzzle sequence validity
//!
//! ## Output Formats
//!
//! The application supports multiple output formats:
//!
//! - `text`: Human-readable text format (default)
//! - `json`: JSON format for programmatic consumption
//! - `sql`: SQLite-compatible SQL format for mobile integration
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
//! // Generate SQL export for mobile
//! wordladder-engine generate --format sql --output puzzles.sql
//!
//! // Generate mobile-optimized puzzles
//! wordladder-engine generate-mobile --count 1000 --output mobile_puzzles.sql
//!
//! // Verify a puzzle solution
//! wordladder-engine verify --puzzle "cat,cot,cog,dog"
//! ```

use crate::config::Config;
use crate::exporters::sql::{SqlExportConfig, SqlExporter};
use crate::graph::WordGraph;
use crate::puzzle::{Difficulty, PuzzleGenerator};
use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use std::path::{Path, PathBuf};

/// Output format for generated puzzles.
#[derive(Debug, Clone, ValueEnum)]
pub enum OutputFormat {
    /// Human-readable text format (default)
    Text,
    /// JSON format for programmatic consumption
    Json,
    /// SQLite-compatible SQL format for mobile integration
    Sql,
}

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
    /// - Output results in text, JSON, or SQL format
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
        /// Output format: text, json, or sql
        #[arg(short, long, default_value = "text")]
        format: OutputFormat,
        /// Output file path (optional, defaults to output/ directory)
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Include CREATE TABLE schema in SQL output
        #[arg(long)]
        include_schema: Option<bool>,
        /// Batch size for SQL INSERT statements
        #[arg(long, default_value = "100")]
        batch_size: usize,
    },
    /// Generate multiple puzzles of specified difficulty to a file
    ///
    /// Creates a batch of puzzles with consistent difficulty and saves them
    /// to a file. Supports text, JSON, and SQL output formats.
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
        #[arg(long, default_value = "medium")]
        difficulty: String,
        /// Output format: text, json, or sql
        #[arg(short, long, default_value = "text")]
        format: OutputFormat,
        /// Output file path (optional, defaults to output/ directory)
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Include CREATE TABLE schema in SQL output
        #[arg(long)]
        include_schema: Option<bool>,
        /// Batch size for SQL INSERT statements
        #[arg(long, default_value = "100")]
        batch_size: usize,
    },
    /// Generate balanced puzzles optimized for mobile applications
    ///
    /// Creates a balanced set of puzzles with configurable difficulty distribution
    /// and exports them in SQLite-compatible SQL format for direct mobile integration.
    GenerateMobile {
        /// Path to dictionary file (defaults to config value)
        #[arg(short, long, default_value = "data/dictionary.txt")]
        dict: PathBuf,
        /// Path to base words file (defaults to config value)
        #[arg(short = 'b', long, default_value = "data/base_words.txt")]
        base_words: PathBuf,
        /// Total number of puzzles to generate
        #[arg(short, long, default_value = "1000")]
        count: usize,
        /// Output file path for the SQL export (optional, defaults to output/ directory)
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Ratio of easy puzzles (0.0 to 1.0)
        #[arg(long, default_value = "0.4")]
        easy_ratio: f64,
        /// Ratio of medium puzzles (0.0 to 1.0)
        #[arg(long, default_value = "0.4")]
        medium_ratio: f64,
        /// Ratio of hard puzzles (0.0 to 1.0)
        #[arg(long, default_value = "0.2")]
        hard_ratio: f64,
        /// Include CREATE TABLE schema in SQL output
        #[arg(long)]
        include_schema: Option<bool>,
        /// Batch size for SQL INSERT statements
        #[arg(long, default_value = "100")]
        batch_size: usize,
    },
    /// Export dictionary to SQL format for mobile applications
    ///
    /// Creates a SQLite-compatible SQL file containing all dictionary words
    /// with proper indexing for efficient lookups (O(log n) performance).
    ExportDict {
        /// Path to dictionary file (defaults to config value)
        #[arg(short, long, default_value = "data/dictionary.txt")]
        dict: PathBuf,
        /// Output file path for the SQL export (optional, defaults to output/ directory)
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Include CREATE TABLE schema in SQL output
        #[arg(long)]
        include_schema: Option<bool>,
        /// Batch size for SQL INSERT statements
        #[arg(long, default_value = "100")]
        batch_size: usize,
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

/// Resolves the output path, providing a default if none is specified.
///
/// If no output path is provided, generates a default filename based on the format
/// and places it in the output directory. If a relative path is provided, resolves
/// it relative to the output directory.
///
/// # Arguments
///
/// * `output` - Optional user-specified output path
/// * `config` - Configuration containing output directory
/// * `format` - Output format for default filename generation
/// * `default_name` - Default filename prefix (without extension)
///
/// # Returns
///
/// Returns the resolved output path.
fn resolve_output_path(
    output: Option<PathBuf>,
    config: &Config,
    format: &OutputFormat,
    default_name: &str,
) -> Result<PathBuf> {
    use std::fs;

    let output_path = match output {
        Some(path) => {
            // If it's an absolute path, use it as-is
            if path.is_absolute() {
                path
            } else {
                // If it's relative, resolve it relative to the output directory
                config.output_dir.join(path)
            }
        }
        _ => {
            // Generate default filename based on format
            let extension = match format {
                OutputFormat::Text => "txt",
                OutputFormat::Json => "json",
                OutputFormat::Sql => "sql",
            };
            config
                .output_dir
                .join(format!("{}.{}", default_name, extension))
        }
    };

    // Ensure the parent directory exists
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }

    Ok(output_path)
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
/// use std::ffi::OsString;
///
/// // Parse arguments and run (example with help command)
/// let args = vec![OsString::from("wordladder-engine"), OsString::from("--help")];
/// let cli = Cli::parse_from(args);
/// // Note: This would normally run the CLI, but we skip execution in doctest
/// ```
pub fn run(cli: Cli) -> Result<()> {
    let config = Config::default();

    match cli.command {
        Commands::Generate {
            dict,
            base_words,
            start,
            end,
            format,
            output,
            include_schema,
            batch_size,
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
            if start.is_none() && end.is_none() {
                match format {
                    OutputFormat::Sql => {
                        let output_path =
                            resolve_output_path(output, &config, &format, "bulk_puzzles")?;
                        generate_bulk_sql(
                            &generator,
                            &config,
                            &output_path,
                            include_schema.unwrap_or(config.include_schema_by_default),
                            batch_size,
                        )?;
                    }
                    _ => generate_bulk_puzzles(&generator, &config, &format)?,
                }
            } else {
                let (start_word, end_word) = if let (Some(s), Some(e)) = (start, end) {
                    (s.to_lowercase(), e.to_lowercase())
                } else {
                    generator.pick_random_words()?
                };

                if let Some(puzzle) = generator.generate_puzzle(&start_word, &end_word) {
                    match format {
                        OutputFormat::Json => {
                            println!("{}", puzzle.to_json()?);
                        }
                        OutputFormat::Sql => {
                            let output_path = resolve_output_path(
                                output,
                                &config,
                                &format,
                                &format!("{}_{}", start_word, end_word),
                            )?;
                            let sql_config = SqlExportConfig {
                                batch_size,
                                include_schema: include_schema
                                    .unwrap_or(config.include_schema_by_default),
                                include_comments: true,
                            };
                            let mut exporter = SqlExporter::with_config(sql_config);
                            let sql = exporter.export_puzzles(&[puzzle])?;
                            std::fs::write(&output_path, sql)?;
                            println!("SQL puzzle exported to {}", output_path.display());
                        }
                        OutputFormat::Text => {
                            println!("Start: {}", puzzle.start);
                            println!("End: {}", puzzle.end);
                            println!("Path: {}", puzzle.path.join(" -> "));
                            println!("Difficulty: {:?}", puzzle.difficulty);
                        }
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
            format,
            output,
            include_schema,
            batch_size,
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

            let output_path =
                resolve_output_path(output, &config, &format, &format!("batch_{}", difficulty))?;

            match format {
                OutputFormat::Sql => {
                    let sql_config = SqlExportConfig {
                        batch_size,
                        include_schema: include_schema.unwrap_or(config.include_schema_by_default),
                        include_comments: true,
                    };
                    let mut exporter = SqlExporter::with_config(sql_config);
                    let sql = exporter.export_puzzles(&puzzles)?;
                    std::fs::write(&output_path, sql)?;
                    println!(
                        "Generated {} SQL puzzles and saved to {}",
                        puzzle_count,
                        output_path.display()
                    );
                }
                OutputFormat::Json => {
                    let json_array: Result<Vec<_>, _> =
                        puzzles.iter().map(|p| p.to_json()).collect();
                    let json_array = json_array?;
                    let json_output = format!("[\n{}\n]", json_array.join(",\n"));
                    std::fs::write(&output_path, json_output)?;
                    println!(
                        "Generated {} JSON puzzles and saved to {}",
                        puzzle_count,
                        output_path.display()
                    );
                }
                OutputFormat::Text => {
                    let mut output_content = String::new();
                    for puzzle in puzzles {
                        let solution = puzzle.path.join(" -> ");
                        output_content.push_str(&format!(
                            "{} -> {}: {}\n",
                            puzzle.start, puzzle.end, solution
                        ));
                    }
                    std::fs::write(&output_path, output_content)?;
                    println!(
                        "Generated {} text puzzles and saved to {}",
                        puzzle_count,
                        output_path.display()
                    );
                }
            }
        }
        Commands::GenerateMobile {
            dict,
            base_words,
            count,
            output,
            easy_ratio,
            medium_ratio,
            hard_ratio,
            include_schema,
            batch_size,
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

            // Generate all possible puzzles first
            println!("Generating base puzzles for mobile optimization...");
            let all_puzzles = generate_all_puzzles_for_mobile(&generator, &config)?;
            println!("Generated {} base puzzles", all_puzzles.len());

            // Create balanced set
            let sql_config = SqlExportConfig {
                batch_size,
                include_schema: include_schema.unwrap_or(config.include_schema_by_default),
                include_comments: true,
            };
            let exporter = SqlExporter::with_config(sql_config.clone());
            let balanced_puzzles = exporter.create_balanced_set(
                &all_puzzles,
                count,
                easy_ratio,
                medium_ratio,
                hard_ratio,
            );

            // Export to SQL
            let output_path =
                resolve_output_path(output, &config, &OutputFormat::Sql, "mobile_puzzles")?;
            let mut sql_exporter = SqlExporter::with_config(sql_config);
            let sql = sql_exporter.export_puzzles(&balanced_puzzles)?;
            std::fs::write(&output_path, sql)?;

            println!(
                "Generated {} balanced mobile puzzles and saved to {}",
                balanced_puzzles.len(),
                output_path.display()
            );
            println!(
                "Distribution: Easy: {:.1}%, Medium: {:.1}%, Hard: {:.1}%",
                easy_ratio * 100.0,
                medium_ratio * 100.0,
                hard_ratio * 100.0
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
        Commands::ExportDict {
            dict,
            output,
            include_schema,
            batch_size,
        } => {
            let dict_path = if dict == PathBuf::from("data/dictionary.txt") {
                config.dictionary_path.clone()
            } else {
                dict
            };

            // Load the dictionary
            let mut graph = WordGraph::new();
            graph.load_dictionary(dict_path.to_str().unwrap())?;
            let words = graph.get_words();

            // Export to SQL
            let output_path =
                resolve_output_path(output, &config, &OutputFormat::Sql, "dictionary")?;
            let sql_config = SqlExportConfig {
                batch_size,
                include_schema: include_schema.unwrap_or(config.include_schema_by_default),
                include_comments: true,
            };
            let mut exporter = SqlExporter::with_config(sql_config);
            let sql = exporter.export_dictionary(words)?;
            std::fs::write(&output_path, sql)?;

            println!(
                "Exported {} dictionary words to {}",
                words.len(),
                output_path.display()
            );
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
/// * `format` - Output format (Text or Json)
///
/// # Returns
///
/// Returns `Ok(())` on success, or an error if file operations fail.
fn generate_bulk_puzzles(
    generator: &PuzzleGenerator,
    config: &Config,
    format: &OutputFormat,
) -> Result<()> {
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

        match format {
            OutputFormat::Json => {
                let json_array: Result<Vec<_>, _> = puzzles.iter().map(|p| p.to_json()).collect();
                let json_array = json_array?;
                let output_content = format!("[\n{}\n]", json_array.join(",\n"));
                let output_path = config.output_dir.join(format!("{}.json", filename));
                fs::write(&output_path, output_content)?;
                println!(
                    "Generated {} {} puzzles in {}",
                    puzzle_count,
                    filename,
                    output_path.display()
                );
            }
            OutputFormat::Text => {
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
            OutputFormat::Sql => {
                // This should not happen as SQL format is handled separately
                return Err(anyhow::anyhow!(
                    "SQL format should be handled by generate_bulk_sql"
                ));
            }
        }
    }

    Ok(())
}

/// Generates bulk puzzles and exports them to a single SQL file.
///
/// This function creates a single SQL file containing all puzzles from all
/// difficulty levels, optimized for mobile application consumption.
///
/// # Arguments
///
/// * `generator` - The puzzle generator to use
/// * `config` - Configuration containing output settings
/// * `output_path` - Path to the output SQL file
/// * `include_schema` - Whether to include CREATE TABLE statement
/// * `batch_size` - Batch size for INSERT statements
///
/// # Returns
///
/// Returns `Ok(())` on success, or an error if file operations fail.
fn generate_bulk_sql(
    generator: &PuzzleGenerator,
    config: &Config,
    output_path: &Path,
    include_schema: bool,
    batch_size: usize,
) -> Result<()> {
    use std::fs;

    let difficulties = vec![Difficulty::Easy, Difficulty::Medium, Difficulty::Hard];

    let mut all_puzzles = Vec::new();

    for difficulty in difficulties {
        let puzzles = generator.generate_batch(config.bulk_puzzle_count, difficulty);
        all_puzzles.extend(puzzles);
    }

    let sql_config = SqlExportConfig {
        batch_size,
        include_schema,
        include_comments: true,
    };
    let mut exporter = SqlExporter::with_config(sql_config);
    let sql = exporter.export_puzzles(&all_puzzles)?;

    fs::write(output_path, sql)?;
    println!(
        "Generated {} puzzles in SQL format to {}",
        all_puzzles.len(),
        output_path.display()
    );

    Ok(())
}

/// Generates all possible puzzles for mobile optimization.
///
/// This function creates a comprehensive set of puzzles across all difficulty
/// levels to provide a good base for creating balanced mobile puzzle sets.
///
/// # Arguments
///
/// * `generator` - The puzzle generator to use
/// * `config` - Configuration containing generation settings
///
/// # Returns
///
/// Returns a vector of all generated puzzles.
fn generate_all_puzzles_for_mobile(
    generator: &PuzzleGenerator,
    config: &Config,
) -> Result<Vec<crate::puzzle::Puzzle>> {
    let difficulties = vec![Difficulty::Easy, Difficulty::Medium, Difficulty::Hard];

    let mut all_puzzles = Vec::new();

    for difficulty in difficulties {
        let puzzles = generator.generate_batch(config.bulk_puzzle_count * 2, difficulty); // Generate more for better selection
        all_puzzles.extend(puzzles);
    }

    Ok(all_puzzles)
}
