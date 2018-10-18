# catsay

[![Build Status](https://travis-ci.org/Jimver/catsay.svg?branch=master)](https://travis-ci.org/Jimver/catsay)
[![Build status](https://ci.appveyor.com/api/projects/status/yfam2mj2f4gqvd9i?svg=true)](https://ci.appveyor.com/project/Jimver/catsay)
[![Snap Status](https://build.snapcraft.io/badge/Jimver/catsay.svg)](https://build.snapcraft.io/user/Jimver/catsay)

This is a useless cli tool of a cat echoing what you say.

## Installation
If you have `snap` installed (Ubuntu 16.04 and up) you can simply install using:

`sudo snap install cat-say`

And run it like:

`cat-say hello`

If you don't have snap you can just get the latest binaries [here](https://github.com/Jimver/catsay/releases/latest)


## Operation
You can either provide your text: 

- Like separate arguments 

`catsay hello there`

- As a single string 

`catsay "hello there"`

- As standard input 

`echo "hello there" | catsay`

For help run either:
- `catsay -h`  
- `catsay --help`

Note: when no argument is given (so just `catsay`) the program assumes the input will come from stdin so it will block until it receives an EOF (Ctrl+D)

## Build instructions
To build this project you need the Rust language installed:

`cargo build`

And to run it:

`cargo run hello there`

## TODOs
- Add tests
- Proper help message