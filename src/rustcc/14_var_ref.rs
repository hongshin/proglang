fn main () 
{
	let country = String::from("Austria") ;
	let one = country ;
	let two = country ;

	println!("{}", one) ;
	//println!("{}", ref_two) ;

	/*
	print_country(country) ;
	print_country(country) ;
	*/

	/*
	let ref_three = return_str() ;
	println!("{}", ref_three) ;
	*/

	/*
	let mut s = String::from("Title") ;
	add_tag(&mut s) ;
	println!("{}", s) ;
	*/
}

/*
fn print_country (country: String) {
	println!("{}", country) ;
}
*/

/*
fn return_str () -> &str {
	let country = String::from("German") ;
	let country_ref = &country ;
	country_ref 
}
*/

/*
fn add_tag (name: &mut String) {
	name.push_str("-tag") ;
}
*/
