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
                ']' => panic!("Invalid program"),
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

        let mut data = match inital {
            Some(init) => init.clone(),
            None => vec![0,0,0,0,0,0,0,0],
        };

        let mut iter = data.iter_mut();
        let mut cell = *iter.next().unwrap();
        for sym in &self.code {
            match *sym {
                Symbol::Add => cell = if cell == std::u8::MAX {
                    std::u8::MIN
                } else {
                    cell + 1
                },
                Symbol::Sub => cell = if cell == std::u8::MIN {
                    std::u8::MAX
                } else {
                    cell - 1
                },
                Symbol::Next => cell = match iter.next() {
                    Some(n) => *n,
                    None => cell,
                },
                Symbol::Prev => cell = match iter.next_back() {
                    Some(n) => *n,
                    None => cell,
                },
                Symbol::While(_) => (),
                Symbol::Print => print!("{}", cell as char),
                Symbol::Get => (),
            }
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
