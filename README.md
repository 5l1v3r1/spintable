# spintable

Stream Youtube music videos through the terminal.

## Introduction

__spintable__ is a terminal-based application that enables users to play Youtube music videos over the terminal! Search strings are supported, as well as downloading! 

__spintable__ will soon have a new user interface, complete with animations. As for features, we hope to also integrate adding tracks, and pause/play.


## Features 

* Easy-to-use interface and command-line parsing
* Enable downloading of `.mp3` files


## Installation

__spintable__ is based off of __mpv__, as well as 
__youtube-dl__ and the Youtube Data Search API (found [here](https://developers.google.com/youtube/v3/)). While `cargo` and the Rust toolchain makes it easy to get dependencies, you still do need a few.

  apt install libmpv1 youtube-dl
  
To install:

    cargo build

When complete, obtain a [Youtube Data API](https://developers.google.com/youtube/v3/) key.

    echo "YOUR_API_KEY_HERE" >> ~/.spintable/api.txt

## Usage

### Help

    USAGE:
    spintable [FLAGS] [target]

    FLAGS:
    -d, --download    Saves MP3 of downloaded video
    -h, --help        Prints help information
    -v, --version     Prints version information

    ARGS:
    <target>    Sets the video to be played


### Use with URL

    spintable https://www.youtube.com/watch?v=AX8-YzMKZhQ

### Use with search string

    spintable "avicii the nights"


## TODO

[] Playlist support!
[] Implement rustbox for playlist query
[] ...or text / ascii tables.
[] Implement a `trait` that handles deserialization for both search queries and URLs.

