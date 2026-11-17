enum Solution {
    Double(f64, f64),
    Single(f64),
    NoSolution,
}

fn quad (a : f64, b : f64, c : f64) -> Solution 
{
    if a == 0.0 {
        Solution::Single(-1.0 * c / b)
    }
    else if b*b - 4.0 * a * c < 0.0 {
        Solution::NoSolution 
    }
    else if b * b == 4.0 * a * c {
         Solution::Single(-1.0 * b / (2.0 * a)) 
    }
    else {
        Solution::Double((-1.0 * b + (b*b - 4.0 * a * c).sqrt()) / (2.0 * a), 
                         (-1.0 * b - (b*b - 4.0 * a * c).sqrt()) / (2.0 * a))
    }
}

fn main () 
{
    let s : Solution ;
    s = quad(1.0, 2.0, 1.0) ;

    match s {
        Solution::NoSolution => println!("Unsolvable"),
        Solution::Single(x) => println!("{}", x),
        Solution::Double(x, y) => println!("{}, {}", x, y)
    }
}
