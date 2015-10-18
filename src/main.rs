extern crate bf;

#[cfg(not(test))]
use bf::*;

#[cfg(not(test))]
fn main() {
    BFProgram::parse(".".to_string()).unwrap().run(None);
    print!("\n");
    BFProgram::parse("+.".to_string()).unwrap().run(None);
    print!("\n");
    BFProgram::parse("-".to_string()).unwrap().run(None);
    print!("\n");
    BFProgram::parse("[-]".to_string()).unwrap().run(None);
    BFProgram::parse("++++>++".to_string()).unwrap().run(None);
}

