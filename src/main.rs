extern crate bf;

#[cfg(not(test))]
use bf::*;

#[cfg(not(test))]
fn main() {
    run(&parse(".".to_string()), None);
    print!("\n");
    run(&parse("+.".to_string()), None);
    print!("\n");
    run(&parse("-".to_string()), None);
    print!("\n");
    run(&parse("[-]".to_string()), None);
    print!("\n");
    run(&parse("[][][][]".to_string()), None);
}

