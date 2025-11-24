use reqwest;
use serde_json::Value;

pub async fn fetch_games(username: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let url = format!("https://api.chess.com/pub/player/{}/games", username);
    println!("Fetching: {}", url);
    
    // Create a client with User-Agent header (required by chess.com API)
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "Rust Chess Analyzer/1.0")
        .send()
        .await?;
    
    // Check if the response was successful
    let status = response.status();
    println!("Response status: {}", status);
    
    if !status.is_success() {
        return Err(format!("API returned error status: {}", status).into());
    }
    
    // Get the response text first to see what we're getting
    let text = response.text().await?;
    println!("Response body: {}", text);
    
    // Parse the text as JSON
    let data: Value = serde_json::from_str(&text)?;

    Ok(data)
}
