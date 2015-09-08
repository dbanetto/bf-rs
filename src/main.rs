
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

    let mut i = 0;
    for sym in program {
        match *sym {
            BFSymbol::Add => data[i] += 1,
            BFSymbol::Sub => data[i] -= 1,
            BFSymbol::Next => i = (i + 1) % data.len(),
            BFSymbol::Prev => {
                i -= 1;
                if i < 0 {
                    i = data.len() - 1;
                }
            },
            BFSymbol::WhileStart => (),
            BFSymbol::WhileEnd => (),
            BFSymbol::Print => print!("{}", data[i] as char),
            BFSymbol::Get => (),
        }
    }
}

fn main() {
    run(&parse(".".to_string()), None);
}
