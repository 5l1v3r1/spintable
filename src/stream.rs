pub mod stream {
    
    use std::process::Command;
    use std::path::Path;
    use std::fs;

    
    pub enum YTReturn {
        URL,
        StringTitle,
      // Playlist, not yet covered, but most be able to parse.
    }

         
    pub fn process(target: &str) -> YTReturn {
        
        let check_str = "https://";
        match target.starts_with(check_str){
            false => YTReturn::URL,
            true =>  YTReturn::StringTitle,
        }
    }

    pub fn download_mp3(target: &str) -> Result<(), &'static str,>{
            
        let path = Path::new("downloads/");
        match path.exists(){
            false => {
                println!("Directory doesn't exist. Creating.");
                let _ = fs::create_dir("downloads/"); 
            },
            true => {},
        }
        
        println!("Starting download... May take a while.");
        
        let target = target.as_ref();
        let out = Command::new("youtube-dl")
            .current_dir("downloads/")
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