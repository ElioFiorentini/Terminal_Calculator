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
use std::collections::VecDeque;
use std::io;
use std::io::Write;

#[derive(Debug, Copy, Clone, PartialEq)]
enum MathFunction {
    Pow,
    Sqrt,
    Exp,
    Log,
    Ln,
    Abs,
    Floor,
    Sin,
    Cos,
    Tan,
    Asin,
    Acos,
    Atan,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Token {
    Operand(f64),
    Operator(char),
    Function(MathFunction),
    Ponctuation(char),
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

fn slice_to_token(input_string: &str) -> Token {
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

            // Functions:
            // power
            "^" => Token::Function(MathFunction::Pow),
            "pow" => Token::Function(MathFunction::Pow),
            "power" => Token::Function(MathFunction::Pow),
            // square root
            "sqrt" => Token::Function(MathFunction::Sqrt),
            "squareroot" => Token::Function(MathFunction::Sqrt),
            // exponential
            "exp" => Token::Function(MathFunction::Exp),
            "exponential" => Token::Function(MathFunction::Exp),
            "e" => Token::Function(MathFunction::Exp),
            // logarithm
            "log" => Token::Function(MathFunction::Log),
            "logarithm" => Token::Function(MathFunction::Log),
            // natural logarithm
            "ln" => Token::Function(MathFunction::Ln),
            "naturallogarithm" => Token::Function(MathFunction::Ln),
            "naturallog" => Token::Function(MathFunction::Ln),
            // absolute value
            "abs" => Token::Function(MathFunction::Abs),
            "absolute" => Token::Function(MathFunction::Abs),
            "absolutevalue" => Token::Function(MathFunction::Abs),
            // floor
            "floor" => Token::Function(MathFunction::Floor),
            // trigonometry
            "sin" => Token::Function(MathFunction::Sin),
            "cos" => Token::Function(MathFunction::Cos),
            "tan" => Token::Function(MathFunction::Tan),
            "asin" => Token::Function(MathFunction::Asin),
            "acos" => Token::Function(MathFunction::Acos),
            "atan" => Token::Function(MathFunction::Atan),

            // Others:
            "pi" => Token::Operand(f64::consts::PI),
            "π" => Token::Operand(f64::consts::PI),
            "," => Token::Ponctuation(','),
            "(" => Token::Ponctuation('('),
            ")" => Token::Ponctuation(')'),
            _ => Token::None,
        }
    }
}

fn string_to_token(input_string: String) -> Vec<Token> {
    let char_vector: Vec<char> = input_string.chars().collect();
    let mut token_vector = Vec::new();
    let mut i = 0;
    while i < char_vector.len() {
        let mut j = i + 1;
        if match char_vector.get(i) {
            Some(i) => i,
            None => &'.',
        }
        .is_digit(10)
        {
            let mut str = String::from(*char_vector.get(i).unwrap());
            while match char_vector.get(j) {
                Some(i) => i,
                None => &'.',
            }
            .is_digit(10)
            {
                str.push(*char_vector.get(j).unwrap());
                j += 1;
            }
            token_vector.push(slice_to_token(&str));
        } else if match char_vector.get(i) {
            Some(i) => i,
            None => &'.',
        }
        .is_alphabetic()
        {
            let mut str = String::from(*char_vector.get(i).unwrap());
            while match char_vector.get(j) {
                Some(i) => i,
                None => &'.',
            }
            .is_alphabetic()
            {
                str.push(*char_vector.get(j).unwrap());
                j += 1;
            }
            token_vector.push(slice_to_token(&str));
        } else {
            token_vector.push(slice_to_token(&char_vector.get(i).unwrap().to_string()));
        }
        if j > i + 1 {
            i = j;
        } else {
            i += 1;
        }
    }

    token_vector
}

fn is_parenthesis(o: &Token) -> bool {
    if *o == Token::Ponctuation('(') || *o == Token::Ponctuation(')') {
        true
    } else {
        false
    }
}

fn is_exponential(o: &Token) -> bool {
    if *o == Token::Function(MathFunction::Exp) {
        true
    } else {
        false
    }
}

fn is_mult_or_div(o: &Token) -> bool {
    if *o == Token::Operator('*') || *o == Token::Operator('/') || *o == Token::Operator('%') {
        true
    } else {
        false
    }
}

fn is_add_or_sub(o: &Token) -> bool {
    if *o == Token::Operator('+') || *o == Token::Operator('-') {
        true
    } else {
        false
    }
}

