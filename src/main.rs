extern crate bf;

#[cfg(not(test))]
use bf::*;

#[cfg(not(test))]
fn main() {
    BFProgram::parse(".".to_string()).unwrap().run();
    print!("\n");
    BFProgram::parse(",.".to_string()).unwrap().run(); // FIXME:
    print!("\n");
    BFProgram::parse("+.".to_string()).unwrap().run();
    print!("\n");
    BFProgram::parse("-".to_string()).unwrap().run();
    print!("\n");
    BFProgram::parse("[-]".to_string()).unwrap().run();
    BFProgram::parse("++++>++".to_string()).unwrap().run();
}

