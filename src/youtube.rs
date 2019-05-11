//!
//! youtube.rs
//!
//!     Defines main interface for interacting with YouTube
//!     Data API through reqwest::Client.
use reqwest::Client;
use serde::{Serialize, Deserialize};

/// this is the default absolute url for making requests to the Data API
static API_URL: &'static str = "https://www.googleapis.com/youtube/v3/search";


#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Id {
    #[serde(rename="videoId")]
    pub video_id: String,
}

#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Snippet {
    pub title: String,
    #[serde(rename="channelTitle")]
    pub channel_title: String
}

#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Items {
    pub id: Id,
    pub snippet: Snippet
}

#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct RootInterface {
    pub items: Vec<Items>,
}


pub fn send_request(target: &str, api: &String) -> RootInterface {
    let client = Client::new();
    let query = str::replace(target, " ", "%20");
    let params = format!("{}?q={}&maxResults=1&part=snippet&key={}", API_URL, &query, api);
    let mut res = client.get(params.as_str()).send().unwrap();
    res.json().unwrap()
}
