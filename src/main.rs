extern crate clap;
extern crate mpv;
extern crate reqwest;

#[macro_use] extern crate serde;

mod youtube;
mod stream;

use std::{fs, env};
use std::error::Error;
use std::path::Path;
use std::fs::OpenOptions;
use std::io::prelude::*;

use clap::{App, Arg, AppSettings};

fn main(){

    // get $HOME and create config dir path
    let home_env: String = match env::var_os("HOME") {
        Some(path) => format!("{}/.spintable", path.to_str().unwrap()),
        None       => { panic!("no $HOME variable set"); }
    };
    let dir_path: String = format!("{}/.spintable", home_env);
    let path = Path::new(dir_path.as_str());

    // create path if not exists
    if !path.exists() {
        let _ = fs::create_dir(dir_path.as_str());
    }

    // open $HOME/.spintable/api
    let mut file = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create(true)
                    .open(&path).unwrap();

    // read to string
    let mut content = String::new();
    if let Err(e) = file.read_to_string(&mut content) {
        panic!("{}", e.description());
    }

    // Argument parsing
    let args = App::new("spintable")
        .version("0.3")
        .version_short("v")
        .setting(AppSettings::ArgRequiredElseHelp)
        .author("Alan <ex0dus@codemuch.tech>")
        .about("play youtube music videos in the terminal")
        .arg(Arg::with_name("target")
            .help("Sets the video to be played")
            .multiple(true)
            .takes_value(true))
        .arg(Arg::with_name("download")
            .short("d")
            .long("download")
            .help("Saves MP3 of downloaded video"))
        .get_matches();

    // create Youtube object
    let target = args.value_of("target").unwrap();
    let mut yt = match stream::Youtube::new(target) {
        Ok(res) => res,
        Err(e) => panic!("[ERROR] {}", e)
    };

    let check_query = yt.search_query.clone();

    // If a search query is actually present...
    if let Some(query) = check_query {

        println!("{}", query);
        // Send a request to Data API
        let video = youtube::send_request(query, &content);

        // Add url to struct
        yt.add_url(&video.items[0].id.video_id);
    }

    // Check if download is necessary
    if args.is_present("download"){
        yt.download_mp3(home_env);
    }

    loop {
        match yt.start_streaming(){
            Ok(r) => {
                println!("{}", r);
            }
            Err(e) => {
                panic!("[ERROR] {:?}", e);
            }
        }
    }
}
