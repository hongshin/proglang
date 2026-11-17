use ExprType::Num ;
use ExprType::Expr ;
use Opr::Plus ;
use Opr::Product ;

enum Opr {
    Plus,
    Product
}

enum ExprType {
    Num(i32),
    Expr(Box<ExprType>, Opr, Box<ExprType>)
}

fn eval (e: &ExprType) -> i32 {
    match e {
        Num(n) => *n,
        Expr(l, Plus, r) => eval(l) + eval(r),
        Expr(l, Product, r) => eval(l) * eval(r),
    }
}

fn main () 
{
    let e = Expr(Box::new(Expr(Box::new(Num(3)), Product, Box::new(Num(2)))), Plus, Box::new(Num(2))) ;

    println!("{}", eval(&e)) ;
}
