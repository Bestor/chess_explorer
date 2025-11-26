# Chess Analyzer

## Summary

A Rust-based tool designed to extract and analyze insights from chess games. This application fetches game data from Chess.com's public API and provides analysis capabilities to help players identify patterns, evaluate positions, and discover areas for improvement in their gameplay.

The tool converts chess positions into analyzable board states, enabling deep analysis of game positions and strategic patterns.

## How to Run

### Prerequisites
- Rust (edition 2024 or later)
- Cargo package manager

### Running the Application

1. Build and run the project:
   ```bash
   cargo run
   ```

2. The application will:
   - Fetch current games from Chess.com for the configured username
   - Convert each game position to a chess board representation
   - Display the board states for analysis

### Configuration

Currently, the username is hardcoded in `main.rs`. To analyze a different player's games, modify the username in the `fetch_games()` call in the main function