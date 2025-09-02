fn main () {
    let s = 1 ;

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
