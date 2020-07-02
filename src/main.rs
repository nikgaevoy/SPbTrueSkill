use trueskill_spb::{Contest, simulate_contest};
use read_codeforces::{get_contest, get_contest_ids};
use read_codeforces::Contest as EbTechContest;
use std::time;


fn contest_adaptor(from: &EbTechContest) -> (Contest, usize) {
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

    (ans, from.time_seconds)
}


fn simulate_stored_contests(rating: &mut trueskill_spb::RatingHistory) {
    for contest_id in get_contest_ids() {
        let contest: EbTechContest = get_contest("cache", contest_id);
        println!(
            "Processing {:5} contestants in contest/{:4}: {}",
            contest.standings.len(),
            contest.id,
            contest.name
        );
        let adapted = contest_adaptor(&contest);
        simulate_contest(rating, &adapted.0, adapted.1);
    }
}


fn write_results(rating: &trueskill_spb::RatingHistory, filename: &str, history_size: usize) {
    use std::io::Write;
    let file = std::fs::File::create(filename).expect("Output file not found");
    let mut out = std::io::BufWriter::new(file);
    let mut to_sort = Vec::new();

    for (key, value) in rating {
        to_sort.push((key.clone(), value.clone()));
    }

    to_sort.sort_by(|(_ak, av), (_bk, bv)|
        av.last().unwrap().0.mu.partial_cmp(&bv.last().unwrap().0.mu).unwrap());
    to_sort.reverse();

    let mut ord = 1;
    for (key, value) in to_sort {
        write!(out, "{}.\t{:30}", ord, key).ok();
        ord += 1;
        for (rating, _when) in &value[value.len() - usize::min(history_size, value.len())..value.len()] {
            write!(out, "\t({:.2}, {:.2})", rating.mu, rating.sigma).ok();
        }
        writeln!(out).ok();
    }
}


fn main() {
    let mut rating = trueskill_spb::RatingHistory::new();

    let now = time::Instant::now();

    simulate_stored_contests(&mut rating);

    let rating = rating;
    let mut actual_rating = trueskill_spb::RatingHistory::new();

    for (key, value) in &rating {
        if value.last().unwrap().1 >= 1578148500 &&  // "Hello 2020"
            value.len() >= 10 {
            actual_rating.insert(key.clone(), value.clone());
        }
    }

    let actual_rating = actual_rating;

    write_results(&rating, "data/CFratings.txt", 1);
    write_results(&rating, "data/CFratings_10.txt", 10);
    write_results(&rating, "data/CFratings_full.txt", usize::MAX);
    write_results(&actual_rating, "data/CFratings_actual.txt", 1);
    write_results(&actual_rating, "data/CFratings_10_actual.txt", 10);
    write_results(&actual_rating, "data/CFratings_full_actual.txt", usize::MAX);

    let mut sum = 0.;

    for (_key, val) in &rating {
        sum += val.last().unwrap().0.mu;
    }

    println!("Avg. rating: {}", sum / (rating.len() as f64));

    println!("Finished in {:.2} seconds", now.elapsed().as_secs_f64());
}
