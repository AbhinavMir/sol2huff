# Sol2Huff

Sol2Huff is a command-line tool for transpiling Solidity contracts to Huff.

## Requirements

- Rust (tested on Rust 1.56)
- Solidity compiler (tested on Solidity 0.8.10)

## Installation

`cargo +nightly install --git https://github.com/abhinavmir/sol2huff`

## Usage

`sol2huff --dir=<path-to-solidity-file> --artifact=<directory-of-artifact> --verbose`

## Acknowledgements

Shoutout @iFrostizz for developing [Murph](https://github.com/iFrostizz/murph) - a bytecode to Huff transpiler. Sol2Huff is heavily inspired by Murph.
