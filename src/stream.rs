//!
//! stream.rs
//!
//!     YouTube interaction helper module.
//!
use std::process::Command;
use std::fs;


/// url endpoint used to append videoID
static TARGET_URL: &'static str = "http://www.youtube.com/watch?v=";


#[derive(Clone, Debug)]
pub struct Youtube {
    pub search_query:   Option<String>,
    pub url:            Option<String>,
    home_env:           String,
}


/// enum type that is returned from the
/// self::process_target() helper function
pub enum YTReturn {
    Url,
    StringTitle,
}


impl Youtube {

    /// initializes new Youtube struct for interaction. Passes
    /// a target str that is processed
    pub fn new(target: &str) -> Result<Youtube, &'static str> {
        match self.process_target(target) {
            YTReturn::StringTitle => {
                Ok(Youtube {
                    search_query:   Some(String::from(target)),
                    url:            None
                })
            },
            YTReturn::Url => {
                Ok(Youtube {
                    search_query:   None,
                    url:            Some(String::from(target))
                })
            }
        }
    }

    /// helper that parses target url
    fn process_target(target: &str) -> YTReturn {
        let check_url = vec![
            "https://youtube.com", "http://youtube.com",
            "https://www.youtube.com", "http://www.youtube.com",
            "https://youtu.be", "http://youtu.be"];

        for url in check_url.iter() {
            if target.starts_with(url) {
                return YTReturn::Url;
            }
        }
        YTReturn::StringTitle
    }


    fn add_url() -> String {
    }


    /// method for
    pub fn download_mp3(&self) -> () {
        use std::path::Path;

        // initialize download path
        let download_path = format!("{}/.spintable/downloads/", self.home_env);
        let path = Path::new(&download_path);
        if !path.exists() {
            let _ = fs::create_dir(download_path.clone());
        }

        // initialize youtube-dl command to download mp3
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


    /// initializes a new MPV builder that streams mp3
    pub fn start_streaming(&self) -> Result<(), mpv::Error> {

        // initialize new builder with options
        let mut mpv_builder = mpv::MpvHandlerBuilder::new().expect("failed to init MPV builder");
        mpv_builder.set_option("sid","no").unwrap();
        mpv_builder.set_option("video", "no").expect("failed to set option 'video' to 'no'");
        mpv_builder.set_option("ytdl", true).unwrap();
        mpv_builder.set_option("osc", true).unwrap();

        // build command and call `loadfile` command`
        let mut mpv = mpv_builder.build().expect("failed to build MPV handler");
        mpv.command(&["loadfile", self.url]).expect("error loading file");
        mpv.set_property("loop", "1").unwrap();
        mpv.set_property("speed", "1").unwrap();

        // mpv event loop
        'main: loop {
            while let Some(event) = mpv.wait_event(0.0) {
                use mpv::Event::*;
                match event {
                    Shutdown    =>      { break 'main;                          },
                    StartFile   =>      { println!("Starting stream...");       },
                    FileLoaded  =>      { println!("File loaded...");           },
                    EndFile(e)  =>      { if let Some(_) = e { break 'main; }   },
                    _           =>      {}
                };
            }
        }
        Ok(())
    }
}


