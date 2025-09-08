# Word Ladder Engine

A high-performance Rust CLI application for generating and solving word ladder puzzles. Uses BFS algorithm for optimal path finding and supports configurable difficulty levels with comprehensive puzzle validation.

## 🚀 Features

- **Efficient Word Graph**: Adjacency list representation with BFS for shortest path finding
- **Configurable Difficulty**: Easy (3-4 steps), Medium (5-7 steps), Hard (8+ steps)
- **Flexible Configuration**: Centralized config system for file paths and settings
- **Dual Dictionary System**: Separate dictionary for path finding and base words for puzzle endpoints
- **Multiple Output Formats**: Text files, JSON, and CLI output
- **Comprehensive CLI**: Generate, batch, verify, and bulk operations
- **Async File I/O**: Fast loading of large dictionary files
- **Error Handling**: Robust error handling with detailed messages
- **Unit Tests**: Comprehensive test coverage for core algorithms

## 📁 Project Structure

```
wordladder-engine/
├── src/
│   ├── main.rs          # Application entry point
│   ├── lib.rs           # Module declarations
│   ├── config.rs        # Configuration management
│   ├── cli.rs           # Command-line interface
│   ├── graph.rs         # Word graph and BFS implementation
│   └── puzzle.rs        # Puzzle generation and validation
├── data/
│   ├── dictionary.txt   # Full word dictionary for path finding
│   ├── base_words.txt   # Curated words for puzzle start/end points
│   └── dictionary-2.txt # Alternative dictionary
├── output/              # Generated puzzle files
├── Cargo.toml           # Project dependencies
└── README.md           # This file
```

## ⚙️ Configuration System

The application uses a centralized configuration system defined in `src/config.rs`. All default paths and settings can be customized through the `Config` struct.

### Default Configuration

```rust
Config {
    dictionary_path: "data/dictionary.txt",     // Full word dictionary
    base_words_path: "data/base_words.txt",     // Curated puzzle words
    output_dir: "output",                       // Output directory
    bulk_puzzle_count: 100,                     // Puzzles per difficulty
}
```

### Configuration Builder Pattern

```rust
use wordladder_engine::Config;

let config = Config::new()
    .with_dictionary_path("custom/dict.txt".into())
    .with_base_words_path("custom/base.txt".into())
    .with_output_dir("results".into())
    .with_bulk_puzzle_count(50);
```

## 📋 Requirements

### Data Files

1. **Dictionary File** (`data/dictionary.txt`):
   - Contains all words for path finding
   - One word per line, lowercase, alphabetic only
   - Used to build the word graph and find valid transitions

2. **Base Words File** (`data/base_words.txt`):
   - Curated list of words for puzzle start/end points
   - Subset of dictionary words, typically common words
   - Ensures puzzles use appropriate starting and ending words

### File Format Examples

**dictionary.txt**:
```
cat
dog
bat
rat
mat
hat
hot
dot
dat
```

**base_words.txt**:
```
cat
dog
bat
rat
hat
hot
```

## 🛠️ Installation

### Prerequisites
- Rust 1.70+ (2024 edition)
- Cargo package manager

### Build from Source

```bash
git clone <repository-url>
cd wordladder-engine
cargo build --release
```

### Run Tests

```bash
cargo test
```

## 📖 Usage

### Quick Start

Generate bulk puzzles with default settings:

```bash
cargo run -- generate
```

This creates:
- `output/easy.txt` - 100 easy puzzles
- `output/medium.txt` - 100 medium puzzles
- `output/hard.txt` - 100 hard puzzles

### CLI Commands

#### Generate Bulk Puzzles (Default)

```bash
# Use default config
wordladder-engine generate

# Custom dictionary paths
wordladder-engine generate --dict custom/dict.txt --base-words custom/base.txt
```

#### Generate Single Puzzle

```bash
# Random puzzle
wordladder-engine generate --start cat --end dog

# JSON output
wordladder-engine generate --start cat --end dog --json
```

#### Generate Batch Puzzles

```bash
# 50 medium difficulty puzzles
wordladder-engine batch --count 50 --difficulty medium --output my-puzzles.txt

# Custom files
wordladder-engine batch --dict data/dict.txt --base-words data/base.txt --count 25 --difficulty hard --output hard-puzzles.txt
```

