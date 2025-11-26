use pleco::Board;

pub mod piece_count;

/// Result of an analysis containing insights
pub struct AnalyzeResults {
    pub description: String,
}

/// Trait that all chess game analyzers must implement
pub trait GameAnalyzer {
    /// Analyze a list of chess boards and return insights
    fn analyze(&self, boards: &[Board]) -> AnalyzeResults;
    
    /// Get the name of this analyzer
    fn name(&self) -> &str;
}

/// Runs multiple analyzers on a set of boards
pub fn run_analyses(boards: &[Board], analyzers: &[Box<dyn GameAnalyzer>]) -> Vec<AnalyzeResults> {
    analyzers
        .iter()
        .map(|analyzer| {
            println!("\n--- Running {} ---", analyzer.name());
            analyzer.analyze(boards)
        })
        .collect()
}
