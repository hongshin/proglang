/* String
   
   * string constants are in the &str type
   
   * String::from() produces a String object from the &str value
   	- associated function

   * {}   : display printing (std::fmt::Display trait)
   * {:?} : debug printing (std::fmt::Debug trait)

   * String.len() returns the number of bytes storing the string
   * String.chars().count() returns the number of characters in 
     the character array from the string
*/

fn main () {
    let name = "서태지" ; // This is a Korean name. No problem, because a &str is UTF-8.
    let other_name = String::from("Adrian Fahrenheit Țepeș") ; // Ț and ș are no problem in UTF-8.

    println!("{} {}", name, other_name) ;
    println!("{:?} {:?}", &name, &other_name) ;
    dbg!(&name, &other_name) ;

    println!("{} {}", name.len(), other_name.len() ) ;
    println!("{} {}", name.chars().count(), other_name.chars().count()) ;

    let together = format!("/*{}--{}*/", name, other_name) ;
    println!("{}", together) ;
}
