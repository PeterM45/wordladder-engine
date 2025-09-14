# Word Ladder Engine

A high-performance Rust CLI application for generating and solving word ladder puzzles. Uses BFS algorithm for optimal path finding and supports configurable difficulty levels with SQL export for mobile game integration.

## 🚀 Features

- **Efficient Word Graph**: Adjacency list representation with BFS for shortest path finding
- **Configurable Difficulty**: Easy (2-3 steps), Medium (4-5 steps), Hard (6-10 steps)
- **Flexible Configuration**: Centralized config system for file paths and settings
- **Dual Dictionary System**: Separate dictionary for path finding and base words for puzzle endpoints
- **Multiple Output Formats**: Text files, JSON, and SQLite-compatible SQL
- **Dictionary Export**: Export dictionary to SQL for O(log n) mobile lookups
- **Mobile Integration**: Direct SQL export for React Native/SQLite applications
- **Comprehensive CLI**: Generate, batch, verify, bulk, mobile-optimized, and dictionary export operations
- **Performance Optimized**: Batched SQL inserts and balanced difficulty distribution

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

# Generate batch of puzzles (defaults to output/batch_medium.txt)
cargo run -- batch --count 50 --difficulty medium

# Generate SQL export for mobile (defaults to output/bulk_puzzles.sql)
cargo run -- generate --format sql

# Generate mobile-optimized puzzles (defaults to output/mobile_puzzles.sql)
cargo run -- generate-mobile --count 1000

# Export dictionary to SQL for mobile apps (defaults to output/dictionary.sql)
cargo run -- export-dict

# Verify puzzle solution
cargo run -- verify --puzzle "cat,cot,cog,dog"
```

## 📁 Project Structure

```
wordladder-engine/
├── src/
│   ├── exporters/       # Export format modules
│   │   └── sql.rs      # SQL export functionality
│   ├── cli.rs          # Command-line interface
│   ├── config.rs       # Configuration management
│   ├── graph.rs        # Word graph and BFS
│   ├── puzzle.rs       # Puzzle generation
│   └── lib.rs          # Library exports
├── data/               # Dictionary files
│   ├── dictionary.txt  # Full word dictionary
│   └── base_words.txt  # Curated puzzle words
├── output/             # Generated puzzle files (default output directory)
└── Cargo.toml         # Project dependencies
```

## ⚙️ Configuration

The application uses sensible defaults but can be customized:

```rust
// Default configuration
dictionary_path: "data/dictionary.txt"
base_words_path: "data/base_words.txt"
output_dir: "output"                    // Default output directory
bulk_puzzle_count: 100
sql_batch_size: 100
include_schema_by_default: true
mobile_difficulty_distribution: {easy: 0.4, medium: 0.4, hard: 0.2}
```

Override defaults with command-line flags:
```bash
cargo run -- generate --dict custom/dict.txt --base-words custom/base.txt
```

**Note**: All generated files are automatically placed in the configured output directory (`output/` by default) when no explicit output path is provided.

## 📖 Usage

### Generate Bulk Puzzles
Creates 100 puzzles each for easy/medium/hard difficulties:
```bash
cargo run -- generate
```
**Output**: `output/easy.txt`, `output/medium.txt`, `output/hard.txt`

### Generate Single Puzzle
```bash
cargo run -- generate --start cat --end dog
cargo run -- generate --start cat --end dog --format json  # JSON output
cargo run -- generate --start cat --end dog --format sql   # SQL output (saved to output/cat_dog.sql)
```

### Generate Batch
```bash
# Generate 50 medium puzzles (defaults to output/batch_medium.txt)
cargo run -- batch --count 50 --difficulty medium

# Generate 100 hard puzzles as JSON (defaults to output/batch_hard.json)
cargo run -- batch --count 100 --difficulty hard --format json

# Generate SQL batch with custom output path
cargo run -- batch --count 100 --difficulty hard --format sql --output custom_hard.sql
```

### Generate Mobile-Optimized Puzzles
Creates balanced puzzle sets optimized for mobile games:
```bash
# Default balanced distribution (40% easy, 40% medium, 20% hard)
# Output: output/mobile_puzzles.sql
cargo run -- generate-mobile --count 1000

# Custom distribution with custom output
cargo run -- generate-mobile --count 5000 --easy-ratio 0.3 --medium-ratio 0.5 --hard-ratio 0.2 --output custom_mobile.sql
```

### Verify Puzzle
```bash
cargo run -- verify --puzzle "cat,cot,cog,dog"
```

### Export Dictionary to SQL
Export dictionary words to SQLite format for efficient mobile lookups:
```bash
# Export dictionary to SQL (defaults to output/dictionary.sql)
cargo run -- export-dict

