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

use core::f64;

#[derive(Debug)]
enum Token {
    Operand(f64),
    Operator(char),
    Function(String),
    None,
}

fn is_str_a_number(str: &str) -> bool {
    for char in str.chars() {
        if !char.is_digit(10) && char != '.' {
            return false;
        }
    }
    true
}

fn string_to_token(input_string: &str) -> Token {
    if is_str_a_number(input_string) {
        Token::Operand(input_string.parse().unwrap())
    } else {
        match input_string {
            // Operators:
            "+" => Token::Operator('+'),
            "-" => Token::Operator('-'),
            "−" => Token::Operator('-'),
            "*" => Token::Operator('*'),
            "x" => Token::Operator('*'),
            "×" => Token::Operator('*'),
            "⋅" => Token::Operator('*'),
            "/" => Token::Operator('/'),
            "÷" => Token::Operator('/'),
            "%" => Token::Operator('%'),
            "mod" => Token::Operator('%'),
            "(" => Token::Operator('('),
            ")" => Token::Operator(')'),

            // Functions:
            // power
            "^" => Token::Function("pow".to_string()),
            "pow" => Token::Function("pow".to_string()),
            "power" => Token::Function("pow".to_string()),
            // square root
            "sqrt" => Token::Function("sqrt".to_string()),
            "squareroot" => Token::Function("sqrt".to_string()),
            // exponential
            "exp" => Token::Function("exp".to_string()),
            "exponential" => Token::Function("exp".to_string()),
            "e" => Token::Function("exp".to_string()),
            // logarithm
            "log" => Token::Function("log".to_string()),
            "logarithm" => Token::Function("log".to_string()),
            // natural logarithm
            "ln" => Token::Function("ln".to_string()),
            "naturallogarithm" => Token::Function("ln".to_string()),
            "naturallog" => Token::Function("ln".to_string()),
            // absolute value
            "abs" => Token::Function("abs".to_string()),
            "absolute" => Token::Function("abs".to_string()),
            "absolutevalue" => Token::Function("abs".to_string()),
            // floor
            "floor" => Token::Function("floor".to_string()),
            // trigonometry
            "sin" => Token::Function("sin".to_string()),
            "cos" => Token::Function("cos".to_string()),
            "tan" => Token::Function("tan".to_string()),
            "asin" => Token::Function("asin".to_string()),
            "acos" => Token::Function("acos".to_string()),
            "atan" => Token::Function("atan".to_string()),

            // Others:
            "pi" => Token::Operand(f64::consts::PI),
            "π" => Token::Operand(f64::consts::PI),
            _ => Token::None,
        }
    }
}

fn infix_to_postfix(input: String) -> Vec<Token> {
    // Implementation of the "Shunting yard algorithm" from Edsger Dijkstra.
    // Seen at: https://en.wikipedia.org/wiki/Shunting_yard_algorithm
    _ = input;
    Vec::new()
}

fn compute(postfix: Vec<Token>) -> f64 {
    _ = postfix;
    0.0
}

fn main() {
    // User input: to do

    // Tests:
    println!("{:?}", string_to_token("3256"));
    println!("{:?}", string_to_token("0.6453"));
    println!("{:?}", string_to_token("pi"));
    println!("{:?}", string_to_token("pow"));
    println!("{:?}", string_to_token("^"));
    println!("{:?}", string_to_token("sqrt"));
    println!("{:?}", string_to_token("+"));
    println!("{:?}", string_to_token("mod"));
    println!("{:?}", string_to_token("÷"));
    println!("{:?}", string_to_token("⋅"));
    println!("{:?}", string_to_token("0,6453"));
    println!("{:?}", string_to_token("bonjour"));

    let user_input = String::from("15+4/2*8");
    let postfix_notation = infix_to_postfix(user_input);
    let result = compute(postfix_notation);

    println!("{result}");
}
