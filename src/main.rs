mod clients;

use serde_json::Value;
use pleco::Board;
use clients::chess_com::fetch_games;

fn convert_game_to_board(fen: &str) -> Result<Board, Box<dyn std::error::Error>> {
    let board = Board::from_fen(fen)
        .map_err(|e| format!("Failed to parse FEN: {:?}", e))?;
    Ok(board)
}

#[tokio::main]
async fn main() {

    // Fetch game data
    match fetch_games("Bestor0").await {
        Ok(data) => {
            println!("Fetched data successfully\n");
            
            // Get the games array from the JSON
            if let Some(games) = data["games"].as_array() {
                println!("Found {} games\n", games.len());
                
                // Loop through each game and convert one-by-one
                for (i, game) in games.iter().enumerate() {
                    println!("=== Processing Game {} ===", i + 1);
                    
                    // Get the FEN string from the game
                    if let Some(fen) = game["fen"].as_str() {
                        // Convert this game to a board
                        match convert_game_to_board(fen) {
                            Ok(board) => {
                                println!("Successfully created board for game {}", i + 1);
                                println!("{}", board);
                            }
                            Err(e) => {
                                eprintln!("Error converting game {}: {}", i + 1, e);
                            }
                        }
                    } else {
                        eprintln!("No FEN string found for game {}", i + 1);
                    }
                    println!(); // Blank line between games
                }
            } else {
                eprintln!("No games found in response");
            }
        }
        Err(e) => eprintln!("Error fetching data: {}", e),
    }
}
