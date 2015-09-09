#[derive(Debug)]
pub enum Symbol {
    Add,
    Sub,
    Next,
    Prev,
    WhileStart,
    WhileEnd,
    Print,
    Get,
}

pub fn parse(code: String) -> Vec<Symbol> {

    let mut program = Vec::new();
    for c in code.chars() {
        let symbol = match c {
            '+' => Symbol::Add,
            '-' => Symbol::Sub,
            '>' => Symbol::Next,
            '<' => Symbol::Prev,
            '[' => Symbol::WhileStart,
            ']' => Symbol::WhileEnd,
            '.' => Symbol::Print,
            ',' => Symbol::Get,
            _ => continue, // ignore every other character
        };
        println!("{} {:?}", c, symbol);
        program.push(symbol);
    }
    println!("{:?}\n", program);
    program
}

pub fn run(program: &Vec<Symbol>, inital: Option<Vec<u8>>) {

    let mut data = match inital {
        Some(init) => init.clone(),
        None => vec![0,0,0,0,0,0,0,0],
    };

    let mut iter = data.iter_mut();
    let mut cell = *iter.next().unwrap();
    for sym in program {
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
            Symbol::WhileStart => (),
            Symbol::WhileEnd => (),
            Symbol::Print => print!("{}", cell as char),
            Symbol::Get => (),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn no_arthimatic_underflow() {
        run(&parse("-".to_string()), None);
    }
}
