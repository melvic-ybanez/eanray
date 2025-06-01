use crate::core::math;
use crate::core::math::Real;
use std::iter::Peekable;
use std::str::Chars;

pub type EvalResult = EvalResultF<f64>;
pub type EvalResultF<A> = Result<A, &'static str>;

pub struct Expr<'a> {
    values: Vec<String>,
    operators: Vec<(String, OpKind)>,
    tokens: Peekable<Chars<'a>>,
}

impl<'a> Expr<'a> {
    pub fn eval_str(expr: &str) -> EvalResult {
        Expr::new(expr).eval()
    }
    
    pub fn new(expr: &'a str) -> Self {
        Self {
            values: Vec::new(),
            operators: Vec::new(),
            tokens: expr.chars().peekable(),
        }
    }

    /// Evaluates the expression.
    /// Note: Right now, it might take expressions that do not make sense and
    /// still try to compute them. In the future, we might need to modify this to
    /// support proper parsing.
    pub fn eval(&mut self) -> EvalResult {
        while self.tokens.peek().is_some() {
            if let Some(number) = self.scan_number() {
                self.values.push(number);
            }

            self.scan_open_paren();
            self.scan_close_paren()?;
            self.scan_constants();

            if let Some((operator, op_kind)) = self.scan_operator() {
                while let Some((prev_op, prev_op_kind)) = self.operators.last() {
                    let prev_op_kind = (*prev_op_kind).clone();

                    if precedence(&prev_op, &prev_op_kind)
                        >= precedence(&operator.to_string(), &op_kind)
                    {
                        match self.eval_op(&prev_op.clone(), &prev_op_kind) {
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
                self.operators.push((operator.to_string(), op_kind));
                self.tokens.next();
            }
        }

        self.apply_all_ops(|_| false)?;

        if self.values.len() > 1 {
            Err("Too many values remaining")
        } else if let Some(result) = self.values.first() {
            let result: Result<f64, _> = result.parse();
            result.map_err(|_| "Result is not a number")
        } else {
            Err("No values available")
        }
    }

    fn apply_all_ops(&mut self, until: fn(&String) -> bool) -> EvalResultF<()> {
        while let Some((operator, kind)) = self.operators.pop() {
            if until(&operator) {
                break;
            }

            match self.eval_op(&operator, &kind) {
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

    fn scan_open_paren(&mut self) {
        self.skip_whitespaces();

        match self.tokens.peek() {
            Some(p) if p.to_string() == lexemes::LEFT_PAREN => {
                self.tokens.next();
                self.operators
                    .push((lexemes::LEFT_PAREN.to_string(), OpKind::Param))
            }
            _ => (),
        }
    }

    fn scan_close_paren(&mut self) -> EvalResultF<()> {
        self.skip_whitespaces();

        match self.tokens.peek() {
            Some(c) if c.to_string() == lexemes::RIGHT_PAREN => {
                self.tokens.next();
                self.apply_all_ops(|operator| operator == lexemes::LEFT_PAREN)
            }
            _ => Ok(()),
        }
    }

    fn scan_number(&mut self) -> Option<String> {
        self.skip_whitespaces();

        self.scan_digits()
            .and_then(|whole| match self.tokens.peek() {
                Some(c) if c.to_string() == lexemes::DOT => {
                    self.tokens.next();
                    self.scan_digits()
                        .map(|fractional| whole + lexemes::DOT + &fractional)
                }
                _ => Some(whole),
            })
    }

    fn scan_digits(&mut self) -> Option<String> {
        self.scan_word(|c| is_digit(c))
    }

    fn scan_alphabetic(&mut self) -> Option<String> {
        self.scan_word(|c| c.is_alphabetic() && c.is_lowercase())
    }

    fn scan_constants(&mut self) {
        self.skip_whitespaces();

        if let Some(lexeme) = self.scan_alphabetic() {
            let supported_functions = vec![lexemes::COS, lexemes::SIN, lexemes::TAN];

            match lexeme.as_str() {
                lexemes::PI => self.values.push(math::PI.to_string()),
                func if supported_functions.contains(&func) => {
                    self.operators.push((func.to_string(), OpKind::Unary))
                }
                _ => (),
            }
        }
    }

    fn scan_word(&mut self, filter: fn(char) -> bool) -> Option<String> {
        let mut chars = String::new();

        // we are not using `take_while` to avoid consuming an extra token
        while matches!(self.tokens.peek(), Some(&c) if filter(c)) {
            chars.push(self.tokens.next().unwrap());
        }

        if chars.is_empty() { None } else { Some(chars) }
    }

    fn scan_operator(&mut self) -> Option<(String, OpKind)> {
        self.skip_whitespaces();

        let supported_binary_ops = vec![
            lexemes::TIMES,
            lexemes::DIVIDE,
            lexemes::PLUS,
            lexemes::MINUS,
        ]
        .join("");

        match self.tokens.peek() {
            Some(c) => {
                let cstr = c.to_string();
                let bin_count = self
                    .operators
                    .iter()
                    .filter(|(_, kind)| matches!(kind, OpKind::Binary))
                    .count();

                if bin_count == self.values.len()
                    && (cstr == lexemes::MINUS || cstr == lexemes::PLUS)
                {
                    Some((cstr, OpKind::Unary))
                } else if supported_binary_ops.contains(*c) {
                    if bin_count < self.values.len() - 1{
                        self.tokens.next();
                        None
                    } else {
                        Some((cstr, OpKind::Binary))
                    }
                } else {
                    None
                }
            }
            None => None,
        }
    }

    fn eval_op(&mut self, operator: &str, kind: &OpKind) -> EvalResult {
        match kind {
            OpKind::Param => Ok(Real::NAN),
            OpKind::Unary => self.eval_unary(operator),
            OpKind::Binary => self.eval_binop(operator),
        }
    }

    fn eval_unary(&mut self, operator: &str) -> EvalResult {
        if let Some(value) = self.values.pop() {
            let value = value.parse().unwrap();
            let result = match operator {
                lexemes::COS => Real::cos(value),
                lexemes::PLUS => value,
                lexemes::MINUS => -value,
                _ => return Err("Unknown function"),
            };
            Ok(result)
        } else {
            errors::NO_VALUE_PROVIDED
        }
    }

    fn eval_binop(&mut self, operator: &str) -> EvalResult {
        if let Some(a) = self.values.pop() {
            if let Some(b) = self.values.pop() {
                // values are expected to be numeric
                let a: Real = a.parse().unwrap();
                let b: Real = b.parse().unwrap();

                let result = match operator {
                    lexemes::TIMES => b * a,
                    lexemes::DIVIDE => b / a,
                    lexemes::PLUS => b + a,
                    lexemes::MINUS => b - a,
                    _ => return Err("Unknown operator"),
                };
                Ok(result)
            } else {
                Err("Only one value provided")
            }
        } else {
            errors::NO_VALUE_PROVIDED
        }
    }
}

#[derive(Debug, Clone)]
enum OpKind {
    Unary,
    Binary,
    Param,
}

fn precedence(operator: &str, kind: &OpKind) -> u16 {
    match kind {
        OpKind::Unary => 3,
        OpKind::Binary => match operator {
            lexemes::PLUS | lexemes::MINUS => 1,
            lexemes::TIMES | lexemes::DIVIDE => 2,
            lexemes::COS | lexemes::SIN | lexemes::TAN => 3,
            _ => 0,
        },
        _ => 0,
    }
}

fn is_digit(c: char) -> bool {
    c.is_digit(10)
}

mod lexemes {
    pub const PI: &str = "pi";

    pub const COS: &str = "cos";
    pub const SIN: &str = "sin";
    pub const TAN: &str = "tan";

    pub const LEFT_PAREN: &str = "(";
    pub const RIGHT_PAREN: &str = ")";

    pub const PLUS: &str = "+";
    pub const MINUS: &str = "-";
    pub const TIMES: &str = "*";
    pub const DIVIDE: &str = "/";

    pub const DOT: &str = ".";
}

mod errors {
    use crate::dsl::expr::EvalResult;

    pub const NO_VALUE_PROVIDED: EvalResult = Err("No value provided");
}
