#![cfg(test)]
extern crate bf;

use bf::BFProgram;
use std::io::stdin;

macro_rules! code_test {
    ($name:ident, $code:expr, $expect:expr) => {
        #[test]
        fn $name() {
            let mut out = Vec::<u8>::new();

            BFProgram::parse($code.to_string()).unwrap()
                .run_with(&mut stdin(), &mut out);

            assert_eq!(out, $expect);
        }

    }
}

code_test!(hello_world,
           "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.", 
           b"Hello World!\n");

