use reqwest;
use serde_json::Value;
use std::fs;
use std::path::PathBuf;

const CACHE_DIR_ARCHIVES: &str = "/tmp/chess/analyzer/cache/archives";
const CACHE_DIR_GAMES: &str = "/tmp/chess/analyzer/cache/games";

/// Fetches all available archive URLs for a player
async fn fetch_archives(username: &str, client: &reqwest::Client) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    // Create cache directory path
    let cache_dir = PathBuf::from(CACHE_DIR_ARCHIVES);
    fs::create_dir_all(&cache_dir)?;
    
    // Create cache file path for this user
    let cache_filename = format!("{}_archives.json", username);
    let cache_path = cache_dir.join(cache_filename);
    
    // Check if cached file exists
    if cache_path.exists() {
        println!("Loading archives list from cache: {}", cache_path.display());
        let cached_content = fs::read_to_string(&cache_path)?;
        let data: Value = serde_json::from_str(&cached_content)?;
        
        // Extract the archives array
        if let Some(archives) = data["archives"].as_array() {
            let urls: Vec<String> = archives
                .iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect();
            return Ok(urls);
        } else {
            return Err("No archives found in cached response".into());
        }
    }
    
    // Not in cache, fetch from API
    let url = format!("https://api.chess.com/pub/player/{}/games/archives", username);
    println!("Fetching archives list: {}", url);
    
    let response = client
        .get(&url)
        .header("User-Agent", "Rust Chess Analyzer/1.0")
        .send()
        .await?;
    
    let status = response.status();
    if !status.is_success() {
        return Err(format!("API returned error status: {}", status).into());
    }
    
    let data: Value = response.json().await?;
    
    // Save to cache
    let cache_content = serde_json::to_string_pretty(&data)?;
    fs::write(&cache_path, cache_content)?;
    println!("Cached archives list to: {}", cache_path.display());
    
    // Extract the archives array
    if let Some(archives) = data["archives"].as_array() {
        let urls: Vec<String> = archives
            .iter()
            .filter_map(|v| v.as_str().map(String::from))
            .collect();
        Ok(urls)
    } else {
        Err("No archives found in response".into())
    }
}

/// Fetches games from a specific archive URL
async fn fetch_games_from_archive(url: &str, client: &reqwest::Client) -> Result<Value, Box<dyn std::error::Error>> {
    // Extract the date part (YYYY/MM) from URL for cache filename
    let date_part = url
        .split("/games/")
        .nth(1)
        .ok_or("Invalid archive URL format")?;
    
    // Create cache directory path
    let cache_dir = PathBuf::from(CACHE_DIR_GAMES);
    fs::create_dir_all(&cache_dir)?;
    
    // Create cache file path (replace / with - for filename)
    let cache_filename = format!("{}.json", date_part.replace('/', "-"));
    let cache_path = cache_dir.join(cache_filename);
    
    // Check if cached file exists
    if cache_path.exists() {
        println!("Loading games from cache: {}", cache_path.display());
        let cached_content = fs::read_to_string(&cache_path)?;
        let data: Value = serde_json::from_str(&cached_content)?;
        return Ok(data);
    } else {
        // Not in cache, fetch from API
        println!("Fetching games from: {}", url);
        
        let response = client
            .get(url)
            .header("User-Agent", "Rust Chess Analyzer/1.0")
            .send()
            .await?;
        
        let status = response.status();
        if !status.is_success() {
            return Err(format!("API returned error status: {}", status).into());
        }
        
        let data: Value = response.json().await?;
        
        // Save to cache
        let cache_content = serde_json::to_string_pretty(&data)?;
        fs::write(&cache_path, cache_content)?;
        println!("Cached games to: {}", cache_path.display());
        
        Ok(data)
    }
    

}

/// Fetches games for a player within a date range (year/month format: "YYYY/MM")
/// start_date and end_date should be in format "YYYY/MM" (e.g., "2024/01")
pub async fn fetch_games(
    username: &str, 
    start_date: &str, 
    end_date: &str
) -> Result<Value, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    
    // Fetch all available archives
    let all_archives = fetch_archives(username, &client).await?;
    
    // Filter archives within the date range
    let filtered_archives: Vec<String> = all_archives
        .into_iter()
        .filter(|url| {
            // Extract the date part from URL (e.g., "2024/01" from ".../games/2024/01")
            if let Some(date_part) = url.split("/games/").nth(1) {
                date_part >= start_date && date_part <= end_date
            } else {
                false
            }
        })
        .collect();
    
    println!("Found {} archives in date range {}-{}", 
             filtered_archives.len(), start_date, end_date);
    
    // Fetch games from all filtered archives
    let mut all_games = Vec::new();
    
    for archive_url in filtered_archives {
        match fetch_games_from_archive(&archive_url, &client).await {
            Ok(data) => {
                if let Some(games) = data["games"].as_array() {
                    all_games.extend(games.clone());
                    println!("Fetched {} games from {}", games.len(), archive_url);
                }
            }
            Err(e) => {
                eprintln!("Error fetching archive {}: {}", archive_url, e);
            }
        }
    }
    
    // Return all games in the same format as before
    Ok(serde_json::json!({
        "games": all_games
    }))
}
