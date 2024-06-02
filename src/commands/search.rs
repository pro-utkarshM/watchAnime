use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize)]
struct Anime {
    // Define fields according to your API response
    title: String,
    url: String,
    // Add other relevant fields
}

pub async fn search_anime(query: &str) {
    let client = Client::new();
    let url = format!("https://api.example.com/search?query={}", query);

    let response = client.get(&url).send().await.unwrap().json::<Vec<Anime>>().await.unwrap();

    for anime in response {
        println!("Title: {}, URL: {}", anime.title, anime.url);
    }
}
