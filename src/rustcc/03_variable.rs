fn main () {
    let s : i32 ;

    let mut input = String::new() ;
    io::stdin().read_line(&mut input).expect("failed") ;
    s = input.trim().parse().expect("wrong") ;

    println!("s: {}", s) ;


    let s = 2 ;
    println!("s: {}", s) ;
    
    
    {
        let s = 3 ;
        println!("s: {}", s) ;
    }

    
    let r = {
        let v = 42 ;
        println!("v: {}", v) ;
        v + s
    } ;

    println!("r: {}", r) ;
    
}
