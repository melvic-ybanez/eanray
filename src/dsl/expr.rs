use std::iter::Peekable;
use std::str::Chars;

pub type EvalResult = EvalResultF<f64>;
pub type EvalResultF<A> = Result<A, &'static str>;

pub struct Expr<'a> {
    text: &'a str,
    values: Vec<String>,
    operators: Vec<String>,
    tokens: Peekable<Chars<'a>>,
}

impl<'a> Expr<'a> {
    pub fn new(expr: &'a str) -> Self {
        Self {
            text: expr,
            values: Vec::new(),
            operators: Vec::new(),
            tokens: expr.chars().peekable(),
        }
    }

    pub fn eval(&mut self) -> EvalResult {
        while self.tokens.peek().is_some() {
            let number = self.scan_number();
            if !number.is_empty() {
                self.values.push(number);
            }

            self.scan_open_param();
            self.scan_close_param()?;

            if let Some(operator) = self.scan_operator() {
                while let Some(prev_op) = self.operators.last() {
                    if precedence(&prev_op) >= precedence(&operator.to_string()) {
                        match self.eval_op(&prev_op.clone()) {
                            Ok(result) => {
                                self.values.push(result.to_string());
                                self.operators.pop();
                            }
                            err => return err,
                        }
                    } else {
                        break;
                    }
                }
                self.operators.push(operator.to_string());
            }
        }

        self.apply_all_ops(|_| false)?;

        if let Some(result) = self.values.first() {
            let result: Result<f64, _> = result.parse();
            result.map_err(|_| "Result is not a number")
        } else {
            Err("No values available")
        }
    }

    fn apply_all_ops(&mut self, until: fn(&String) -> bool) -> EvalResultF<()> {
        while let Some(operator) = self.operators.pop() {
            if until(&operator) {
                break;
            }

            match self.eval_op(&operator) {
                Ok(result) => self.values.push(result.to_string()),
                Err(msg) => return Err(msg),
            }
        }

        Ok(())
    }

    fn skip_whitespaces(&mut self) {
        while matches!(self.tokens.peek(), Some(c) if c.is_whitespace()) {
            self.tokens.next();
        }
    }

    fn scan_open_param(&mut self) {
        self.skip_whitespaces();
        if let Some('(') = self.tokens.peek() {
            self.operators.push("(".to_string());
        }
    }

    fn scan_close_param(&mut self) -> EvalResultF<()> {
        self.skip_whitespaces();
        if let Some(')') = self.tokens.peek() {
            self.apply_all_ops(|operator| operator == "(")
        } else {
            Ok(())
        }
    }

    fn scan_number(&mut self) -> String {
        self.skip_whitespaces();

        let whole = self.scan_digits();

        if let Some('.') = self.tokens.peek() {
            self.tokens.next();
            let fractional = self.scan_digits();
            whole + "." + &fractional
        } else {
            whole
        }
    }

    fn scan_digits(&mut self) -> String {
        let mut digits = String::new();

        // we are not using `take_while` to avoid consuming an extra token
        while matches!(self.tokens.peek(), Some(&c) if is_digit(c)) {
            digits.push(self.tokens.next().unwrap());
        }

        digits
    }

    fn scan_operator(&mut self) -> Option<String> {
        self.skip_whitespaces();

        let supported_functions = vec!["cos", "sin", "tan"];
        let supported_ops = "*/+-";

        self.tokens.next().and_then(|c| {
            if c.is_alphabetic() && c.is_lowercase() {
                let mut function = c.to_string();
                while let Some(c) = self.tokens.next() {
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

    fn eval_op(&mut self, operator: &str) -> EvalResult {
        if let Some(a) = self.values.pop() {
            if let Some(b) = self.values.pop() {
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
