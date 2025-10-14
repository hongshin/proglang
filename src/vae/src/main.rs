use std::collections::BTreeMap ;

use lalrpop_util::lalrpop_mod ;

pub mod ast ;
use ast::Expr ;
use ast::Expr::{Op, Num, Val, Use} ;
use ast::Opr::{Add, Sub} ;

lalrpop_mod!(pub vae) ;

fn interp (e: Box<Expr>, env: &BTreeMap::<String, i32>) -> i32 
{
    match *e {
        Op(l, Add, r) => interp(l, env) + interp(r, env),
        Op(l, Sub, r) => interp(l, env) - interp(r, env),
        Num(n) => n,
        Use(id) => *env.get(&id).unwrap(),
        Val(id, v, e) => {
            let mut nenv = env.clone() ;
            nenv.insert(id, interp(v, env)) ;
            interp(e, &nenv)
        }
    } 
}


fn main() 
{
    let env = BTreeMap::<String, i32>::new() ;
    let e0 = vae::ExprParser::new().parse("val i=3 in (i + (1 + i))").unwrap() ;
    println!("e0: {}", e0) ;
    println!("e0: {:?}", e0) ;
    println!("interp(e0,[]): {}", interp(e0, &env)) ;

    let e1 = vae::ExprParser::new().parse("val i=3 in (i + val i=5 in (1 + i))").unwrap() ;
    println!("e1: {}", e1) ;
    println!("e1: {:?}", e1) ;
    println!("interp(e1,[]): {}", interp(e1, &env)) ;
}
