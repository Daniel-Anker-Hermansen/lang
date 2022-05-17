use parser::*;
use intepreter::*;

mod parser;

mod intepreter;

fn main() {
    let program = parse("( ( - 3 + 2 ) * 2 ) % 3 * 3");
    println!("{:#?}", program);
    println!("{}", eval(program));
}
