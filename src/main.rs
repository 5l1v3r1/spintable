extern crate clap;

use clap::{App, Arg};
use std::process;
use std::process::Command;
use std::path::Path;
use std::fs;

// TODO: Get title name while verbose!
// TODO: Set verbosity flag!
// TODO: Add some color!

enum YTReturn {
    URLNotFormatted(String),
    GoodReturn,
}

     
fn process_url(url: &str) -> YTReturn {
    
    // TODO: check if playlist
    let check_str = "https://";
    match url.starts_with(check_str){
        false => YTReturn::URLNotFormatted("[ERROR] URL not properly formatted.".to_string()),
        true =>  YTReturn::GoodReturn
    }
}

fn download_mp3(url: &str) -> Result<(), &'static str,>{
    
    let path = Path::new("downloads/");
    match path.exists(){
        false => {
            println!("Directory doesn't exist. Creating.");
            let _ = fs::create_dir("downloads/"); 
        },
        true => {},
    }
    
    println!("Starting download... May take a while.");
    
    let url = url.as_ref();
    let out = Command::new("youtube-dl")
        .current_dir("downloads/")
        .args(&[
            "-x", 
            "--audio-quality", "0",
            "--audio-format", "mp3", url])
        .output()
        .expect("failed to execute process");
    
    let _ = out.stdout;
    
    Ok(())
}

fn start_streaming(url: &str) -> Result<(), &'static str>{
    
    println!("Streaming! Enjoy!");
    
    let url = url.as_ref();
    let out = Command::new("mpv")
        .args(&[url, "--no-video"])
        .output()
        .expect("failed to execute process");
    
    let _ = out.stdout;
    
    Ok(())
}

fn main(){
    let args = App::new("koop")
        .version("0.1")
        .version_short("v")
        .author("Alan <ex0dus@codemuch.tech>")
        .about("play youtube music videos in the terminal")
        .arg(Arg::with_name("url")
            .help("Sets the URL to be played")
            .takes_value(true))
        .arg(Arg::with_name("download")
            .short("d")
            .long("download")
            .help("Saves MP3 of downloaded video"))
        .get_matches();
        
    if args.is_present("url") {
        let url = args.value_of("url").unwrap();
        match process_url(&url){
            YTReturn::GoodReturn => { 
                if args.is_present("download"){
                    if let Ok(()) = download_mp3(url){
                        println!("Successfully downloaded! Now streaming...");
                    }
                }
                
                if let Ok(()) = start_streaming(url){
                    println!("Successfully streamed!");
                    process::exit(0);
                }
            },
            YTReturn::URLNotFormatted(message) => {
                println!("{:?}", message);
                process::exit(1);
            }
        }
        
    } else {
        println!("[ERROR] No URL is supplied!");
        process::exit(1);
    }
    
}