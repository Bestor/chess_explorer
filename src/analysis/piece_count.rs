use pleco::Board;
use super::{GameAnalyzer, AnalyzeResults};

/// Example analyzer that counts total pieces across all boards
pub struct PieceCountAnalyzer;

impl GameAnalyzer for PieceCountAnalyzer {
    fn analyze(&self, boards: &[Board]) -> AnalyzeResults {
        if boards.is_empty() {
            return AnalyzeResults {
                description: "No boards to analyze".to_string(),
            };
        }
        
        let total_pieces: usize = boards
            .iter()
            .map(|board| board.count_all_pieces() as usize)
            .sum();
        
        let avg_pieces = total_pieces as f64 / boards.len() as f64;
        
        AnalyzeResults {
            description: format!(
                "Piece Count Analysis:\n\
                 - Total boards analyzed: {}\n\
                 - Average pieces per position: {:.1}",
                boards.len(),
                avg_pieces
            ),
        }
    }
    
    fn name(&self) -> &str {
        "Piece Count Analyzer"
    }
}
