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

	/*
	let mut i = 0 ;
	loop {
		if i == 10 {
			break ;
		}
		i = i + 1 ;
		print!("{} ", i) 
	}
	*/

	let mut i = 0 ;
	let mut j ; 
	'l1: loop {
		j = 0 ;
		loop {
			if i == 4 && j == 2 {
				break 'l1 ;
			}
			if j == 100 {
				break ;
			}
			j = j + 1 ;
		}
		i = i + 1 ;
	}
	println!("i: {}, j: {}", i, j) ;
}
