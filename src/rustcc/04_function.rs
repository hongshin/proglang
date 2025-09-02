fn two () -> i32 {
    2 
}

fn double (n: i32) -> i32 {
    n * 2 
}



fn main () {
    let r = two() ; 

    println!("r: {}, double(3): {}", r, double(3)) ;

    /*
    {
        let r = 3 ;
        println!("r: {}, double(3): {}", r, double(3)) ;

        //println!("multiply(5): {}", multiply(r)) ;
    }
    */
}

/*
fn multiply (n: i32) -> i32 {
    fact * n 
}
*/

