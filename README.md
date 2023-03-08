# Sol2Huff

Sol2Huff is a command-line tool for transpiling Solidity contracts to Huff.

## Requirements

- Rust (tested on Rust 1.56)
- Solidity compiler (tested on Solidity 0.8.10)

## Installation

`cargo +nightly install --git git@github.com:AbhinavMir/sol2huff.git`

## Usage

`sol2huff --dir=<path-to-solidity-file> --artifact=<directory-of-artifact> --verbose`