fn main () {
	let a : i32 = 42 ;
	let b : Box<i32> = Box::new(a) ;
	println!("{a}") ;
	println!("{}", *b) ;
	/*
	*b = *b + 1 ;
	println!("{}", *b) ;
	*/

	/*
	let c : &mut Box<i32> = &mut b ;
	//println!("{b}") ;
	println!("{c}") ;

	**c = 63 ;
	println!("{c}") ;
	*/
}
