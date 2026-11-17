/* Function 

    * Function type   	
	-"()" is the unit value of the unit type (similar to null)
	- (a list of input argument:type's) -> (Output type)
	- For simplicity, () means () -> () 

    * Use the return statement, or block expression

*/

fn two () -> i32 {
    2 
}

fn double (n: i32) -> i32 {
    n * 2 
}



fn main () {
    let r = two() ; 
    println!("r: {}, double(r): {}", r, double(r)) ;

    {
        let r = 3 ;
        println!("double(r): {}", double(r)) ;

        println!("multiply(5,4): {}", multiply(5, 4))
    }
}

fn multiply (fact: i32, n: i32) -> i32 {
    fact * n 
}