#### Verify Puzzle

```bash
# Verify puzzle sequence
wordladder-engine verify --puzzle "cold,cord,word,warm"

# Custom dictionary
wordladder-engine verify --dict custom/dict.txt --puzzle "cat,dat,dog"
```

### Command Reference

```
wordladder-engine [COMMAND]

Commands:
  generate  Generate puzzles (bulk or single)
  batch     Generate multiple puzzles to file
  verify    Verify puzzle sequence validity
  help      Print help information

Generate Options:
  -d, --dict <FILE>          Dictionary file path [default: data/dictionary.txt]
  -b, --base-words <FILE>    Base words file path [default: data/base_words.txt]
  -s, --start <WORD>         Starting word (optional)
  -e, --end <WORD>           Ending word (optional)
  --json                     Output as JSON

Batch Options:
  -d, --dict <FILE>          Dictionary file path [default: data/dictionary.txt]
  -b, --base-words <FILE>    Base words file path [default: data/base_words.txt]
  -c, --count <NUMBER>       Number of puzzles [default: 10]
  --difficulty <LEVEL>       Difficulty: easy|medium|hard [default: medium]
  -o, --output <FILE>        Output file path

Verify Options:
  -d, --dict <FILE>          Dictionary file path [default: data/dictionary.txt]
  -b, --base-words <FILE>    Base words file path [default: data/base_words.txt]
  -p, --puzzle <SEQUENCE>    Puzzle as comma-separated words
```

## 🎯 Difficulty Levels

- **Easy**: 3-4 word transitions
- **Medium**: 5-7 word transitions
- **Hard**: 8+ word transitions

## 📄 Output Formats

### Text File Format
```
start_word -> end_word: start_word -> intermediate1 -> intermediate2 -> end_word
another_start -> another_end: another_start -> path -> another_end
```

### JSON Format
```json
{
  "start": "cat",
  "end": "dog",
  "path": ["cat", "dat", "dot", "dog"],
  "difficulty": "Easy"
}
```

### CLI Output
```
Start: cat
End: dog
Path: cat -> dat -> dot -> dog
Difficulty: Easy
```

## 🔧 Dependencies

- **clap**: Command-line argument parsing with derive macros
- **serde**: Serialization framework for JSON support
- **tokio**: Asynchronous runtime for file I/O
- **anyhow**: Error handling
- **rand**: Random number generation for puzzle selection

## ⚡ Performance

- **Optimized for 15k-25k word dictionaries**
- **BFS algorithm** ensures shortest paths
- **Async file loading** for large dictionaries
- **Memory efficient** adjacency list representation
- **Fast puzzle generation** with pre-built word graph

## 🧪 Testing

Run the complete test suite:

```bash
cargo test
```

Test specific modules:

```bash
cargo test graph    # Test word graph functionality
cargo test puzzle   # Test puzzle generation
cargo test config   # Test configuration system
```

## 🔄 Development

### Adding New Features

1. **Configuration**: Add new fields to `Config` struct in `src/config.rs`
2. **CLI Commands**: Extend `Commands` enum in `src/cli.rs`
3. **Core Logic**: Implement in appropriate module (`graph.rs`, `puzzle.rs`)
4. **Tests**: Add unit tests for new functionality

### Code Quality

- Uses Rust 2024 edition
- Comprehensive error handling with `anyhow`
- Async file operations with `tokio`
- Builder pattern for configuration
- Modular architecture with clear separation of concerns

## 📝 Examples

### Custom Configuration

```rust
// Programmatic config (for library usage)
let config = Config::new()
    .with_dictionary_path("large_dict.txt".into())
    .with_bulk_puzzle_count(500);

// CLI usage with custom paths
wordladder-engine generate --dict my_dict.txt --base-words my_base.txt
```

### Batch Processing

```bash
# Generate 1000 easy puzzles
wordladder-engine batch --count 1000 --difficulty easy --output easy_challenge.txt

# Verify multiple puzzles
echo "cat,dat,dog" | xargs -I {} wordladder-engine verify --puzzle {}
```

### Integration

```bash
# Generate puzzles and process with other tools
wordladder-engine generate
cat output/easy.txt | grep "cat" | head -5
```

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
