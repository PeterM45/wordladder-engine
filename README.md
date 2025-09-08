# Word Ladder Engine

A high-performance Rust CLI application for generating and solving word ladder puzzles. Uses BFS algorithm for optimal path finding and supports configurable difficulty levels.

## 🚀 Features

- **Efficient Word Graph**: Adjacency list representation with BFS for shortest path finding
- **Configurable Difficulty**: Easy (3-4 steps), Medium (5-7 steps), Hard (8+ steps)
- **Flexible Configuration**: Centralized config system for file paths and settings
- **Dual Dictionary System**: Separate dictionary for path finding and base words for puzzle endpoints
- **Multiple Output Formats**: Text files, JSON, and CLI output
- **Comprehensive CLI**: Generate, batch, verify, and bulk operations

## 📦 Quick Start

```bash
# Install Rust and clone
git clone https://github.com/PeterM45/wordladder-engine
cd wordladder-engine
cargo build --release

# Generate bulk puzzles (creates easy/medium/hard.txt in output/)
cargo run -- generate

# Generate single puzzle
cargo run -- generate --start cat --end dog

# Generate batch of puzzles
cargo run -- batch --count 50 --difficulty medium --output puzzles.txt

# Verify puzzle solution
cargo run -- verify --puzzle "cat,cot,cog,dog"
```

## 📁 Project Structure

```
wordladder-engine/
├── src/                 # Source code
├── data/               # Dictionary files
│   ├── dictionary.txt  # Full word dictionary
│   └── base_words.txt  # Curated puzzle words
├── output/             # Generated puzzle files
└── Cargo.toml         # Project dependencies
```

## ⚙️ Configuration

The application uses sensible defaults but can be customized:

```rust
// Default configuration
dictionary_path: "data/dictionary.txt"
base_words_path: "data/base_words.txt"
output_dir: "output"
bulk_puzzle_count: 100
```

Override defaults with command-line flags:
```bash
cargo run -- generate --dict custom/dict.txt --base-words custom/base.txt
```

## 📖 Usage

### Generate Bulk Puzzles
Creates 100 puzzles each for easy/medium/hard difficulties:
```bash
cargo run -- generate
```

### Generate Single Puzzle
```bash
cargo run -- generate --start cat --end dog
cargo run -- generate --start cat --end dog --json  # JSON output
```

### Generate Batch
```bash
cargo run -- batch --count 50 --difficulty medium --output puzzles.txt
```

### Verify Puzzle
```bash
cargo run -- verify --puzzle "cat,cot,cog,dog"
```

## 🎯 Difficulty Levels

- **Easy**: 3-4 word changes
- **Medium**: 5-7 word changes
- **Hard**: 8+ word changes

## 📄 Output Formats

### Text Format
```
start_word -> end_word: start_word -> intermediate -> end_word
```

### JSON Format
```json
{
  "start": "cat",
  "end": "dog",
  "path": ["cat", "cot", "cog", "dog"],
  "difficulty": "Easy"
}
```

## 🛠️ Development

### Prerequisites
- Rust 1.70+ (2024 edition)

### Build & Test
```bash
cargo build --release
cargo test
cargo doc --open  # View documentation
```

### Dependencies
- `clap`: Command-line argument parsing
- `serde`: Serialization for JSON support
- `tokio`: Async file I/O
- `anyhow`: Error handling
- `rand`: Random puzzle selection

## 📚 Documentation

- **Local API Documentation** - Run `cargo doc --open` to view generated docs
- [Configuration Guide](#configuration) - Advanced configuration options

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure all tests pass
5. Submit a pull request

## 📄 License

This project is open source. See LICENSE file for details.

---

**Built with ❤️ in Rust** | **Optimized for Performance** | **CLI-First Design**
