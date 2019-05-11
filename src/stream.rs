//!
//! stream.rs
//!
//!     YouTube interaction helper module.
//!
use std::process::Command;
use std::fs;

use youtube;


/// defines Youtube struct. Parses a url from user input and interacts with libmpv
/// and youtube-dl in order to perform audio playback over the cli
#[derive(Clone, Debug)]
pub struct Youtube {
    pub url:            String,
    store_dir:          String,
}


/// enum type that is returned from the
/// self::process_target() helper function
pub enum YTReturn {
    Url,
    StringTitle,
}


impl Youtube {

    /// `new` initializes a new YouTube struct by parsing user input and initializing a URL to play
    pub fn new(target: &str, api: String, store_dir: String) -> Result<Youtube, &'static str> {
        match self::process_target(target) {
            YTReturn::StringTitle => {
                let video = youtube::send_request(target, &api);
                Ok(Youtube {
                    url: String::from(video.items[0].id.video_id.clone()),
                    store_dir: store_dir
                })
            },
            YTReturn::Url => {
                Ok(Youtube {
                    url: target.to_string(),
                    store_dir: store_dir,
                })
            }
        }
    }


    /// `download_mp3` is a method called to interface `youtube-cli` through CLI
    pub fn download_mp3(&self) -> () {
        use std::path::Path;

        // initialize download path
        let download_path = format!("{}/downloads/", self.store_dir);
        let path = Path::new(&download_path);
        if !path.exists() {
            let _ = fs::create_dir(download_path.clone());
        }

        // initialize youtube-dl command to download mp3
        let out = Command::new("youtube-dl")
            .current_dir(download_path)
            .args(&[
                "-x",
                "--audio-quality", "0",
                "--audio-format", "mp3", self.url.as_str()])
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
        mpv.command(&["loadfile", self.url.as_str()]).expect("error loading file");
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
                    EndFile(e)  =>      { if let Err(_) = e { break 'main; }   },
                    _           =>      {}
                };
            }
        }
        Ok(())
    }
}



/// helper that parses target url
#[inline]
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
