use std::iter::Peekable;
use std::str::Chars;

pub type EvalResult = EvalResultF<f64>;
pub type EvalResultF<A> = Result<A, &'static str>;

pub fn eval(expr: &str) -> EvalResult {
    let values = &mut Vec::<String>::new();
    let operators = &mut Vec::<String>::new();
    let tokens = &mut expr.chars().peekable();

    while tokens.peek().is_some() {
        let number = scan_number(tokens);
        if !number.is_empty() {
            values.push(number);
        }

        if let Some(operator) = scan_operator(tokens) {
            while let Some(prev_op) = operators.last() {
                if precedence(&prev_op) >= precedence(&operator.to_string()) {
                    match eval_op(&prev_op, values) {
                        Ok(result) => {
                            values.push(result.to_string());
                            operators.pop();
                        },
                        err => return err,
                    }
                } else {
                    break;
                }
            }
            operators.push(operator.to_string());
        }
    }

    while let Some(operator) = operators.pop() {
        match eval_op(&operator, values) {
            Ok(result) => values.push(result.to_string()),
            err => return err,
        }
    }

    if let Some(result) = values.first() {
        let result: Result<f64, _> = result.parse();
        result.map_err(|_| "Result is not a number")
    } else {
        Err("No values available")
    }
}

fn skip_whitespaces(expr: &mut Peekable<Chars>) {
    while matches!(expr.peek(), Some(c) if c.is_whitespace()) {
        expr.next();
    }
}

fn scan_number(expr: &mut Peekable<Chars>) -> String {
    skip_whitespaces(expr);

    let whole = scan_digits(expr);

    if let Some('.') = expr.peek() {
        expr.next();
        let fractional = scan_digits(expr);
        whole + "." + &fractional
    } else {
        whole
    }
}

fn scan_digits(expr: &mut Peekable<Chars>) -> String {
    let mut digits = String::new();

    // we are not using `take_while` to avoid consuming an extra token
    while matches!(expr.peek(), Some(&c) if is_digit(c)) {
        digits.push(expr.next().unwrap());
    }

    digits
}

fn scan_operator(expr: &mut Peekable<Chars>) -> Option<String> {
    skip_whitespaces(expr);

    let supported_functions = vec!["cos", "sin", "tan"];
    let supported_ops = "*/+-";

    expr.next().and_then(|c| {
        if c.is_alphabetic() && c.is_lowercase() {
            let mut function = c.to_string();
            while let Some(c) = expr.next() {
                if c.is_alphabetic() {
                    function.push(c);
                }
            }
            if supported_functions.contains(&function.as_str()) {
                Some(function)
            } else {
                None
            }
        } else {
            if supported_ops.contains(c) {
                Some(c.to_string())
            } else {
                None
            }
        }
    })
}

fn eval_op(operator: &str, values: &mut Vec<String>) -> EvalResult {
    if let Some(a) = values.pop() {
        if let Some(b) = values.pop() {
            // values are expected to be numeric
            let a: f64 = a.parse().unwrap();
            let b: f64 = b.parse().unwrap();

            let result = match operator {
                "*" => b * a,
                "/" => b / a,
                "+" => b + a,
                "-" => b - a,
                _ => f64::NAN,
            };
            Ok(result)
        } else {
            Err("Not enough values to evaluate")
        }
    } else {
        Err("No values to evaluate")
    }
}

fn precedence(operator: &str) -> u16 {
    match operator {
        "+" | "-" => 1,
        "*" | "/" => 2,
        _ => 0,
    }
}

fn is_digit(c: char) -> bool {
    c.is_digit(10)
}
