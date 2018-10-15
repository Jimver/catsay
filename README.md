# catsay

This is a useless cli tool of a cat echoing the standard input.

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