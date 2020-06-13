mod read_codeforces;
mod rating_system;

use rating_system::{Contest, simulate_contest};
use read_codeforces::{get_contest, get_contest_ids};
use read_codeforces::Contest as EbTechContest;
use std::time;


fn contest_adaptor(from: &EbTechContest) -> Contest {
    let mut ans = Contest::new();

    for i in 1..from.standings.len() {
        assert!(from.standings[i - 1].1 <= from.standings[i].1);
    }

    let mut prev = usize::MAX;

    for (user, lo, _hi) in &from.standings {
        if *lo != prev {
            ans.push(Vec::new());
        }
        ans.last_mut().unwrap().push(vec![user.clone()]);

        prev = *lo;
    }

    ans
}


fn main() {
    let mut rating = rating_system::Rating::new();

    let now = time::Instant::now();

    for contest_id in get_contest_ids() {
        let contest: EbTechContest = get_contest("cache", contest_id);
        println!(
            "Processing {:5} contestants in contest/{:4}: {}",
            contest.standings.len(),
            contest.id,
            contest.name
        );
        simulate_contest(&mut rating, &contest_adaptor(&contest));
    }

    use std::io::Write;
    let filename = "data/CFratings.txt";
    let file = std::fs::File::create(filename).expect("Output file not found");
    let mut out = std::io::BufWriter::new(file);

    for (key, value) in &rating {
        writeln!(out, "{}:\t{:4.2}\t{:4.2}", key, value.mu, value.sigma).ok();
    }

    println!("Finished in {:.2} seconds", now.elapsed().as_secs_f64());
}
