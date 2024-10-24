package RFAE

sealed trait Expr
case class Num (value: Int) extends Expr
case class Add (left: Expr, right: Expr) extends Expr
case class Sub (left: Expr, right: Expr) extends Expr
case class Val (x: String, i: Expr, b: Expr) extends Expr
case class Id (x: String) extends Expr
case class Fun (x: String, b: Expr) extends Expr 
case class App (f: Expr, a: Expr) extends Expr

case class Ifz (c: Expr, t: Expr, f: Expr) extends Expr
case class Def (f: String, x: String, b: Expr, e: Expr) extends Expr


type Env = Map[String, Value]

sealed trait Value
case class NumV (n: Int) extends Value

case class CloV (x: String, b: Expr, var env: Env) extends Value

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

		case Ifz(c, t, f) => {
			if (interp(c, env) == NumV(0))
				interp(t, env)
			else
				interp(f, env)
		}

		case Def(f, x, b, e) => {
			val clos = CloV(x, b, env)
			val nenv = env + (f -> clos)
			clos.env = nenv 

			interp(e, nenv)
		}
	}
}

def main (args: Array[String]): Unit = {
// def sum(x)=ifz x 0 (x + sum(x -1)) in sum 10

	val f = Ifz(Id("x"), Num(0), Add(Id("x"), App(Id("sum"), Sub(Id("x"), Num(1)))))
	val e = Def("sum", "x", f, App(Id("sum"), Num(10))) ;

	println(interp(e, Map()))
}
