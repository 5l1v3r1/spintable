// Use this module to grab Youtube Data API.

pub mod youtube {
    use serde_json;
    use serde_json::{Value, Error};
    
    use std::io::{self, Write};
    use futures::{Future, Stream};

    pub fn send_request(query: &str) -> (){
        
    }
    
    pub fn json_parse(data: &str) -> Result<Value, Error>{
        
        
        let v: Value = serde_json::from_str(data)?;
        Ok(v)
    }
    
}