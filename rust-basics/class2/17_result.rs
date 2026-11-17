use std::io ;

fn main () 
{
	let mut input = String::new() ;

	io::stdin().read_line(&mut input).expect("") ;
	//io::stdin().read_line(&mut input).unwrap() ;
	/*
	let r : Result<usize, io::Error> = io::stdin().read_line(&mut input) ;
	match r {
		Ok(n) => println!("read {} bytes", n),
		Err(e) => println!("error: {}", e),
	}
	*/

	let r : i32 = input.trim().parse().unwrap() ;
	//let r : i32 = input.trim().parse().expect("") ;
	/*
	let r : Result<i32, std::num::ParseIntError> = input.trim().parse() ;
	match r {
		Ok(n) => { println!("{}", n) },
		Err(_) => { println!("Error") },
	}
	*/
	println!("{}", r) ;
}
