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
enum Token {
    Number(i32),
    Bool(bool),
    Exp { op: Operand, t1: Rc<Token>, t2: Rc<Token> },
    If { cond: Rc<Token>, if_value: Rc<Token>, else_value: Rc<Token> },
}

impl Eval for Token {
    fn eval(&self) -> Result<Value, EvalError> {
        match self {
            Token::Number(v) => Ok(Value::Number(v.clone())),
            Token::Exp { op, t1, t2 } => {
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
            Token::Bool(v) => Ok(Value::Bool(v.clone())),
            Token::If { cond, if_value, else_value } => {
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

fn main() -> Result<()> {
    // (+ 2 3) == 5
    let t = Token::Exp {
        op: Operand::Plus,
        t1: Rc::new(Token::Number(2)),
        t2: Rc::new(Token::Number(3)),
    };
    let ret = t.eval()?;
    println!("{:?}", ret);

    // (- 10 3) == 7
    let t = Token::Exp {
        op: Operand::Minus,
        t1: Rc::new(Token::Number(10)),
        t2: Rc::new(Token::Number(3)),
    };
    let ret = t.eval()?;
    println!("{:?}", ret);

    // (if true 100 200) == 100
    let t = Token::If {
        cond: Rc::new(Token::Bool(true)),
        if_value: Rc::new(Token::Number(100)),
        else_value: Rc::new(Token::Number(200)),
    };
    let ret = t.eval()?;
    println!("{:?}", ret);

    // (if 100 100 200) == 100
    let t = Token::If {
        cond: Rc::new(Token::Number(100)),
        if_value: Rc::new(Token::Number(100)),
        else_value: Rc::new(Token::Number(200)),
    };
    t.eval()?; // Error: if cond requires boolean: Number(100)

    Ok(())
}
