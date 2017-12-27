use reqwest::Client;

// This is the default absolute url for making requests to the Data API
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

pub fn send_request(target: String, api: &String) -> RootInterface {
    
    // Create new HTTP GET client
    let client = Client::new();

    // Replace spaces with %20 character
    let query = str::replace(target.as_str(), " ", "%20");
    
    // Create a string that concatenates pieces of the request
    let params = format!("{}?q={}&maxResults=1&part=snippet&key={}", API_URL, &query, api);
    
    // Send request, retrieve text from response.
    let res = client.get(params.as_str())
                .send().unwrap()
                .json().unwrap();
    
    res
}
    
