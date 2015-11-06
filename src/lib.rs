use std::str::Chars;
use Symbol::*;
use std::io::{Read, Write, stdout, stdin};

#[derive(Debug, PartialEq, Eq)]
enum Symbol {
    Add,
    Sub,
    Next,
    Prev,
    While(Vec<Symbol>),
    Print,
    Get,
}

#[derive(Debug)]
pub struct BFProgram {
    code: Vec<Symbol>,
}

pub type ParseResult<T> = Result<T, String>;

impl BFProgram {

    ///
    ///
    ///
    pub fn parse(code: String) -> ParseResult<Self> {

        let mut program = Vec::new();
        let mut iter = code.chars();
        while let Some(c) = iter.next() {
            let symbol = match c as char {
                '+' => Add,
                '-' => Sub,
                '>' => Next,
                '<' => Prev,
                '[' => {
                    match BFProgram::parse_while(&mut iter) {
                        Ok(block) => While(block),
                        Err(e) => return Err(e),
                    }
                }
                ']' => return Err("Incorrect placement of ]".to_string()),
                '.' => Print,
                ',' => Get,
                _ => continue, // ignore every other character
            };
            program.push(symbol);
        }

        Ok(BFProgram { code: program })
    }

    fn parse_while(iter: &mut Chars) -> ParseResult<Vec<Symbol>> {

        let mut code = Vec::new();
        loop {
            code.push(match iter.next() {
                Some(c) => match c as char {
                    '+' => Add,
                    '-' => Sub,
                    '>' => Next,
                    '<' => Prev,
                    '[' => While(BFProgram::parse_while(iter).unwrap()),
                    ']' => break,
                    '.' => Print,
                    ',' => Get,
                    _ => continue, // ignore every other character
                },
                None => return Err("Incorrect placement of ]".to_string()),
            });
        }
        Ok(code)
    }

    /// run `BFProgram` with stdin as the input
    ///
    /// # Example
    ///
    /// ```
    /// use bf::BFProgram;
    ///
    /// BFProgram::parse("-".to_string()).unwrap().run()
    ///
    /// ```
    pub fn run(&self) {
        self.run_with(&mut stdin(), &mut stdout());
    }

    /// run `BFProgram` with anything that implments `Read` as an input
    ///
    /// # Example
    ///
    /// ```
    /// use std::io;
    /// use std::io::prelude::*;
    /// use std::fs::File;
    /// use bf::BFProgram;
    /// # fn foo() -> io::Result<()> {
    ///
    /// let mut f_in = try!(File::open("in.txt"));
    /// let mut f_out = try!(File::open("out.txt"));
    /// BFProgram::parse("-".to_string()).unwrap()
    ///                                  .run_with(&mut f_in, &mut f_out);
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn run_with<R: Read, W: Write>(&self, input: &mut R, out: &mut W) {
        let mut buffer = [0; 256];
        let mut i =  0;

        Self::run_internal(input, out, &mut i, &mut buffer, &self.code);
    }

    fn run_internal<R: Read, W: Write>(input: &mut R,
                                       output: &mut W,
                                       index: &mut usize,
                                       buffer: &mut [u8],
                                       code: &Vec<Symbol>) {

        // Would use iterator dor buffer if there was that did: &mut + peek() +
        // DoubleEndedIterator
        for sym in code {
            match *sym {
                Add => {
                    buffer[*index] = if buffer[*index] == std::u8::MAX {
                        std::u8::MIN
                    } else {
                        buffer[*index] + 1
                    };
                }
                Sub => {
                    buffer[*index] = if buffer[*index] == std::u8::MIN {
                        std::u8::MAX
                    } else {
                        buffer[*index] - 1
                    };
                }
                Next => {
                    *index = if *index + 1 < buffer.len() {
                        *index + 1
                    } else {
                        0
                    }
                }
                Prev => {
                    *index = if *index > 0 {
                        *index - 1
                    } else {
                        buffer.len() - 1
                    }
                }
                While(ref while_code) => while buffer[*index] != 0 {
                    Self::run_internal(input, output, index, buffer, while_code);
                },
                Print => { write!(output, "{}", buffer[*index] as char).unwrap(); },
                Get => {
                    let mut buf = [0; 1];
                    buffer[*index] = match input.read(&mut buf) {
                        Ok(_) => buf[0],
                        Err(_) => 0,
                    };
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::Symbol::*;

    // TODO: Update macros once prefixing function names in macros are implemented
    macro_rules! parse_test {
        ($name:ident, $code:expr, $exp:expr) => {
            #[test]
            fn $name() {
                assert_eq!(BFProgram::parse($code.to_string()).unwrap().code, $exp);
            }
        }
    }

    macro_rules! incorrect_parse_test {
        ($name:ident, $code:expr) => {
            #[test]
            #[should_panic]
            fn $name() {
                BFProgram::parse($code.to_string()).unwrap();
            }
        }
    }

    #[test]
    fn no_arthimatic_underflow() {
        BFProgram::parse("-".to_string()).unwrap().run();
    }

    #[test]
    fn no_arthimatic_overflow() {
        BFProgram::parse("-[>+<-]".to_string()).unwrap().run();
    }

    incorrect_parse_test!(parse_fail_while_first_brackets,"[");
    incorrect_parse_test!(parse_fail_while_second_brackets,"[[]");

    parse_test!(parse_simple_sub, "-", [Sub]);
    parse_test!(parse_simple_add, "+", [Add]);
    parse_test!(parse_simple_next, ">", [Next]);
    parse_test!(parse_simple_prev, "<", [Prev]);
    parse_test!(parse_simple_get, ",", [Get]);
    parse_test!(parse_simple_print, ".", [Print]);
    parse_test!(parse_simple_while, "[]", [While(vec![])]);
    parse_test!(parse_simple_all, "+-.,><[]", [Add, Sub, Print, Get, Next, Prev, While(vec![])]);

    parse_test!(parse_while_nested, "[[]]", [While(vec![While(vec![])])]);
    parse_test!(parse_while_with_contents, "[+-]", [While(vec![Add, Sub])]);

}
