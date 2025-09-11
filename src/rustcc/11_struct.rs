fn double (p: (i32, i32)) -> (i32, i32)
{
	(p.0 * 2, p.1 * 3)
}

fn main () {
	let p1 = (10, 20) ;
	let p2 = double(p1) ;

	println!("x: {}, y: {}", p1.0, p1.1) ;
}


/*
struct Point2D(i32, i32) ;
*/

/*
struct Point2D {
	x: i32,
	y: i32,
} 
*/