fn takes_precedence(o1: &Token, o2: &Token) -> bool {
    if o1 == o2 {
        return false;
    }
    if is_parenthesis(&o1) {
        if is_parenthesis(&o2) {
            return false;
        }
        return true;
    }
    if is_exponential(&o1) {
        if is_parenthesis(&o2) || is_exponential(&o2) {
            return false;
        }
        return true;
    }
    if is_mult_or_div(&o1) {
        if is_parenthesis(&o2) || is_exponential(&o2) || is_mult_or_div(&o2) {
            return false;
        }
        return true;
    }
    if is_add_or_sub(&o1) {
        if is_parenthesis(&o2) || is_exponential(&o2) || is_mult_or_div(&o2) || is_add_or_sub(&o2) {
            return false;
        }
        return true;
    }
    false
}

fn same_precedence(o1: Token, o2: Token) -> bool {
    if (is_parenthesis(&o1) && is_parenthesis(&o2))
        || (is_exponential(&o1) && is_exponential(&o2))
        || (is_mult_or_div(&o1) && is_mult_or_div(&o2))
        || (is_add_or_sub(&o1) && is_add_or_sub(&o2))
    {
        true
    } else {
        false
    }
}

fn is_left_associative(o: &Token) -> bool {
    if *o == Token::Function(MathFunction::Pow) {
        false
    } else {
        true
    }
}

fn is_function(f: &Token) -> bool {
    match f {
        Token::Function(_) => true,
        _ => false,
    }
}

fn infix_to_postfix(infix: Vec<Token>) -> Result<Vec<Token>, &'static str> {
    // Implementation of the "Shunting yard algorithm" from Edsger Dijkstra.
    // Seen at: https://en.wikipedia.org/wiki/Shunting_yard_algorithm

    let mut output_queue: VecDeque<Token> = VecDeque::new();
    let mut operator_stack: Vec<Token> = Vec::new();

    for token in infix.iter() {
        match token {
            Token::Operand(n) => output_queue.push_back(Token::Operand(*n)),
            Token::Function(f) => operator_stack.push(Token::Function(*f)),
            Token::Operator(o1) => {
                let o2 = match operator_stack.last() {
                    Some(o2) => o2.clone(),
                    None => Token::None,
                };
                while match o2 {
                    Token::None => false,
                    _ => true,
                } && (takes_precedence(&o2, &Token::Operator(*o1))
                    || (same_precedence(Token::Operator(*o1), o2)
                        && is_left_associative(&Token::Operator(*o1))))
                {
                    output_queue.push_back(operator_stack.pop().unwrap());
                }
                operator_stack.push(Token::Operator(*o1));
            }
            Token::Ponctuation(p) => {
                if *p == ',' {
                    while *operator_stack.last().unwrap() != Token::Ponctuation('(') {
                        output_queue.push_back(operator_stack.pop().unwrap());
                    }
                } else if *p == '(' {
                    operator_stack.push(Token::Operator(*p));
                } else if *p == ')' {
                    while !operator_stack.is_empty()
                        && *operator_stack.last().unwrap() != Token::Ponctuation('(')
                    {
                        output_queue.push_back(operator_stack.pop().unwrap());
                    }
                    if operator_stack.is_empty()
                        || *operator_stack.last().unwrap() != Token::Ponctuation('(')
                    {
                        return Err("Error: unmatching parenthesis.");
                    }
                    let _ = operator_stack.pop();
                    if is_function(operator_stack.last().unwrap()) {
                        output_queue.push_back(operator_stack.pop().unwrap());
                    }
                }
            }
            _ => {}
        }
    }
    while !operator_stack.is_empty() {
        if is_parenthesis(operator_stack.last().unwrap()) {
            return Err("Error: unmatching parenthesis.");
        }
        output_queue.push_back(operator_stack.pop().unwrap());
    }
    println!("{:?}", output_queue);
    Ok(output_queue.into())
}

fn compute(postfix: Vec<Token>) -> f64 {
    _ = postfix;
    0.0
}

fn main() {
    // User input: to do
    println!("Calculator by Elio Fiorentini.");
    print!(">>> ");
    io::stdout().flush().unwrap();

    let mut user_input = String::new();
    let _ = io::stdin().read_line(&mut user_input);

    // Tests:
    /*
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
    */

    // let user_input = String::from("15+4/2*8");
    let postfix_notation: Vec<Token> = match infix_to_postfix(string_to_token(user_input)) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}", e);
            Vec::new()
        }
    };
    let result = compute(postfix_notation);

    println!("{result}");
}
