use std::process::Command;
use std::fs;
use mpv;

// Represents basis for URL before attaching videoID
static TARGET_URL: &'static str = "http://www.youtube.com/watch?v=";


#[derive(Clone, Debug)]
pub struct Youtube {
    pub search_query: Option<String>,
    url: Option<String>,
}


pub enum YTReturn {
    Url,                // good!
    StringTitle,        // good!
}

impl Youtube {

    pub fn new(target: &str) -> Result<Youtube, &'static str> {
        // Important, used to determine if we actually need to call API or not.
        match self::process_target(target) {

            // If the target provided matches that of a StringTitle,
            YTReturn::StringTitle => {

                return Ok(Youtube {
                    search_query: Some(String::from(target)),
                    url: None
                });
            },

            // If the target provided is in the format of a URL
            YTReturn::Url => {

                return Ok(Youtube {
                    search_query: None,
                    url: Some(String::from(target))
                });
            }
        }
    }

    pub fn add_url(&mut self, video_id: &String){
        self.url = Some(format!("{}{}", TARGET_URL, video_id));

    }


    pub fn download_mp3(&self, home_env: String){

        use std::path::Path;

        // Create a new path for downloads
        let download_path = format!("{}/.spintable/downloads/", home_env);
        let path = Path::new(&download_path);

        // Check if path already exists. If it doesn't create it.
        match path.exists(){
            false => {
                println!("Directory doesn't exist. Creating.");
                let _ = fs::create_dir(download_path.clone());
            },
            true => {},
        }

        println!("Starting download... May take a while.");

        // Call youtube-dl as a command to download the file as an mp3
        let target = self.url.as_ref();
        let out = Command::new("youtube-dl")
            .current_dir(download_path.clone())
            .args(&[
                "-x",
                "--audio-quality", "0",
                "--audio-format", "mp3", target.unwrap()])
            .output()
            .expect("failed to execute process");

        let _ = out.stdout;

    }

    pub fn start_streaming(&self) -> Result<&'static str, mpv::Error>{

        // Create a new mpvHandler
        let mut mpv_builder = mpv::MpvHandlerBuilder::new().expect("Failed to init MPV builder");
        mpv_builder.set_option("sid","no").unwrap();
        mpv_builder.set_option("video", "no").expect("Failed to set option 'video' to 'no'");
        mpv_builder.set_option("ytdl", true).unwrap();
        mpv_builder.set_option("osc", true).unwrap();

        println!("Found URL: {:?}", self.url.clone().unwrap().as_str());

        let mut mpv = mpv_builder.build().expect("Failed to build MPV handler");
        mpv.command(&["loadfile", self.url.clone().unwrap().as_str()])
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

#[inline]
fn process_target(target: &str) -> YTReturn {

    // Youtube URLS
    let check_url: Vec<&str> = vec![
        "https://youtube.com", "http://youtube.com",
        "https://www.youtube.com", "http://www.youtube.com",
        "https://youtu.be", "http://youtu.be"];


    for url in check_url.iter(){
        // Check if check_urls are prepended
        if target.starts_with(url){
            return YTReturn::Url;
        }
    }

    YTReturn::StringTitle
}


