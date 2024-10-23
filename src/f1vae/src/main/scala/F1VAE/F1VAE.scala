package F1VAE

sealed trait Expr

case class Num (value: Int) extends Expr
case class Add (left: Expr, right: Expr) extends Expr
case class Sub (left: Expr, right: Expr) extends Expr
case class Val (x: String, i: Expr, b: Expr) extends Expr
case class Id (x: String) extends Expr
case class Call (x: String, a: Expr) extends Expr

case class FunDef(f: String, x: String, b: Expr) // not of Expr

type Env = Map[String, Int]
type FEnv = Map[String, FunDef]

def identity (e: Expr): Expr = e

def interp (e: Expr, env: Env, fenv: FEnv): Int = {
	e match {
		case Num(n) => n 
		case Add(l, r) => interp(l, env, fenv) + interp(r, env, fenv)
		case Sub(l, r) => interp(l, env, fenv) - interp(r, env, fenv)
		case Val(x, i, b) => interp(b, env + (x -> interp(i, env, fenv)), fenv)
		case Id(x) => env(x)
		case Call(f, a) => {
			val FunDef(_, x, e) = fenv(f)
			interp(e, Map(x -> interp(a, env, fenv)), fenv)
		}
	}
}

def main (args: Array[String]): Unit = {
	val f1 = FunDef("triple", "x", Add(Id("x"), Call("double", Id("x"))))

	val f2 = FunDef("double", "x", Add(Id("x"), Id("x")))

	val e3 = Sub(Add(Num(1), Call("atriple", Num(3))), Call("double", Num(1)))

	println("" + interp(e3, Map(), Map("triple" -> f1, "double" -> f2)))
}











