/*  Autor: Elio Fiorentini
Creation date: 11/11/2024
Last modification: 27/11/2024

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
    Max,
    Min,
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

fn is_unary(token: &Token) -> bool {
    match token {
        Token::Function(MathFunction::Sqrt) => true,
        Token::Function(MathFunction::Exp) => true,
        Token::Function(MathFunction::Log) => true,
        Token::Function(MathFunction::Ln) => true,
        Token::Function(MathFunction::Abs) => true,
        Token::Function(MathFunction::Floor) => true,
        Token::Function(MathFunction::Sin) => true,
        Token::Function(MathFunction::Cos) => true,
        Token::Function(MathFunction::Tan) => true,
        Token::Function(MathFunction::Asin) => true,
        Token::Function(MathFunction::Acos) => true,
        Token::Function(MathFunction::Atan) => true,
        _ => false,
    }
}

fn is_str_a_number(str: &str) -> bool {
    for (i, char) in str.chars().enumerate() {
        if char == '-' && i == 0 && str.len() < 2 {
            return false;
        }
        if !char.is_digit(10) && char != '.' && char != '-' {
            return false;
        }
    }
    true
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

fn slice_to_token(input_string: &str) -> Token {
    if is_str_a_number(input_string) {
        Token::Operand(input_string.trim().parse().unwrap())
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
            // min/max
            "max" => Token::Function(MathFunction::Max),
            "min" => Token::Function(MathFunction::Min),
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
            " " => Token::Ponctuation(' '),
            _ => Token::None,
        }
    }
}

fn string_to_token(input_string: String) -> Result<Vec<Token>, String> {
    let char_vector: Vec<char> = input_string.chars().collect();
    let mut token_vector = Vec::new();
    let mut i = 0;
    while i < char_vector.len() {
        let mut j = i + 1;
        if match char_vector.get(i) {
            Some(i) => i,
            None => break,
        }
        .is_digit(10)
        {
            let mut str = String::new(); //from(*char_vector.get(i).unwrap());
            if i > 0
                && match char_vector.get(i - 1) {
                    Some(i) => i,
                    None => &'\0',
                } == &'-'
            {
                if i == 1 {
                    _ = token_vector.pop();
                    str.push('-');
                } else if !(match char_vector.get(i - 2) {
                    Some(i) => i,
                    None => &'\0',
                }
                .is_digit(10)
                    || match char_vector.get(i - 2) {
                        Some(i) => i,
                        None => &'\0',
                    } == &')')
                {
                    _ = token_vector.pop();
                    str.push('-');
                }
            }
            str.push(*char_vector.get(i).unwrap());
            while match char_vector.get(j) {
                Some(i) => i,
                None => &'\0',
            }
            .is_digit(10)
                || match char_vector.get(j) {
                    Some(i) => i,
                    None => &'\0',
                } == &'.'
            {
                str.push(*char_vector.get(j).unwrap());
                j += 1;
            }
            let token = match slice_to_token(&str) {
                Token::None => {
                    let err_message = String::from(format!("Unrecognized token \"{}\"", &str));
                    return Err(err_message);
                }
                i => i,
            };
            token_vector.push(token);
        } else if match char_vector.get(i) {
            Some(i) => i,
            None => break,
        }
        .is_alphabetic()
        {
            let mut str = String::from(*char_vector.get(i).unwrap());
            while match char_vector.get(j) {
                Some(i) => i,
                None => &'\0',
            }
            .is_alphabetic()
            {
                str.push(*char_vector.get(j).unwrap());
                j += 1;
            }
            let token = match slice_to_token(&str) {
                Token::None => {
                    let err_message = String::from(format!("Unrecognized token \"{}\"", &str));
                    return Err(err_message);
                }
                i => i,
            };
            token_vector.push(token);
        } else {
            let token = match slice_to_token(&char_vector.get(i).unwrap().to_string()) {
                Token::None => {
                    let err_message = String::from(format!(
                        "Unrecognized token \"{}\"",
                        &char_vector.get(i).unwrap().to_string()
                    ));
                    return Err(err_message);
                }
                i => i,
            };
            token_vector.push(token);
        }
        if j > i + 1 {
            i = j;
        } else {
            i += 1;
        }
    }

    Ok(token_vector)
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
                while match operator_stack.last() {
                    Some(i) => match i {
                        Token::Ponctuation('(') => false,
                        _ => true,
                    },
                    None => false,
                } && (takes_precedence(operator_stack.last().unwrap(), &Token::Operator(*o1))
                    || (same_precedence(Token::Operator(*o1), *operator_stack.last().unwrap())
                        && is_left_associative(&Token::Operator(*o1))))
                {
                    output_queue.push_back(operator_stack.pop().unwrap());
                }
                operator_stack.push(Token::Operator(*o1));
            }
            Token::Ponctuation(p) => {
                if *p == ',' {
                    while *match operator_stack.last() {
                        Some(i) => i,
                        None => return Err("Error: incorect use of ','."),
                    } != Token::Ponctuation('(')
                    {
                        output_queue.push_back(operator_stack.pop().unwrap());
                    }
                } else if *p == '(' {
                    operator_stack.push(Token::Ponctuation('('));
                } else if *p == ')' {
                    while !operator_stack.is_empty()
                        && *operator_stack.last().unwrap() != Token::Ponctuation('(')
                    {
                        output_queue.push_back(operator_stack.pop().unwrap());
                    }
                    if operator_stack.is_empty() {
                        return Err("Error: unmatching parenthesis.");
                    }
                    let _ = operator_stack.pop();
                    if is_function(match operator_stack.last() {
                        Some(v) => v,
                        None => &Token::None,
                    }) {
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
    Ok(output_queue.into())
}

fn compute(postfix: Vec<Token>) -> Result<f64, &'static str> {
    let mut operand_stack = Vec::new();
    for token in postfix {
        match token {
            Token::Operand(_) => operand_stack.push(token),
            Token::None => {}
            _ => {
                if is_unary(&token) {
                    let number: f64 = match match operand_stack.pop() {
                        Some(i) => i,
                        None => return Err("Error: missing operand."),
                    } {
                        Token::Operand(i) => i,
                        _ => 0.0,
                    };
                    match token {
                        Token::Function(MathFunction::Sqrt) => {
                            operand_stack.push(Token::Operand(number.sqrt()));
                        }
                        Token::Function(MathFunction::Exp) => {
                            operand_stack.push(Token::Operand(number.exp()));
                        }
                        Token::Function(MathFunction::Log) => {
                            operand_stack.push(Token::Operand(number.log10()));
                        }
                        Token::Function(MathFunction::Ln) => {
                            operand_stack.push(Token::Operand(number.ln()));
                        }
                        Token::Function(MathFunction::Abs) => {
                            operand_stack.push(Token::Operand(number.abs()));
                        }
                        Token::Function(MathFunction::Floor) => {
                            operand_stack.push(Token::Operand(number.floor()));
                        }
                        Token::Function(MathFunction::Sin) => {
                            operand_stack.push(Token::Operand(number.sin()));
                        }
                        Token::Function(MathFunction::Cos) => {
                            operand_stack.push(Token::Operand(number.cos()));
                        }
                        Token::Function(MathFunction::Tan) => {
                            operand_stack.push(Token::Operand(number.tan()));
                        }
                        Token::Function(MathFunction::Asin) => {
                            operand_stack.push(Token::Operand(number.asin()));
                        }
                        Token::Function(MathFunction::Acos) => {
                            operand_stack.push(Token::Operand(number.acos()));
                        }
                        Token::Function(MathFunction::Atan) => {
                            operand_stack.push(Token::Operand(number.atan()));
                        }
                        _ => {}
                    }
                } else {
                    let number2: f64 = match match operand_stack.pop() {
                        Some(i) => i,
                        None => return Err("Error: missing operand."),
                    } {
                        Token::Operand(i) => i,
                        _ => 0.0,
                    };
                    let number1: f64 = match match operand_stack.pop() {
                        Some(i) => i,
                        None => return Err("Error: missing operand."),
                    } {
                        Token::Operand(i) => i,
                        _ => 0.0,
                    };
                    match token {
                        Token::Operator('+') => {
                            operand_stack.push(Token::Operand(number1 + number2));
                        }
                        Token::Operator('-') => {
                            operand_stack.push(Token::Operand(number1 - number2));
                        }
                        Token::Operator('*') => {
                            operand_stack.push(Token::Operand(number1 * number2));
                        }
                        Token::Operator('/') => {
                            operand_stack.push(Token::Operand(number1 / number2));
                        }
                        Token::Operator('%') => {
                            operand_stack.push(Token::Operand(number1 % number2));
                        }
                        Token::Function(MathFunction::Pow) => {
                            operand_stack.push(Token::Operand(number1.powf(number2)));
                        }
                        Token::Function(MathFunction::Max) => {
                            operand_stack.push(Token::Operand(number1.max(number2)));
                        }
                        Token::Function(MathFunction::Min) => {
                            operand_stack.push(Token::Operand(number1.min(number2)));
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    match operand_stack.last().unwrap() {
        Token::Operand(i) => Ok(*i),
        _ => Err("Error: missing operand."),
    }
}

fn main() {
    println!("Calculator by Elio Fiorentini.");
    loop {
        print!(">>> ");
        io::stdout().flush().unwrap();

        let mut user_input = String::new();
        let _ = io::stdin().read_line(&mut user_input);

        let user_input = String::from(user_input.trim());

        let running = match &user_input[..] {
            "quit()" => false,
            "quit" => false,
            "q" => false,
            "stop()" => false,
            "stop" => false,
            "exit()" => false,
            "exit" => false,
            _ => true,
        };
        if !running {
            println!("Exiting...");
            break;
        }

        let infix = match string_to_token(user_input) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("{}", e);
                Vec::new()
            }
        };
        if !infix.is_empty() {
            let postfix: Vec<Token> = match infix_to_postfix(infix) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("{}", e);
                    Vec::new()
                }
            };
            if !postfix.is_empty() {
                match compute(postfix) {
                    Ok(v) => println!("{}", v),
                    Err(e) => eprintln!("{}", e),
                };
            }
        }
    }
}
