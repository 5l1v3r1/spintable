# spintable

Stream Youtube music videos through the terminal.

## intro

__spintable__ is a terminal-based application that enables users to play Youtube music videos over the terminal! Search strings are supported, as well as downloading!

__spintable__ will soon have a new user interface, complete with animations. As for features, we hope to also integrate adding tracks, and pause/play.


# install

Initial dependencies:

* libmpv1
* youtube-dl

To install:

    cargo install

When complete, obtain a [Youtube Data API](https://developers.google.com/youtube/v3/) key.

    echo "YOUR_API_KEY_HERE" >> ~/.spintable/api.txt

## usage

    USAGE:
    spintable [FLAGS] [target]

    FLAGS:
    -d, --download    Saves MP3 of downloaded video
    -h, --help        Prints help information
    -v, --version     Prints version information

    ARGS:
    <target>    Sets the video to be played


You can specify audio to playback with a URL:

    spintable https://www.youtube.com/watch?v=AX8-YzMKZhQ

or a search string:

    spintable "avicii the nights"

## license

[mit](https://codemuch.tech/license.txt)
