fn next_day (year: i32, month: i32, day: i32) -> Option<(i32, i32, i32)> {
    if year < 0 ||  month < 0 || 12 < month || day < 0 || 31 < day {
        return None ;
    }

    match month {
        12 => 
            match day {
                31 => Some((year + 1, 1, 1)),
                _ => Some((year, month, day + 1)) 
            },        
        1 | 3 | 5 | 7 | 8 | 10 =>
            match day {
                31 => Some((year, month + 1, 1)),
                _ => Some((year, month, day + 1))
            },
        2 => 
            match day {
                30 | 31 => None,
                29 => {
                    if year % 400 == 0 || year % 4 == 0 {
                        Some((year, month + 1, 1))
                    }
                    else {
                        None
                    }
                },
                _ => Some((year, month, day + 1))
            }
        _ => {
            match day {
                31 => None,
                30 => Some((year, month + 1, 1)),
                _ => Some((year, month, day + 1))
            }
        }
    }
}

fn main () {
    println!("{:?}", next_day(2024, 2, 28));
    println!("{:?}", next_day(2026, 2, 29));
    println!("{:?}", next_day(2026, 11, 24));
}