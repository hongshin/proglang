use std::fmt ;

#[derive(Debug, Clone)]
pub enum Expr {
    Num(i32),
    Op(Box<Expr>, Opcode, Box<Expr>),
    Val(String, Box<Expr>, Box<Expr>),
    Ref(String)
}

#[derive(Debug, Copy, Clone)]
pub enum Opcode {
    Add,
    Sub,
}

//#[derive(Debug, Clone)]
//pub struct Ref(pub String) ;


//pub type ExprBox = Box<Expr> ;

pub fn add (l: Box<Expr>, r: Box<Expr>) -> Box<Expr> 
{
    Box::new(Expr::Op(l, Opcode::Add, r))
}

pub fn sub (l: Box<Expr>, r: Box<Expr>) -> Box<Expr>
{
    Box::new(Expr::Op(l, Opcode::Sub, r))
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
			Expr::Op(l, op, r) => write!(f, "({} {} {})", l, op, r),
            Expr::Ref(id) => write!(f, "{}", id),
            Expr::Val(x, id, bd) => write!(f, "val {}={} in {}", x, id, bd)
		}
	}
}

impl fmt::Display for Opcode 
{
	fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Opcode::Add => write!(f, "+"),
			Opcode::Sub => write!(f, "-")
		}
	}
}
