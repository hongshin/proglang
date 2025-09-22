enum ThingsInTheSky {
    Sun,
    Stars,
}

fn create_skystate(time: i32) -> ThingsInTheSky {
    /*
    if 6 <= time && time <= 18 {
        ThingsInTheSky::Sun
    }
    else {
        ThingsInTheSky::Stars
    }
    */
    match time {
        6..=18 => ThingsInTheSky::Sun, 
        _ => ThingsInTheSky::Stars,
    }
    
}

fn check_skystate (state: &ThingsInTheSky) -> () {
    match state {
        ThingsInTheSky::Sun => println!("I can see the sun!"),
        ThingsInTheSky::Stars => println!("I can see the stars!")
    }
}


fn main() {
    let time = 8; 
    let skystate : ThingsInTheSky = create_skystate(time); 
    check_skystate(&skystate);
}
