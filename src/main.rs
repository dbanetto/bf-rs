
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

fn parse(code: String) {

    let mut program = Vec::<BFSymbol>::new();
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
            _ => unreachable!(),
        };
        println!("{} {:?}", c, symbol);
        program.push(symbol);
    }
    println!("{:?}", program);
}

fn main() {
    parse("+++.>>>.<<<---".to_string());
}
