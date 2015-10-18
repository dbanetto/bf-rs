use std::str::Chars;

#[derive(Debug, PartialEq, Eq)]
pub enum Symbol {
    Add,
    Sub,
    Next,
    Prev,
    While(Vec<Symbol>),
    Print,
    Get,
}

#[derive(Debug, PartialEq, Eq)]
pub struct BFProgram {
    code: Vec<Symbol>,
}

pub type ParseResult<T> = Result<T, String>;

use Symbol::*;
impl BFProgram {

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
                },
                ']' => return Err("Incorrect placement of ]".to_string()),
                '.' => Print,
                ',' => Get,
                _ => continue, // ignore every other character
            };
            program.push(symbol);
        }
        println!("{:?}\n", program);

        Ok(BFProgram {
            code: program,
        })
    }

    fn parse_while(iter: &mut Chars) -> ParseResult<Vec<Symbol>> {

        let mut code = Vec::new();
        loop {
            code.push(match iter.next() {
                Some(c) => match c as char {
                    '+' => Symbol::Add,
                    '-' => Symbol::Sub,
                    '>' => Symbol::Next,
                    '<' => Symbol::Prev,
                    '[' => Symbol::While(BFProgram::parse_while(iter).unwrap()),
                    ']' => break,
                    '.' => Symbol::Print,
                    ',' => Symbol::Get,
                    _ => continue, // ignore every other character
                },
                None => return Err("Incorrect placement of ]".to_string()),
            });
        }
        Ok(code)
    }

    pub fn run(&self, inital: Option<Vec<u8>>) {
        let mut buffer = [0; 256];
        let mut i =  0;

        Self::run_internal(&mut i, &mut buffer, &self.code);
    }

    fn run_internal(index: &mut usize, buffer: &mut [u8], code: &Vec<Symbol>) {

        // Would use iterator dor buffer if there was that did: &mut + peek() + DoubleEndedIterator
        for sym in code {
            match *sym {
                Symbol::Add => {
                    buffer[*index] = if buffer[*index] == std::u8::MAX {
                        std::u8::MIN
                    } else {
                        buffer[*index] + 1
                    };
                },
                Symbol::Sub => {
                    buffer[*index] = if buffer[*index] == std::u8::MIN {
                        std::u8::MAX
                    } else {
                        buffer[*index] - 1
                    };
                },
                Symbol::Next => {
                    *index = if *index + 1 < buffer.len() {
                        *index + 1
                    } else {
                        0
                    }
                },
                Symbol::Prev => { // Wrap around
                    *index = if *index > 0 {
                        *index - 1
                    } else {
                        buffer.len() - 1
                    }
                },
                Symbol::While(ref while_code) => while buffer[*index] != 0 {
                  Self::run_internal(index, buffer, while_code);
                },
                Symbol::Print => print!("{}", buffer[*index] as char),
                Symbol::Get => (),
            }
            // println!("sym:={:?} buffer:={:?}", sym, cell);
        }
    }
}

#[cfg(test)]
mod test {
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
        BFProgram::parse("-".to_string()).unwrap().run(None);
    }

    #[test]
    fn no_arthimatic_overflow() {
        BFProgram::parse(".+".to_string()).unwrap()
            .run(Some(vec![255]));
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
}
