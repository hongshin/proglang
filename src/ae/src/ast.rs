use std::fmt ;

#[derive(Debug, Clone)]
pub enum Expr {
    Num(i32),
    Op(Box<Expr>, Opr, Box<Expr>),
}

#[derive(Debug, Copy, Clone)]
pub enum Opr {
    Add,
    Sub,
}

//pub type ExprBox = Box<Expr> ;

pub fn add (l: Box<Expr>, r: Box<Expr>) -> Box<Expr> 
{
    Box::new(Expr::Op(l, Opr::Add, r))
}

pub fn sub (l: Box<Expr>, r: Box<Expr>) -> Box<Expr>
{
    Box::new(Expr::Op(l, Opr::Sub, r))
}

pub fn num (n: i32) -> Box<Expr>
{
    Box::new(Expr::Num(n))
}


impl fmt::Display for Expr 
{
	fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Expr::Num(n) => write!(f, "{}", n),
			Expr::Op(l, op, r) => write!(f, "({} {} {})", l, op, r)
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

