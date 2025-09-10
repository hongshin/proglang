fn print_name_num (name : String, num: i32) {
	println!("{} {}", name ,num) ;
	//println!("{:p} {:p}", &name, &num) ;
}

fn main () {
	let s = String::from("Eric") ;
	let i : i32 = 111 ;

	print_name_num(s, i) ;
}

/* Copy trait: https://doc.rust-lang.org/std/marker/trait.Copy.html */
