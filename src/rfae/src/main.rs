use std::rc::Rc ;
use std::cell::RefCell;
use std::collections::BTreeMap ;
use lalrpop_util::lalrpop_mod ;

pub mod ast ;
use ast::Expr ;
use ast::Expr::{Op, Num, Let, Use, Lambda, App, Def, If0} ;
use ast::Opr::{Add, Sub} ;
use ast::Value ;
use ast::Value::{NumV, CloV} ;

lalrpop_mod!(pub rfae) ;

fn interp (e: Box<Expr>, env: Rc<RefCell<BTreeMap::<String, Value>>>) -> Value
{
    match *e {
        Op(l, Add, r) => {
            let vl = interp(l.clone(), Rc::clone(&env)) ;
            let vr = interp(r.clone(), Rc::clone(&env)) ;

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
            let vl = interp(l.clone(), Rc::clone(&env)) ;
            let vr = interp(r.clone(), Rc::clone(&env)) ;

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
            env.borrow().get(&id).unwrap().clone()
        },
        Let(id, v, e) => {
            let nenv = Rc::new(RefCell::new(env.borrow().clone())) ;
            nenv.borrow_mut().insert(id, interp(v, env)) ;
            interp(e, nenv)
        },
        Lambda(p, e) => {
            let nenv = Rc::new(RefCell::new(env.borrow().clone())) ;
            CloV(p.clone(), e.clone(), Rc::clone(&nenv))
        },
        App(e1, e2) => {
            let v1 = interp(e1.clone(), Rc::clone(&env)) ;
            let v2 = interp(e2.clone(), Rc::clone(&env)) ;

            match v1 {
                CloV(pid, le, cenv) => {
                    let nenv = Rc::new(RefCell::new(cenv.borrow().clone())) ;
                    nenv.borrow_mut().insert(pid, v2) ;
                    interp(le, nenv)
                },
                _ => {
                    panic!("Error: {} is not a closure", e1)
                }
            }

        },
        Def(fid, pid, fe, e) => {            
            let nenv = Rc::new(RefCell::new(env.borrow().clone())) ;
            let fun = CloV(pid, fe, Rc::clone(&nenv)) ;
            nenv.borrow_mut().insert(fid, fun) ;
            interp(e, Rc::clone(&nenv))
        },
        If0(ec, et, ef) => {
            let vc = interp(ec.clone(), Rc::clone(&env)) ;

            match vc{
                Value::NumV(0) => {
                    interp(et.clone(), Rc::clone(&env)) 
                },
                Value::NumV(_) => {
                    interp(ef.clone(), Rc::clone(&env))
                },
                _ => {
                    panic!("Error: if condition is not a number")
                }
            }
        }
    } 
}


fn main() 
{
    let env = Rc::new(RefCell::new(BTreeMap::<String, Value>::new())) ;
    let e0 = rfae::ExprParser::new().parse("(def sum(x)=(if0 x 0 (x + (sum(x - 1)))) in (sum(5)))").unwrap() ;
    println!("e0: {}", e0) ;
    println!("e0: {:?}", e0) ;
    println!("interp(e0,[]): {}", interp(e0, Rc::clone(&env))) ;

    /*   
    let e1 = rfae::ExprParser::new().parse("(let i=3 in (let f=(lambda x to (x+i)) in (let i=4 in f(i))))").unwrap() ;
    println!("e1: {}", e1) ;
    println!("e1: {:?}", e1) ;
    println!("interp(e1,[]): {}", interp(e1, Rc::clone(&env))) ;
    */
    
}
