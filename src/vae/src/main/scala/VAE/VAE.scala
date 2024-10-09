package VAE

sealed trait Expr

case class Num (value: Int) extends Expr
case class Add (left: Expr, right: Expr) extends Expr
case class Sub (left: Expr, right: Expr) extends Expr
case class Val (x: String, i: Expr, b: Expr) extends Expr
case class Id (x: String) extends Expr

type Env = Map[String, Int]

def identity (e: Expr): Expr = e

def interp (e: Expr, env: Env): Int = {
	e match {
		case Num(n) => n 
		case Add(l, r) => interp(l, env) + interp(r, env)
		case Sub(l, r) => interp(l, env) - interp(r, env)
		case Val(x, i, b) => interp(b, env + (x -> interp(i, env)))
		case Id(x) => env(x)
	}
}

def main (args: Array[String]): Unit = {
	val e1 = Val("x", Num(3), Sub(Num(5), Id("x"))) // val x=3 in 5-x
	assert(interp(e1, Map()) == 2)
	println("interp(val x=3 in 5 - x, []) == " + interp(e1, Map()))

	val e2 = Val("x", e1, Add(Num(1), Id("x"))) // val x=(val x =3 in 5-x) in 1+x
	assert(interp(e2, Map()) == 3)
	println("interp(val x=(val x=3 in 5 - x, []) in 1+x) == " + interp(e2, Map()))
}






