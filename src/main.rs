mod clients;
mod analysis;

use pleco::Board;
use clients::chess_com::fetch_games;
use analysis::GameAnalyzer;
use analysis::piece_count::PieceCountAnalyzer;

fn convert_game_to_board(fen: &str) -> Result<Board, Box<dyn std::error::Error>> {
    let board = Board::from_fen(fen)
        .map_err(|e| format!("Failed to parse FEN: {:?}", e))?;
    Ok(board)
}

#[tokio::main]
async fn main() {

    // Fetch game data for a date range (format: "YYYY/MM")
    match fetch_games("Bestor0", "2025/11", "2025/11").await {
        Ok(data) => {
            println!("Fetched data successfully\n");
            
            // Get the games array from the JSON
            if let Some(games) = data["games"].as_array() {
                println!("Found {} games\n", games.len());
                
                let mut boards = Vec::new();
                
                // Loop through each game and convert one-by-one
                for (i, game) in games.iter().enumerate() {                    
                    // Get the FEN string from the game
                    if let Some(fen) = game["fen"].as_str() {
                        // Convert this game to a board
                        match convert_game_to_board(fen) {
                            Ok(board) => {
                                boards.push(board);
                            }
                            Err(e) => {
                                eprintln!("Error converting game {}: {}", i + 1, e);
                            }
                        }
                    } else {
                        eprintln!("No FEN string found for game {}", i + 1);
                    }
                }
                
                // Run analyses on all boards
                if !boards.is_empty() {
                    println!("\n========== ANALYSIS ==========\n");
                    
                    let analyzers: Vec<Box<dyn GameAnalyzer>> = vec![
                        Box::new(PieceCountAnalyzer),
                    ];
                    
                    let results = analysis::run_analyses(&boards, &analyzers);
                    
                    for result in results {
                        println!("{}\n", result.description);
                    }
                }
            } else {
                eprintln!("No games found in response");
            }
        }
        Err(e) => eprintln!("Error fetching data: {}", e),
    }
}
