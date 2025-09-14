-- Create puzzles table
CREATE TABLE IF NOT EXISTS puzzles (
	id TEXT PRIMARY KEY,
	start_word TEXT NOT NULL,
	target_word TEXT NOT NULL,
	min_steps INTEGER NOT NULL,
	difficulty TEXT NOT NULL
);

-- Indexes for better query performance
CREATE INDEX IF NOT EXISTS idx_puzzles_difficulty ON puzzles(difficulty);
CREATE INDEX IF NOT EXISTS idx_puzzles_steps ON puzzles(min_steps);

-- Generated 0 puzzles

