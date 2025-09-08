//! # Word Ladder Engine - Main Entry Point
//!
//! This is the main entry point for the word ladder engine CLI application.
//! It parses command-line arguments and delegates to the appropriate command handler.
//!
//! ## Application Overview
//!
//! The word ladder engine is a high-performance Rust application that generates
//! and solves word ladder puzzles. It uses efficient graph algorithms to find
//! shortest paths between words and supports configurable difficulty levels.
//!
//! ## Usage
//!
//! Run the application with `--help` to see available commands:
//!
//! ```bash
//! cargo run -- --help
//! ```
//!
//! ## Architecture
//!
//! The application follows a modular architecture:
//! - `main.rs`: Entry point and CLI argument parsing
//! - `cli.rs`: Command-line interface and command handlers
//! - `config.rs`: Configuration management
//! - `graph.rs`: Word graph and BFS implementation
//! - `puzzle.rs`: Puzzle generation and validation
//!
//! ## Error Handling
//!
//! The application uses `anyhow` for comprehensive error handling and provides
//! user-friendly error messages for common issues like missing files or invalid input.

use anyhow::Result;
use clap::Parser;
use wordladder_engine::cli::{Cli, run};

/// Main entry point for the word ladder engine.
///
/// This function:
/// 1. Parses command-line arguments using clap
/// 2. Delegates execution to the CLI module
/// 3. Handles any errors that occur during execution
///
/// # Returns
///
/// Returns `Ok(())` on successful execution, or an error if something goes wrong.
fn main() -> Result<()> {
    let cli = Cli::parse();
    run(cli)
}
