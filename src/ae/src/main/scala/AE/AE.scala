package AE

sealed trait AE

case class Num (value: Int) extends AE
case class Add (left: AE, right: AE) extends AE
case class Sub (left: AE, right: AE) extends AE

def identity (ae: AE): AE = ae

def eval (e: AE): Int = {
	e match {
		case Num(n) => n 
		case Add(l, r) => eval(l) + eval(r)
		case Sub(l, r) => eval(l) - eval(r)
	}
}