# Export with custom dictionary and output path
cargo run -- export-dict --dict data/custom_dict.txt --output mobile/dict.sql

# Export without schema (for appending to existing database)
cargo run -- export-dict --include-schema false --batch-size 50
```

### Output Directory Behavior
All commands automatically create the `output/` directory if it doesn't exist. When no output path is specified, files are saved with sensible default names in the output directory. You can override this by providing a custom `--output` path (absolute or relative to the output directory).

## 🎯 Difficulty Levels

- **Easy**: 2-3 steps (short paths)
- **Medium**: 4-5 steps (moderate complexity)
- **Hard**: 6-10 steps (complex puzzles requiring multiple transformations)

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

### SQL Format
```sql
-- Create table schema
CREATE TABLE IF NOT EXISTS puzzles (
    id TEXT PRIMARY KEY,
    start_word TEXT NOT NULL,
    target_word TEXT NOT NULL,
    min_steps INTEGER NOT NULL,
    difficulty TEXT NOT NULL
);

-- Insert puzzle data
INSERT INTO puzzles (id, start_word, target_word, min_steps, difficulty) VALUES
('cat_dog_001', 'CAT', 'DOG', 3, 'easy'),
('fire_gold_002', 'FIRE', 'GOLD', 4, 'medium'),
('black_white_003', 'BLACK', 'WHITE', 9, 'hard');
```

### Dictionary SQL Format
```sql
-- Create dictionary table
CREATE TABLE IF NOT EXISTS dictionary (
    word TEXT PRIMARY KEY,
    length INTEGER NOT NULL
);

-- Indexes for efficient word lookups
CREATE INDEX IF NOT EXISTS idx_dictionary_length ON dictionary(length);

-- Insert dictionary words
INSERT OR IGNORE INTO dictionary (word, length) VALUES
('surveying', 9),
('management', 10),
('saturn', 6),
('pictured', 8);
```

## 📱 Mobile Integration

### React Native Setup

1. **Generate SQL Exports**
```bash
# Export puzzles
cargo run -- generate-mobile --count 1000 --output mobile_puzzles.sql

# Export dictionary for efficient word validation
cargo run -- export-dict --output mobile_dictionary.sql
```

2. **Import into SQLite Database**
```javascript
import SQLite from 'react-native-sqlite-storage';

// Open database
const db = SQLite.openDatabase({name: 'wordladder.db'});

// Execute SQL files
const puzzleSql = require('./mobile_puzzles.sql');
const dictSql = require('./mobile_dictionary.sql');

db.transaction(tx => {
  tx.executeSql(puzzleSql);
  tx.executeSql(dictSql);
});
```

3. **Query Puzzles and Validate Words in App**
```javascript
// Get random easy puzzle
db.transaction(tx => {
  tx.executeSql(
    "SELECT * FROM puzzles WHERE difficulty = 'easy' ORDER BY RANDOM() LIMIT 1",
    [],
    (tx, results) => {
      const puzzle = results.rows.item(0);
      console.log(`Solve: ${puzzle.start_word} -> ${puzzle.target_word}`);
    }
  );
});

// Validate if a word exists (O(log n) lookup)
function isValidWord(word) {
  return new Promise((resolve) => {
    db.transaction(tx => {
      tx.executeSql(
        "SELECT 1 FROM dictionary WHERE word = ? LIMIT 1",
        [word.toLowerCase()],
        (tx, results) => {
          resolve(results.rows.length > 0);
        }
      );
    });
  });
}
```

### Performance Optimization

- **Batch Size**: Adjust `--batch-size` for optimal import performance
- **Schema Control**: Use `--include-schema` to control table creation
- **Balanced Distribution**: `generate-mobile` ensures good gameplay balance
- **Large Datasets**: Supports generating 5000+ puzzles efficiently
- **Dictionary Lookups**: O(log n) SQLite queries vs O(n) text file scanning
- **Memory Efficiency**: No need to load entire dictionary into mobile app memory
- **Indexed Queries**: Fast word validation and length-based filtering

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
- [Mobile Integration](#-mobile-integration) - React Native setup guide

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure all tests pass
5. Submit a pull request

## 📄 License

This project is open source. See LICENSE file for details.

---

**Built with ❤️ in Rust** | **Optimized for Performance** | **Mobile-Ready SQL Export**
