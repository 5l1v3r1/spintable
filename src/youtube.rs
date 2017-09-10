// Use this module to grab Youtube Data API.

pub mod youtube {
    use serde_json;
    use serde_json::{Value, Error};
    use curl::easy::{Easy2, Handler, WriteError};
    
    static URL: &'static str = "https://www.googleapis.com/youtube/v3/search";
    
    struct Collector(Vec<u8>);
    
    impl Handler for Collector {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.0.extend_from_slice(data);
        Ok(data.len())
        }
    }

    pub fn send_request(query: &str, api: &String) -> Result<String, u32> {
        let mut easy = Easy2::new(Collector(Vec::new()));
        easy.get(true).unwrap();
        
        // Replace spaces with %20 character
        let query = str::replace(query, " ", "%20");
        
        // Create a string that concatenates pieces of the request
        let params = String::new() + URL + "?q=" + &query
            + "&maxResults=1" + "&part=snippet" + "&key=" 
            + api;

        easy.url(&params).unwrap();
        easy.perform().unwrap();
        match easy.response_code().unwrap() {
            200 => {
                let contents = easy.get_ref(); 
                Ok(String::from_utf8((&contents.0).to_vec()).unwrap())
                
            },
            _ => { Err(easy.response_code().unwrap()) }    
        }
    }
    
    
    pub fn json_parse(data: &str) -> Result<Value, Error>{
        let v: Value = serde_json::from_str(data).unwrap();
        
        Ok(v)
    }
    
}