# bf-rs

[![Build Status](https://travis-ci.org/zyphrus/bf-rs.svg)](https://travis-ci.org/zyphrus/bf-rs)

A simple brainfuck interpreter in Rust.

The goal is to learn more rust and implement brainfuck as described at http://www.muppetlabs.com/~breadbox/bf/

## Build

run `cargo build`

## How to use

Not implemented yet.

## Portability information

Implementation details for specified [undefined behaviour](http://www.muppetlabs.com/~breadbox/bf/standards.html)

* Wraps around for the buffer, forward and back, sized 256 elements

* Wraps around the cell value with the range of 0 to 255

* no more input data

## TODO

- [x] While loops
- [ ] Documentation
- [x] Tests
- [ ] Portability tests using the [archive](http://esoteric.sange.fi/brainfuck/)
- [ ] a CLI
- [ ] Inputs

## License

This is licensed under GPL3, see LICENSE for more details
