use std::rc::Rc;

use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug)]
enum EvalError {
    #[error("operand {0} requires number values: {1:?} {2:?}")]
    InvalidOperandTypes(String, Value, Value),
    #[error("if cond requires boolean: {0:?}")]
    InvalidIfCondType(Value),
}

#[derive(Debug)]
enum Operand {
    Plus,
    Minus,
}

#[derive(Debug)]
enum Exp {
    Number(i32),
    Bool(bool),
    Calc { op: Operand, t1: Rc<Exp>, t2: Rc<Exp> },
    If { cond: Rc<Exp>, if_value: Rc<Exp>, else_value: Rc<Exp> },
    List(Vec<Exp>),
}

impl Eval for Exp {
    fn eval(&self) -> Result<Value, EvalError> {
        match self {
            Exp::Number(v) => Ok(Value::Number(v.clone())),
            Exp::Calc { op, t1, t2 } => {
                let v1 = t1.eval()?;
                let v2 = t2.eval()?;
                match op {
                    Operand::Plus => match (v1, v2) {
                        (Value::Number(v1), Value::Number(v2)) => Ok(Value::Number(v1 + v2)),
                        (v1, v2) => Err(EvalError::InvalidOperandTypes(String::from("plus"), v1, v2)),
                    }
                    Operand::Minus => match (v1, v2) {
                        (Value::Number(v1), Value::Number(v2)) => Ok(Value::Number(v1 - v2)),
                        (v1, v2) => Err(EvalError::InvalidOperandTypes(String::from("minus"), v1, v2)),
                    }
                }
            }
            Exp::Bool(v) => Ok(Value::Bool(v.clone())),
            Exp::If { cond, if_value, else_value } => {
                let cond_v = cond.eval()?;
                match cond_v {
                    Value::Bool(v) => if v {
                        if_value.eval()
                    } else {
                        else_value.eval()
                    }
                    _ => Err(EvalError::InvalidIfCondType(cond_v)),
                }
            }
            Exp::List(_) => Ok(Value::Bool(true)),
        }
    }
}

#[derive(Debug)]
enum Value {
    Number(i32),
    Bool(bool),
}

trait Eval {
    fn eval(&self) -> Result<Value, EvalError>;
}

#[derive(Error, Debug)]
enum ParserError {
    #[error("{0}")]
    Invalid(String),
}

trait Parser<T> {
    fn parse(&self, _: T) -> Result<Exp, ParserError>;
}

struct StringParser {}

impl StringParser {
    fn parse_tokens<'a>(&self, tokens: &'a [String]) -> Result<(Exp, &'a [String]), ParserError> {
        match tokens.split_first() {
            Some((head, tail)) => {
                if head == "(" {
                    self.parse_exps(tail)
                } else if head == ")" {
                    Err(ParserError::Invalid("too many close parenthesis".to_string()))
                } else {
                    self.parse_exp(head).map(|e| (e, tail))
                }
            }
            None => Err(ParserError::Invalid("no parenthesis in tokens".to_string()))
        }
    }

    fn parse_exps<'a>(&self, values: &'a [String]) -> Result<(Exp, &'a [String]), ParserError> {
        let mut current = values;
        let mut exps = vec![];
        loop {
            match current.split_first() {
                Some((head, tail)) => match &head[..] {
                    ")" => return Ok((Exp::List(exps), tail)),
                    "+" => {
                        let (t1, remain) = self.parse_tokens(tail)?;
                        let (t2, remain) = self.parse_tokens(remain)?;
                        let exp = Exp::Calc { op: Operand::Plus, t1: Rc::new(t1), t2: Rc::new(t2) };
                        return Ok((exp, remain));
                    }
                    "-" => {
                        let (t1, remain) = self.parse_tokens(tail)?;
                        let (t2, remain) = self.parse_tokens(remain)?;
                        let exp = Exp::Calc { op: Operand::Minus, t1: Rc::new(t1), t2: Rc::new(t2) };
                        return Ok((exp, remain));
                    }
                    "if" => {
                        let (cond, remain) = self.parse_tokens(tail)?;
                        let (if_value, remain) = self.parse_tokens(remain)?;
                        let (else_value, remain) = self.parse_tokens(remain)?;
                        let exp = Exp::If { cond: Rc::new(cond), if_value: Rc::new(if_value), else_value: Rc::new(else_value) };
                        return Ok((exp, remain));
                    }
                    _ => {
                        let (exp, remain) = self.parse_tokens(current)?;
                        exps.push(exp);
                        current = remain
                    }
                },
                None => return Err(ParserError::Invalid("no parenthesis in exp".to_string()))
            }
        }
    }

    fn parse_exp(&self, v: &String) -> Result<Exp, ParserError> {
        match &v[..] {
            "true" => Ok(Exp::Bool(true)),
            "false" => Ok(Exp::Bool(true)),
            v => {
                let num: i32 = v.parse().map_err(|e| ParserError::Invalid(format!("{} is not number: {:?}", v, e)))?;
                Ok(Exp::Number(num))
            }
        }
    }
}

impl Parser<&str> for StringParser {
    fn parse(&self, input: &str) -> Result<Exp, ParserError> {
        let tokens = input
            .replace("(", " ( ")
            .replace(")", " ) ")
            .split_whitespace()
            .map(|v| v.trim())
            .filter(|&v| !v.is_empty())
            .map(|v| v.to_string())
            .collect::<Vec<String>>();
        self.parse_tokens(&tokens).map(|(e, _)| e)
    }
}

fn run(input: &str) -> Result<()> {
    let parser = StringParser {};
    let exp = parser.parse(input)?;
    let ret = exp.eval()?;
    println!("{:?}", ret);
    Ok(())
}

fn main() -> Result<()> {
    run("(+ 2 3)")?;
    run("(- 10 3)")?;
    run("(if true 100 200)")?;
    Ok(())
}
