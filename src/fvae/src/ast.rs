use std::fmt ;
use std::collections::BTreeMap ;

#[derive(Debug, Clone)]
pub enum Expr {
    Num(i32),
    Op(Box<Expr>, Opr, Box<Expr>),
    Let(String, Box<Expr>, Box<Expr>),
    Use(String),
    Lambda(String, Box<Expr>),
    App(Box<Expr>, Box<Expr>),
    If0(Box<Expr>, Box<Expr>, Box<Expr>),
}

#[derive(Debug, Copy, Clone)]
pub enum Opr {
    Add,
    Sub,
}

#[derive(Debug, Clone)]
pub enum Value {
    NumV(i32),
    CloV(String, Box<Expr>, BTreeMap<String,Value>),
}

impl fmt::Display for Expr 
{
	fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Expr::Num(n) => write!(f, "{}", n),
			Expr::Op(l, op, r) => write!(f, "({} {} {})", l, op, r),
			Expr::Let(x, id, e) => write!(f, "(let {}={} in {})", x, id, e),
			Expr::Use(id) => write!(f, "{}", id),
                        Expr::Lambda(id, e) => write!(f, "(lambda {} to {})", id, e),
                        Expr::App(e1, e2) => write!(f, "{}({})", e1, e2),
                        Expr::If0(e1, e2, e3) => write!(f, "(if0 {} {} {})", e1, e2, e3),
		}
	}
}

impl fmt::Display for Opr 
{
	fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Opr::Add => write!(f, "+"),
			Opr::Sub => write!(f, "-")
		}
	}
}

impl fmt::Display for Value 
{
	fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
                    Value::NumV(n) => write!(f, "{}", n),
                    Value::CloV(param, expr, _) => write!(f, "<(lambda {} to {}), ..>", param, expr),
		}
	}
}

