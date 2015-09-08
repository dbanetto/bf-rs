
#[derive(Debug)]
enum BFSymbol {
    Add,
    Sub,
    Next,
    Prev,
    WhileStart,
    WhileEnd,
    Print,
    Get,
}

fn parse(code: String) -> Vec<BFSymbol> {

    let mut program = Vec::new();
    for c in code.chars() {
        let symbol = match c {
            '+' => BFSymbol::Add,
            '-' => BFSymbol::Sub,
            '>' => BFSymbol::Next,
            '<' => BFSymbol::Prev,
            '[' => BFSymbol::WhileStart,
            ']' => BFSymbol::WhileEnd,
            '.' => BFSymbol::Print,
            ',' => BFSymbol::Get,
            _ => continue, // ignore every other character
        };
        println!("{} {:?}", c, symbol);
        program.push(symbol);
    }
    println!("{:?}", program);
    program
}

fn run(program: &Vec<BFSymbol>, inital: Option<Vec<u8>>) {

    let mut data = match inital {
        Some(init) => init.clone(),
        None => vec![0,0,0,0,0,0,0,0],
    };

    let mut iter = 0;
    let mut cell = *data.get_mut(iter).unwrap();
    for sym in program {
        match *sym {
            BFSymbol::Add => cell += 1,
            BFSymbol::Sub => cell -= 1,
            BFSymbol::Next => cell = match data.get_mut(iter + 1) {
                Some(n) => {iter += 1; *n},
                None => cell,
            },
            BFSymbol::Prev => cell = match data.get_mut(iter - 1) {
                Some(n) => {iter -= 1; *n},
                None => cell,
            },
            BFSymbol::WhileStart => (),
            BFSymbol::WhileEnd => (),
            BFSymbol::Print => print!("{}", cell as char),
            BFSymbol::Get => (),
        }
    }
}

fn main() {
    run(&parse(".".to_string()), None);
    run(&parse("+++.>.".to_string()), None);
}
