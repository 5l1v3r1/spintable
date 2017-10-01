extern crate clap;
extern crate termion;
extern crate mpv;
extern crate pbr;
extern crate serde_json;
extern crate serde;
extern crate curl;
extern crate libc;

#[macro_use]
extern crate serde_derive;

mod youtube;
mod stream;

use clap::{App, Arg, AppSettings};
use std::process;
use std::error::Error;
use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;
use termion::clear;

use stream::stream::*;
use youtube::youtube::*;

// Represents basis for URL before attaching videoID
static TARGET_URL: &'static str = "https://www.youtube.com/watch?v=";

fn main(){
    
    // Clear the entirety of the screen
    println!("{}", clear::All);
    
    // Stores api key from /root/.spintable/api.txt
    let mut content = String::new();
    
    // Create a new directory if not already exists.
    let path = Path::new("/root/.spintable/");
    match path.exists(){
        true => {
            // First, get the API file opened and the content stored and ready
            // for request.            
            let path = Path::new("/root/.spintable/api.txt");
            
            let display = path.display();
            let mut file = match OpenOptions::new().read(true).write(true).create(true).open(&path){
                Err(e) => {
                    println!("[ERROR] Couldn't open {}. Reason: {}", display,
                    e.description());
                    process::exit(1);
                },
                Ok(file) => {
                    file
                },
            };
            
            match file.read_to_string(&mut content){
                Err(e) => {
                    println!("[ERROR] Couldn't read {}. Reason: {}", display,
                    e.description());
                    println!("API File may be empty. Place key at /root/.spintable/api.txt");
                    process::exit(1);
                },
                Ok(_) => {},
            }
        },
        false => { let _ = fs::create_dir("/root/.spintable"); },
    }
    
    // Argument parsing
    let args = App::new("spintable")
        .version("0.2")
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
    
    // Create a heap-allocated string for later consumption
    let mut url = String::new();
    
    // Important, used to determine if we actually need to call API or not.
    match process_target(&target){
        
        // If it is just a string representing the video's title...
        YTReturn::StringTitle => {
                        
            if let Ok(res) = send_request(target, &content) {
                match json_parse(&res) {
                    Ok(v) =>  {
                        url = String::new() + TARGET_URL + &v.items[0].id.video_id;
                    },
                    Err(e) => {
                        println!("[ERROR] {:?}", e);
                    },
                };
                
            } else if let Err(e) = send_request(target, &content) {
                println!("[ERROR] Code: {:?} ", e);
                process::exit(1);
            }
        },
        _ => {
            url = String::new() +  &target;
        }
    }
    
        
    if args.is_present("download"){
        if let Ok(()) = download_mp3(&url){
            println!("Successfully downloaded! Now streaming...");
        }
    }
    
    match start_streaming(&url){ 
        Ok(r) => {
            println!("{}", r);
            process::exit(0);
        }
        Err(e) => {
            println!("[ERROR] {:?}", e);
            process::exit(1);
        }
    }
    
}
