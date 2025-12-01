use std::collections::BTreeMap ;

use lalrpop_util::lalrpop_mod ;

pub mod ast ;
use ast::Expr ;
use ast::Expr::{Op, Num, Let, Use, Lambda, App, If0} ;
use ast::Opr::{Add, Sub} ;
use ast::Value ;
use ast::Value::{NumV, CloV} ;

lalrpop_mod!(pub fvae) ;

fn interp (e: Box<Expr>, env: &BTreeMap::<String, Value>) -> Value
{
    match *e {
        Op(l, Add, r) => {
            let vl = interp(l.clone(), env) ;
            let vr = interp(r.clone(), env) ;

            match (vl, vr) {
                (NumV(nl), NumV(nr))  => {
                    Value::NumV(nl + nr)
                },
                (CloV(_, _, _), _) => {
                    panic!("Error: {} is not a number", *r)
                },
                (_, _) => {
                    panic!("Error: {} is not a number", *l)
                }
            }
        },
        Op(l, Sub, r) => {
            let vl = interp(l.clone(), env) ;
            let vr = interp(r.clone(), env) ;

            match (vl, vr) {
                (NumV(nl), NumV(nr)) => {
                    Value::NumV(nl - nr)
                },
                (CloV(_, _, _), _) => {
                    panic!("Error: {} is not a number", *r)
                },
                (_, _) => {
                    panic!("Error: {} is not a number", *l)
                }
            }
        },
        Num(n) => {
            Value::NumV(n)
        },
        Use(id) => {
            env.get(&id).unwrap().clone()
        },
        Let(id, v, e) => {
            let mut nenv = env.clone() ;
            nenv.insert(id, interp(v, env)) ;
            interp(e, &nenv)
        },
        Lambda(p, e) => {
            CloV(p.clone(), e.clone(), env.clone())
        },
        App(e1, e2) => {
            let v1 = interp(e1.clone(), env) ;
            let v2 = interp(e2.clone(), env) ;

            match v1 {
                CloV(pid, le, cenv) => {
                    let mut nenv = cenv.clone() ;
                    nenv.insert(pid, v2) ;
                    interp(le, &nenv)
                },
                _ => {
                    panic!("Error: {} is not a closure", e1)
                }
            }
        },
        If0(e1, e2, e3) => {
            let c = interp(e1.clone(), env) ;

            match c {
                NumV(0) => {
                    interp(e2.clone(), env) 
                },
                NumV(_) => {
                    interp(e3.clone(), env) 
                },
                _ => {
                    panic!("Error: if-condition is not a number") 
                }
            }
        }
    } 
}


fn main() 
{
    let env = BTreeMap::<String, Value>::new() ;    

    let e0 = fvae::ExprParser::new().parse("
        (let f = (lambda x to (x + x)) in (f 1))").unwrap() ;
    println!("e0: {}", e0) ;
    println!("e0: {:?}", e0) ;
    println!("interp(e0,[]): {}", interp(e0, &env)) ;
    println!("--------------------------") ;

    let e1 = fvae::ExprParser::new().parse("(let i=3 in (let f=(lambda x to (x+i)) in (let i=4 in (f i))))").unwrap() ;
    println!("e1: {}", e1) ;
    println!("e1: {:?}", e1) ;
    println!("interp(e1,[]): {}", interp(e1, &env)) ;
    println!("--------------------------") ;
    
    let e2 = fvae::ExprParser::new().parse("
    (let Z=(lambda h to ((lambda x to (h (lambda v to ((x x) v)))) 
                         (lambda x to (h (lambda v to ((x x) v)))))) in
        (let sum=(lambda f to (lambda v to (if0 v 0 (v + (f (v - 1)))))) in
            ((Z sum) 10)))
    ").unwrap() ;
    println!("e1: {}", e2) ;
    println!("e1: {:?}", e2) ;
    println!("interp(e1,[]): {}", interp(e2, &env)) ;
    
}

