/*  Autor: Elio Fiorentini
    Creation date: 11/11/2024
    Last modification: 11/11/2024

    Objective:
    Building a fully fonctional and usable console calculator.
    It must take a string in input and output
    the result of the operation entered.
    It must be able to compute math functions
    such as sin, power, square root, etc...
*/

enum Token {
    Operand(f64),
    Operator(String),
    Function(String),
}

fn infix_to_postfix(input: String) -> Vec<Token> {
    // Implementation of the "Shunting yard algorithm" from Edsger Dijkstra.
    // Seen at: https://en.wikipedia.org/wiki/Shunting_yard_algorithm
    Vec::new()
}

fn compute(postfix: Vec<Token>) -> f64 {
    0.0
}

fn main() {
    // User input: to do

    let user_input = String::from("15+4/2*8");
    let postfix_notation = infix_to_postfix(user_input);
    let result = compute(postfix_notation);

    println!("{result}");
}
