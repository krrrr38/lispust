use std::fmt::{Display, Formatter};
use std::rc::Rc;

enum Operand {
    Plus,
    Minus,
}

enum Token {
    Number(i32),
    Exp { op: Operand, t1: Rc<Token>, t2: Rc<Token> },
}

impl Eval for Token {
    fn eval(&self) -> Value {
        match self {
            Token::Number(v) => Value::Number(v.clone()),
            Token::Exp { op, t1, t2 } => match op {
                Operand::Plus => match (t1.eval(), t2.eval()) {
                    (Value::Number(v1), Value::Number(v2)) => Value::Number(v1 + v2)
                }
                Operand::Minus => match (t1.eval(), t2.eval()) {
                    (Value::Number(v1), Value::Number(v2)) => Value::Number(v1 - v2)
                }
            }
        }
    }
}

#[derive(Debug)]
enum Value {
    Number(i32),
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Number(v1), Value::Number(v2)) => v1 == v2,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(v) => write!(f, "Number({})", v)
        }
    }
}

trait Eval {
    fn eval(&self) -> Value;
}

fn main() {
    // (+ 2 3) == 5
    let t1 = Token::Exp {
        op: Operand::Plus,
        t1: Rc::new(Token::Number(2)),
        t2: Rc::new(Token::Number(3)),
    };
    let ret1 = t1.eval();
    assert_eq!(ret1, Value::Number(5));
    println!("{}", ret1);

    // (- 10 3) == 7
    let t2 = Token::Exp {
        op: Operand::Minus,
        t1: Rc::new(Token::Number(10)),
        t2: Rc::new(Token::Number(3)),
    };
    let ret2 = t2.eval();
    assert_eq!(ret2, Value::Number(7));
    println!("{}", ret2);
}
