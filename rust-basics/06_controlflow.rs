/* Control expressions

   * Braching expression
   	- if-expression
	- if-else-expression

   * Loop expressions
	- while-expression
	- for-expression
	- loop-expression
	- break and continue with label
*/


fn main () 
{
	let a = 5 ;
	let b = 9 ;

	let gr = if b < a {
		a 
	}
	else {
		b
	} ;

	println!("gr: {}", gr) ;


	let mut x = 5; // mut x: i32
	let mut done = false; // mut done: bool

	while !done {
		x += x - 3 ;
		print!("{} ", x) ;
		if x % 5 == 0 {
			done = true;
		}
	}
	println!("") ;

	
	for x in 0..10 {
		print!( "{} ", x );
	}
	println!("") ;

	'outer: for x in 0..10 {
		'inner: for y in 0..10 {
			if x % 2 == 0 { 
				continue 'outer ; 
			} // Continues the loop over `x`.
			if y % 2 == 0 {
				continue 'inner ;
			} // Continues the loop over `y`.
			println!("x: {}, y: {}", x, y) ;
    		}
	}
}
