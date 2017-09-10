# koop

Stream Youtube music videos through the terminal.

## Introduction

This application provides a level of abstraction over the `ffmpeg`, `youtube-dl` and `mpv` audio/video applications. The current features are quite primitive, but will be greatly improved.

## Features 

* Easy-to-use interface and command-line parsing
* Enable / disable downloading of `.mp3` files


## Installation

Just a few dependencies are needed.

  apt install mpv libmpv1 youtube-dl ffmpeg
  
To install:

    cargo build


## Usage

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

