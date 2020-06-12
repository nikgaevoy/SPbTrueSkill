use crate::rating_system::{Contest, simulate_contest};

mod rating_system;


fn main() {
    let mut rating = rating_system::Rating::new();

    let contest: Contest = vec![vec![vec![String::from("first")]],
                                vec![vec![String::from("mid1")], vec![String::from("mid2")]],
                                vec![vec![String::from("last")]]];

    simulate_contest(&mut rating, &contest);

    for (key, value) in &rating {
        println!("{}:\t{}", key, value.mu);
    }
}
