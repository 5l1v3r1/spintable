extern crate clap;
extern crate serde_json;
extern crate futures;
extern crate hyper;
extern crate tokio_core;

mod youtube;
mod stream;

use clap::{App, Arg};
use std::process;
use hyper::{Client, Method, Request};
use hyper::header::{ContentLength, ContentType};
use tokio_core::reactor::Core;

use stream::stream::*;


fn main(){
    let args = App::new("spintable")
        .version("0.2")
        .version_short("v")
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
        
    if args.is_present("target") {
        let target = args.value_of("target").unwrap();
        match process(&target){
            YTReturn::StringTitle => { 
                
                // First, process the search string to get a URL name.
                //let mut core = Core::new();
                //let client = Client::new(&core.handle());
                //let uri = "https://www.googleapis.com/youtube/v3/search".parse();
                
                // TODO:
            },
            YTReturn::URL=> {}
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
        
    } else {
        println!("[ERROR] No video is supplied!");
        process::exit(1);
    }
    
}