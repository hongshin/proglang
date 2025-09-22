use std::fmt ;

#[derive(Debug, Clone)]
pub enum Expr {
    Num(i32),
    Op(Box<Expr>, Opcode, Box<Expr>),
    Neg(Box<Expr>),
}

#[derive(Debug, Copy, Clone)]
pub enum Opcode {
    Add,
    Sub,
}

//pub type ExprBox = Box<Expr> ;


pub fn add (l: Box<Expr>, r: Box<Expr>) -> Box<Expr> {
    Box::new(Expr::Op(l, Opcode::Add, r))
}

pub fn sub (l: Box<Expr>, r: Box<Expr>) -> Box<Expr> {
    Box::new(Expr::Op(l, Opcode::Sub, r))
}

pub fn num (n: i32) -> Box<Expr> {
    Box::new(Expr::Num(n))
}

pub fn neg (e: Box<Expr>) -> Box<Expr> {
    Box::new(Expr::Neg(e)) 
}


impl fmt::Display for Expr {
	fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Expr::Num(n) => write!(f, "{}", n),
			Expr::Op(l, op, r) => write!(f, "({} {} {})", l, op, r),
            Expr::Neg(e) => write!(f, "-({})", e)
		}
	}
}

impl fmt::Display for Opcode {
	fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Opcode::Add => write!(f, "+"),
			Opcode::Sub => write!(f, "-")
		}
	}
}

