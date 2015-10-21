# bf-rs

[![Build Status](https://travis-ci.org/zyphrus/bf-rs.svg)](https://travis-ci.org/zyphrus/bf-rs)

A simple brainfuck interpreter in Rust.

The goal is to learn more rust and implement brainfuck as described at http://www.muppetlabs.com/~breadbox/bf/

## Build

run `cargo build --release`

## How to use

After build go into `target/release` and run `bfi --help` for usage

To run a simple program with output:

> `./bfi ,.`

To execute a file

> `./bfi -f hello.b`

To read input from a file

> `./bfi -i hello.txt .`

## Portability information

Implementation details for specified [undefined behaviour](http://www.muppetlabs.com/~breadbox/bf/standards.html)

* Wraps around for the buffer, forward and back, sized 256 elements

* Wraps around the cell value with the range of 0 to 255

* When there is no more input data or an error in the input data, the cell will be set to 0

## TODO

- [x] While loops
- [ ] Documentation
- [x] Tests
- [ ] Portability tests using the [archive](http://esoteric.sange.fi/brainfuck/)
- [x] a CLI
- [x] Inputs

## License

This is licensed under GPL3, see LICENSE for more details
