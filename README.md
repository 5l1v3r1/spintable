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


## TODO

* Playlist support!
* Maybe rustbox?
* Interface to mpv Rust library rather than actual application
* Add some colors and design!
* Try getting title name of video?
* Parse playlist through text / config file
* Verbosity flag!