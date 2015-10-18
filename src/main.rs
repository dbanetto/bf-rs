extern crate bf;
extern crate clap;

#[cfg(not(test))]
use bf::*;
use clap::{Arg, App, SubCommand};

#[cfg(not(test))]
fn main() {
    let matches = App::new("bfi")
                    .version("0.1.0")
                    .author("")
                    .about("")
                    .arg(Arg::with_name("CODE")
                        .short("c")
                        .help("brainfuck code to execute")
                        .takes_value(true))
                    .arg(Arg::with_name("FILE")
                        .short("f")
                        .help("brainfuck code to execute from file")
                        .takes_value(true))
                    .arg(Arg::with_name("INPUT")
                        .short("i")
                        .long("input")
                        .help("Read input from file instead of stdin")
                        .takes_value(true))
                    .get_matches();
    // Hello world from wikipedia
    BFProgram::parse("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.".to_string()).unwrap().run();
}

