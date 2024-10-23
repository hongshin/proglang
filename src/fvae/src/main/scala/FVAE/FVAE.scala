package FVAE

sealed trait Expr
case class Num (value: Int) extends Expr
case class Add (left: Expr, right: Expr) extends Expr
case class Sub (left: Expr, right: Expr) extends Expr
case class Val (x: String, i: Expr, b: Expr) extends Expr
case class Id (x: String) extends Expr
case class Fun (x: String, b: Expr) extends Expr 
case class App (f: Expr, a: Expr) extends Expr

type Env = Map[String, Value]

sealed trait Value
case class NumV (n: Int) extends Value
case class CloV (p: String, b: Expr, e: Env) extends Value
case class RunErr (msg: String) extends Value


def interp (e: Expr, env: Env): Value = {
	e match {
		case Num(n) => NumV(n)
		case Add(l, r) => {
			val NumV(n) = interp(l, env)
			val NumV(m) = interp(r, env)
			
			NumV(n + m)
		}
		case Sub(l, r) => {
			val NumV(n) = interp(l, env)
			val NumV(m) = interp(r, env)
			
			NumV(n - m)
		}
		case Val(x, i, b) => {
			interp(b, env + (x -> interp(i, env)))
		}
		case Id(x) => env(x)
		case Fun(x, b) => CloV(x, b, env)
		case App(f, a) => {
			val CloV(x, b, cEnv) = interp(f, env)
			interp(b, cEnv + (x -> interp(a, env)))
		}
	}
}

def main (args: Array[String]): Unit = {
	val f = Fun("x", Add(Add(Id("x"), Id("x")), Id("x"))) ;
	val e = Val("triple", f, App(Id("triple"), Id("triple"))) ;

	println(interp(e, Map()))
}
