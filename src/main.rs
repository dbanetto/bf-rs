extern crate bf;
extern crate clap;

use std::io::prelude::*;
use std::io::stdout;
use std::fs::File;
use bf::*;
use clap::{Arg, App};

fn main() {
    let matches = App::new("bfi")
                      .version("0.1.0")
                      .author("David Barentt <david@barnett.net.nz>")
                      .about("A rust implementation of brainfuck")
                      .arg(Arg::with_name("CODE").help("brainfuck code to execute"))
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

    let code: String = if let Some(s) = matches.value_of("CODE") {
        // Always take code from ARGS over FILE
        s.to_string()
    } else {
        if let Some(f) = matches.value_of("FILE") {
            let mut s = String::new();
            let mut file = match File::open(f) {
                Ok(fi) => fi,
                Err(e) => {
                    println!("Error while opening file `{}`: {}", f, e);
                    return;
                }
            };
            if let Err(e) = file.read_to_string(&mut s) {
                println!("Error while reading from `{}`: {}", f, e);
                return;
            }
            s
        } else {
            println!("{}", matches.usage());
            return;
        }
    };

    match BFProgram::parse(&code) {
        Ok(prog) => {
            if let Some(read) = matches.value_of("INPUT") {
                match File::open(read) {
                    Ok(mut r) => prog.run_with(&mut r, &mut stdout()),
                    Err(e) => {
                        println!("Error while opening file `{}` {}", read, e);
                        return;
                    }
                }
            } else {
                prog.run();
            }
        }
        Err(e) => println!("Error while parsing program: {}", e),
    }
}
