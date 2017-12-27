extern crate clap;
extern crate termion;
extern crate mpv;
extern crate pbr;
extern crate serde_json;
extern crate serde;
extern crate reqwest;

#[macro_use]
extern crate serde_derive;

mod youtube;
mod stream;

use std::error::Error;
use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;
use std::env;

use termion::clear;
use clap::{App, Arg, AppSettings};

fn main(){
    
    // Clear the entirety of the screen
    println!("{}", clear::All);
    
    // Stores api key from /root/.spintable/api.txt
    let mut content: String = String::new();

    // Store $HOME environmental variable
    let home_env: String = match env::home_dir() {
        Some(path) => String::from(path.to_str().unwrap()),
        None => String::from("/root"),
    };

    // Final path to API keys
    let api_path: String = format!("{}/.spintable/api.txt", home_env);
    
    // Create a new directory if not already exists.
    let path = Path::new(api_path.as_str());

    // If the path already exists
    match path.exists(){
        true => {
            
            // First, get the API file opened and the content stored and ready
            // for request.            
            let display = path.display();
            let mut file = match OpenOptions::new().read(true).write(true).create(true).open(&path){
                Err(e) => {
                    panic!("[ERROR] Couldn't open {}. Reason: {}", display,
                    e.description());
                },
                Ok(file) => file
            };
            
            // Attempt to write API data to content String
            match file.read_to_string(&mut content){
                Err(e) => {
                    println!("API File may be empty. Place key at $HOME/.spintable/api.txt");
                    panic!("[ERROR] Couldn't read {}. Reason: {}", display,
                    e.description());
                },
                Ok(_) => {},
            }
        },
        false => { let _ = fs::create_dir("/root/.spintable"); },
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
                
    let target = args.value_of("target").unwrap();

    // Create Youtube object
    let mut yt = match stream::Youtube::new(target){
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
