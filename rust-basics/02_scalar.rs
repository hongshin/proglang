/* 
 * Scalar types: https://doc.rust-lang.org/book/ch03-02-data-types.html

     * Integers: 
	  - signed: i8, i16, i32, i64, i128
	  - unsigned: u8, u16, u32, u64, u128
     * Floating-point real numbers: f32, f64
     * Boolean: bool
     * Character: char

*/

fn main () {
    let a : i16 = 100 ;
    let b : f64 = 3.14 ;
    println!("a: {}, b: {}", a, b) ;
    
    let c = b - 1.01 ;
    let d = a as f64 + b ;
    println!("c: {}, d: {}", c, d) ;
    
    let e = true ;
    let f : bool = false ;
    let g : char = '😊' ;
    println!("e: {}, f: {}, g: {}", e, f, g) ;
    
    let h = i8::MIN ;
    let i = i8::MAX ;
    println!("h: {}, i: {}", h, i) ;
}
