#![cfg(test)]
extern crate bf;

use bf::BFProgram;
use std::io::{stdin, Cursor};
use std::io::prelude::*;
use std::fs::File;

macro_rules! test_from_file {
    ($name:ident, $file:expr, $expect:expr) => {
        #[test]
        fn $name() {
            let mut out = Vec::<u8>::new();

            let mut code = String::new();
            let mut file = File::open($file.to_string()).unwrap();
            let _ = file.read_to_string(&mut code).unwrap();
            
            BFProgram::parse(&code).unwrap()
                .run_with(&mut stdin(), &mut out);

            assert_eq!(out, $expect);
        }
    }
}

macro_rules! test_from_file_with_input {
    ($name:ident, $file:expr, $expect:expr, $input:expr) => {
        #[test]
        fn $name() {
            let mut out = Vec::<u8>::new();
            let mut buff = Cursor::new($input);

            let mut code = String::new();
            let mut file = File::open($file.to_string()).unwrap();
            let _ = file.read_to_string(&mut code).unwrap();
            
            BFProgram::parse(&code).unwrap()
                .run_with(&mut buff, &mut out);

            assert_eq!(out, $expect);
        }
    }
}

test_from_file!(test_file_mul, "tests/mul.b", vec![35]);
test_from_file!(test_file_mul10, "tests/mul10.b", vec![60]);
test_from_file!(test_file_666, "tests/666.b", b"666\n");
test_from_file!(test_file_hello, "tests/hello.b", b"Hello World!\n");

// input
test_from_file_with_input!(test_file_input_muli, "tests/muli.b", vec![35], vec![5,7]);
test_from_file_with_input!(test_file_input_cat, "tests/cat.b", vec![10,11,98], vec![10,11,98]);
test_from_file_with_input!(test_file_input_cat2, "tests/cat2.b", vec![15,11,98,0], vec![15,11,98]);
test_from_file_with_input!(test_file_input_rev, "tests/rev.b", vec![1,2,3,4], vec![4,3,2,1]);
