[![Build binary release](https://github.com/nogibjj/week5-rust-jk499/actions/workflows/release.yml/badge.svg)](https://github.com/nogibjj/week5-rust-jk499/actions/workflows/release.yml)

[![Tests](https://github.com/nogibjj/week5-rust-jk499/actions/workflows/tests.yml/badge.svg)](https://github.com/nogibjj/week5-rust-jk499/actions/workflows/tests.yml)

[![Clippy](https://github.com/nogibjj/week5-rust-jk499/actions/workflows/lint.yml/badge.svg)](https://github.com/nogibjj/week5-rust-jk499/actions/workflows/lint.yml)


# Binary Search Operation: log(O(N)).

**Developer:** Jaya Khan 

## Abstract
This program uses a loop to find an element in a sorted error.


## Requirements
The code was tested on:
- rust edition = 2021


## Project Structure
                                                                               
    binary-search
    ├── Cargo.toml
    ├── README.md   
    ├── src
        ├── main.rs
    ├── .github
        ├── workflows
            ├── lint.yml
            ├── release.yml
            ├── rustfmt.yml
            ├── tests.yml
    ├── Makefile


## Commands to install Rust
1. sudo apt install curl
2. curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
3. source $HOME/.cargo/env

    ### Check installation:
    rustc --version

    ### Command to uninstall rust
    rustup self uninstall

## Command to build project (CI/CD): 
`make all`