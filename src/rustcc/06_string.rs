fn main () {
    let name = "서태지" ; // This is a Korean name. No problem, because a &str is UTF-8.
    let other_name = String::from("Adrian Fahrenheit Țepeș") ; // Ț and ș are no problem in UTF-8.

    println!("{:?} {:?}", name, other_name) ;

    let together = format!("1: {}, 2:{}", name, other_name) ;
    println!("{}", together) ;

    /*
    println!("{:?} {:?}", name, other_name) ;
    //dbg!(name, other_name) ;
    */
    /*
    println!("{} {} {}", name, std::mem::size_of_val(name), std::mem::size_of_val("서  태  지")) ;
    */
    /*
    println!("{} {}", std::mem::size_of_val(other_name), std::mem::size_of_val("Adrian Fahrenheit Țepeș")) ;
    */
    /*
    println!("{} {}", other_name.len(), std::mem::size_of_val("Adrian Fahrenheit Țepeș")) ;
    */

}
