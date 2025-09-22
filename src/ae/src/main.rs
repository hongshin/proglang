use lalrpop_util::lalrpop_mod ;

pub mod ast ;
use ast::Expr ;
use ast::{add, sub, num} ;
use ast::Expr::{Op, Num} ;
use ast::Opcode::{Add, Sub} ;

lalrpop_mod!(pub ae) ;

fn interp (e: Box<Expr>) -> i32 
{
    match *e {
        Op(l, Add, r) => interp(l) + interp(r),
        Op(l, Sub, r) => interp(l) - interp(r),
        Num(n) => n
    } 
}


fn main() 
{
    let e0 = Box::new( Op( Box::new( Op(Box::new( Num(5) ), 
                                     Sub, 
                                     Box::new( Num(1) )) 
                                    ), 
                           Add, 
                           Box::new( Num(3) ))
                      ) ;
    println!("e0: {}", e0) ;
    println!("interp(e0): {}", interp(e0)) ;
    println!("") ;

    let e1 = add(sub(num(5), num(1)), num(3)) ;    
    println!("e1: {}", e1) ;
    println!("interp(e1): {}", interp(e1)) ;
    println!("") ;

    let e2 = ae::ExprParser::new().parse("((5 - 1) + 3)").unwrap() ;
	println!("e2: {}", e2) ;
    println!("interp(e2): {}", interp(e2)) ;
}
