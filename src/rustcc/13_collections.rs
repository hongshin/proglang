fn main () {
	let arr1 = [1, 2, 3, 4, 5] ;
	let slice1 = &arr1[2..4] ;

	println!("slice: {:?}", slice1) ;

	/*
	let mut vec1 : Vec<&str> = Vec::new() ;
	let mut vec2 = vec![11, 12, 13] ;

	vec1.push("A") ;
	vec1.push("B") ;
	vec1.push("C") ;

	vec2.push(14) ;
	vec2.push(1500) ;

	for e in vec1 {
		print!("{} ", e) ;
	}
	println!("") ;
	*/
}
