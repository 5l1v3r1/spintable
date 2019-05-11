extern crate clap;
extern crate mpv;
extern crate reqwest;
extern crate serde;

mod youtube;
mod stream;

use std::{fs, env};
use std::error::Error;
use std::path::Path;
use std::fs::OpenOptions;
use std::io::prelude::*;

use clap::{App, Arg, AppSettings};

fn main() {

    // get $HOME and create config dir path
    let dir_path: String = match env::var_os("HOME") {
        Some(path) => format!("{}/.spintable", path.to_str().unwrap()),
        None       => { panic!("no $HOME variable set"); }
    };
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

    // read api key string
    let mut api_key = String::new();
    if let Err(e) = file.read_to_string(&mut api_key) {
        panic!("{}", e.description());
    }

    // argument parsing
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
    let yt = match stream::Youtube::new(target, api_key, dir_path) {
        Ok(res) => res,
        Err(e) => panic!("{}", e)
    };

    // Check if download is necessary
    if args.is_present("download") {
        yt.download_mp3();
    }

    loop {
        match yt.start_streaming() {
            Ok(r)   => { println!("{:?}", r) },
            Err(e)  => { panic!("{:?}", e) },
        }
    }
}
