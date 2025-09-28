use std::collections::BTreeMap ;

use lalrpop_util::lalrpop_mod ;

pub mod ast ;
use ast::Expr ;
use ast::Expr::{Op, Num, Ref, Val} ;
use ast::Opcode::{Add, Sub} ;

lalrpop_mod!(pub vae) ;

fn interp (e: Box<Expr>, env: &BTreeMap::<String, i32>) -> i32 
{
    match *e {
        Op(l, Add, r) => interp(l, env) + interp(r, env),
        Op(l, Sub, r) => interp(l, env) - interp(r, env),
        Num(n) => n,
        Ref(id) => *env.get(&id).unwrap(),
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
    let e = vae::ExprParser::new().parse("val i=3 in (i + (1 + i))").unwrap() ;
    println!("e: {}", e) ;
    println!("interp(e,env): {}", interp(e, &env)) ;

    let e = vae::ExprParser::new().parse("val i=3 in (i + val i=5 in (1 + i))").unwrap() ;
    println!("e: {}", e) ;
    println!("interp(e,env): {}", interp(e, &env)) ;
}
