fn main () {
    let v : i32 = 2025 ;
    let r = &v ;

    println!("v: {}, r: {}", v, r) ;

    /*
    println!("v: {}, r: {:p}", v, r) ;
    */

    
    let mut v : i32 = 2026 ;
    let mut w : i32 = 99 ;

    let mut p = &mut v ;
    *p = 2024 ;
    println!("v: {}", v) ;

    p = &mut w ;
    println!("p: {}", p) ;
    
}
