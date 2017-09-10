extern crate clap;
extern crate serde_json;
extern crate curl;
extern crate regex;

mod youtube;
mod stream;

use clap::{App, Arg, AppSettings};
use std::process;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use stream::stream::*;
use youtube::youtube::*;


fn main(){
    let args = App::new("spintable")
        .version("0.2")
        .version_short("v")
        .setting(AppSettings::ArgRequiredElseHelp)
        .author("Alan <ex0dus@codemuch.tech>")
        .about("play youtube music videos in the terminal")
        .arg(Arg::with_name("target")
            .help("Sets the video to be played")
            .takes_value(true))
        .arg(Arg::with_name("download")
            .short("d")
            .long("download")
            .help("Saves MP3 of downloaded video"))
        .get_matches();
                
    let target = args.value_of("target").unwrap();
    
    match process_target(&target){
        YTReturn::StringTitle => {
            
            // First, get the API file opened and the content stored and ready
            // for request.
            
            let path = Path::new("api.txt");
            let display = path.display();
            let mut file = match File::open(&path){
                Err(e) => {
                    println!("[ERROR] Couldn't open {}. Reason: {}", display,
                    e.description());
                    process::exit(1);
                },
                Ok(file) => file,
            };
            
            let mut content = String::new();
            match file.read_to_string(&mut content){
                Err(e) => {
                    println!("[ERROR] Couldn't read {}. Reason: {}", display,
                    e.description());
                    process::exit(1);
                },
                Ok(_) => {},
            }
            
            if let Ok(res) = send_request(target, &content) {
                let result = match json_parse(&res) {
                    Ok(v) =>  {
                        println!("{:?}", v["items"]);
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
        _ => {}
    }
    
    if args.is_present("download"){
        if let Ok(()) = download_mp3(target){
            println!("Successfully downloaded! Now streaming...");
        }
    }
    if let Ok(()) = start_streaming(target){
        println!("Successfully streamed!");
        process::exit(0);
    }
    
}