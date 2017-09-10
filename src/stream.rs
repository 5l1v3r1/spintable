pub mod stream {
    
    use std::process::Command;
    use std::fs;
    //use regex::Regex;
    
    pub enum YTReturn {
        URL,
        StringTitle,
    }
         
    pub fn process_target(target: &str) -> YTReturn {        
        let check_str = "https://";
        
        match target.starts_with(check_str){
            true => {
                //let re = Regex::new(r"/^.*(youtu.be\/|list=)([^#\&\?]*).*/").unwrap();
                //if re.is_match(target){
                //    YTReturn::Playlist
                //}
                //else {
                    YTReturn::URL
                //}    
            },
            false =>  YTReturn::StringTitle,
        }
    }

    pub fn download_mp3(target: &str) -> Result<(), &'static str,>{
        use std::path::Path;
        
        let path = Path::new("/root/.spintable/downloads/");
        match path.exists(){
            false => {
                println!("Directory doesn't exist. Creating.");
                let _ = fs::create_dir("/root/.spintable/downloads/"); 
            },
            true => {},
        }
        
        println!("Starting download... May take a while.");
        
        let target = target.as_ref();
        let out = Command::new("youtube-dl")
            .current_dir("/root/.spintable/downloads/")
            .args(&[
                "-x", 
                "--audio-quality", "0",
                "--audio-format", "mp3", target])
            .output()
            .expect("failed to execute process");
        
        let _ = out.stdout;
        
        Ok(())
    }

    pub fn start_streaming(target: &str) -> Result<(), &'static str>{
        
        println!("Streaming {:?}! Enjoy!", target);
        
        let target = target.as_ref();
        let out = Command::new("mpv")
            .args(&[target, "--no-video"])
            .output()
            .expect("failed to execute process");
        
        let _ = out.stdout;
        
        Ok(())
    }
    
}