pub mod stream {
    use std::process::Command;
    use std::fs;
    use mpv;
    
    pub enum YTReturn {
        URL,
        StringTitle,
    }
         
    pub fn process_target(target: &str) -> YTReturn {        
        let check_str = "https://";
        
        match target.starts_with(check_str){
            true => {
                YTReturn::URL
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

    pub fn start_streaming(target: &str) -> Result<&'static str, mpv::Error>{
        
        println!("Streaming {:?}! Enjoy!", target);
        
        let mut mpv_builder = mpv::MpvHandlerBuilder::new().expect("Failed to init MPV builder");        
        mpv_builder.set_option("sid","no").unwrap();
        mpv_builder.set_option("video", "no").expect("Failed to set option 'video' to 'no'");        
        mpv_builder.set_option("ytdl", true).unwrap();
        mpv_builder.set_option("osc", true).unwrap();

        let mut mpv = mpv_builder.build().expect("Failed to build MPV handler");
        mpv.command(&["loadfile", target])
            .expect("Error loading file");
        
        mpv.set_property("loop", "1").unwrap();
        mpv.set_property("speed", "1").unwrap();
        
        'main: loop {
           while let Some(event) = mpv.wait_event(0.0) {
               use mpv::Event::*;
               match event {
                   Shutdown => { break 'main; },
                   StartFile => { println!("Starting stream..."); },
                   FileLoaded => { println!("File loaded..."); },
                   EndFile(e) => {
                      if let Err(msg) = e {
                          return Err(msg);
                      } else {
                          break 'main;
                      }
                  },
                   _ => {}
               };
           }
       }
       Ok("Successfully streamed!")
    }
}