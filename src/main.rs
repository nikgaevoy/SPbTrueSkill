use crate::rating_system::{Contest, simulate_contest};
use std::time::Instant;

mod rating_system;


fn main() {
    let mut rating = rating_system::Rating::new();

    let contest: Contest = vec![vec![vec![String::from("first")]],
                                vec![vec![String::from("mid1")], vec![String::from("mid2")]],
                                vec![vec![String::from("last")]]];

    let now = Instant::now();

    simulate_contest(&mut rating, &contest);

    println!("{}", now.elapsed().as_secs_f64());

    for (key, value) in &rating {
        println!("{}:\t{}\t{}", key, value.mu, value.sigma);
    }
}
