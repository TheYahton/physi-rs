use physi_rs::lexer::Lexer;
use physi_rs::parser::Parser;
use physi_rs::units::*;

fn main() {
    // let mass = 3.0 * KILOGRAM;
    // let acceleration = 1.0 * METER / (SECOND * SECOND);
    // let force = mass * acceleration;
    // println!("f = {mass} * {acceleration} = {}", force);

    let expr = "309[kg] * (40[C] - 20[C]) = x[kg] * (100[C] - 40[C])".to_string();
    let tokens = expr.tokenize();
    println!("{tokens:#?}");
    let ast = Parser::new(tokens).parse().unwrap();
    println!("{ast:#?}");
}
