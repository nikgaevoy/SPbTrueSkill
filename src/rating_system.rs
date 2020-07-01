mod nodes;

use std::collections::HashMap;

use nodes::distributions::Gaussian;
use nodes::{ProdNode, LeqNode, GreaterNode, SumNode, TreeNode, ValueNode, FuncNode};
use std::cell::{RefCell};
use std::rc::{Rc, Weak};
use std::f64::INFINITY;

// performance sigma
const BETA: f64 = 200.;
// epsilon used for ties
const EPS: f64 = 0.736;
// default player rating
const MU: f64 = 1500.;
// default player sigma
const SIGMA: f64 = MU / 3.;
// epsilon used for convergence loop
const CONVERGENCE_EPS: f64 = 2e-4;
// defines sigma growth per second
const SIGMA_GROWTH: f64 = 1e5;

pub type PlayerRating = Gaussian;
type Message = nodes::Message;
pub type Player = String;
pub type Team = Vec<Player>;
pub type ContestPlace = Vec<Team>;
pub type Contest = Vec<ContestPlace>;
pub type Rating = HashMap<Player, PlayerRating>;
pub type RatingHistory = HashMap<Player, Vec<(PlayerRating, usize)>>;


impl Default for PlayerRating {
    fn default() -> PlayerRating {
        PlayerRating {
            mu: MU,
            sigma: SIGMA,
        }
    }
}

fn load_rating(old: &RatingHistory, new: &mut Rating, contest: &Contest, when: usize) {
    for place in &contest[..] {
        for team in &place[..] {
            for player in &team[..] {
                let curr = old.get(player).cloned().unwrap_or(vec![(PlayerRating::default(), when)]);
                let mut add = curr.last().unwrap().clone();
                assert!(add.1 <= when);
                add.0.sigma = f64::min(SIGMA, add.0.sigma + (when - add.1) as f64 * SIGMA_GROWTH);
                new.insert(player.to_string(), add.0);
            }
        }
    }
}


fn update_rating(old: &Rating, new: &mut RatingHistory, contest: &Contest, when: usize) {
    for place in &contest[..] {
        for team in &place[..] {
            for player in &team[..] {
                new.entry(player.clone()).or_insert(Vec::new()).push((old.get(player).unwrap().clone(), when));
            }
        }
    }
}


fn gen_team_message<T, K: Clone>(places: &Vec<Vec<T>>, default: &K) -> Vec<Vec<K>> {
    let mut ret: Vec<Vec<K>> = Vec::with_capacity(places.len());

    for place in places {
        ret.push(vec![default.clone(); place.len()]);
    }

    ret
}


fn gen_player_message<T, K: Clone>(places: &Vec<Vec<Vec<T>>>, default: &K) -> Vec<Vec<Vec<K>>> {
    let mut ret = Vec::with_capacity(places.len());

    for place in places {
        ret.push(Vec::with_capacity(place.len()));

        for team in place {
            ret.last_mut().unwrap().push(vec![default.clone(); team.len()]);
        }
    }

    ret
}


fn infer1(who: &mut Vec<impl TreeNode>) {
    for item in who {
        item.infer();
    }
}


fn infer2(who: &mut Vec<Vec<impl TreeNode>>) {
    for item in who {
        infer1(item);
    }
}


fn infer3(who: &mut Vec<Vec<Vec<impl TreeNode>>>) {
    for item in who {
        infer2(item);
    }
}


fn check_convergence(a: &Vec<Rc<RefCell<(Message, Message)>>>,
                     b: &Vec<(Message, Message)>) -> f64 {
    if a.len() != b.len() {
        return INFINITY;
    }

    let mut ret = 0.;

    for i in 0..a.len() {
        ret = f64::max(ret,
                       f64::max(
                           f64::max(f64::abs(RefCell::borrow(&a[i]).0.mu - b[i].0.mu),
                                    f64::abs(RefCell::borrow(&a[i]).0.sigma - b[i].0.sigma)),
                           f64::max(f64::abs(RefCell::borrow(&a[i]).1.mu - b[i].1.mu),
                                    f64::abs(RefCell::borrow(&a[i]).1.sigma - b[i].1.sigma)),
                       ));
    }

    ret
}


fn infer_ld(ld: &mut Vec<impl TreeNode>, l: &mut Vec<impl TreeNode>) {
    for i in 0..ld.len() {
        l[i].infer();
        ld[i].infer();
    }
    l.last_mut().unwrap().infer();
    for i in 0..ld.len() {
        let i = ld.len() - 1 - i;
        ld[i].infer();
        l[i].infer();
    }
}


fn inference(rating: &mut Rating, contest: &Contest) {
    if contest.is_empty() {
        return;
    }

    // could be optimized, written that way for simplicity
    let mut s = gen_player_message(contest, &ProdNode::new());
    let mut perf = gen_player_message(contest, &ProdNode::new());
    let mut p = gen_player_message(contest, &ProdNode::new());
    let mut t = gen_team_message(contest, &ProdNode::new());
    let mut u = gen_team_message(contest, &LeqNode::new(EPS));
    let mut l = vec![ProdNode::new(); contest.len()];
    let mut d = vec![GreaterNode::new(2. * EPS); contest.len() - 1];
    let mut sp = Vec::new();
    let mut pt = Vec::new();
    let mut tul = Vec::new();
    let mut ld = Vec::new();
    let mut players = Vec::new();
    let mut conv = Vec::new();
    let mut old_conv = Vec::new();

    for i in 0..contest.len() {
        for j in 0..contest[i].len() {
            for k in 0..contest[i][j].len() {
                players.push((contest[i][j][k].clone(), s[i][j][k].add_edge()));
                RefCell::borrow_mut(&players.last().unwrap().1.upgrade().unwrap()).0 =
                    rating.get(&players.last().unwrap().0).unwrap().clone();

                let mut tmp: Vec<&mut dyn ValueNode> = Vec::with_capacity(3);
                tmp.push(&mut p[i][j][k]);
                tmp.push(&mut s[i][j][k]);
                tmp.push(&mut perf[i][j][k]);
                sp.push(SumNode::new(&mut tmp));
                RefCell::borrow_mut(perf[i][j][k].last_mut().unwrap()).1 = Gaussian { mu: 0., sigma: BETA };
            }

            let mut tt: Vec<&mut dyn ValueNode> = vec![&mut t[i][j]];
            for pp in &mut p[i][j] {
                tt.push(pp);
            }
            pt.push(SumNode::new(&mut tt));
            let mut tmp: Vec<&mut dyn ValueNode> = Vec::with_capacity(3);
            tmp.push(&mut l[i]);
            tmp.push(&mut t[i][j]);
            tmp.push(&mut u[i][j]);
            tul.push(SumNode::new(&mut tmp));
            conv.push(t[i][j].last_mut().unwrap().clone());
        }

        if i != 0 {
            let mut tmp: Vec<&mut dyn ValueNode> = Vec::with_capacity(3);
            let (a, b) = l.split_at_mut(i);
            tmp.push(a.last_mut().unwrap());
            tmp.push(b.first_mut().unwrap());
            tmp.push(&mut d[i - 1]);
            ld.push(SumNode::new(&mut tmp));
        }
    }

    infer3(&mut s);
    infer1(&mut sp);
    infer3(&mut p);
    infer1(&mut pt);
    infer2(&mut t);
    infer1(&mut tul);
    infer2(&mut u);
    infer1(&mut tul);

    let mut rounds = 0;

    while check_convergence(&conv, &old_conv) >= CONVERGENCE_EPS {
        old_conv.clear();
        for item in &conv {
            old_conv.push(RefCell::borrow(item).clone());
        }
        rounds += 1;

        infer_ld(&mut ld, &mut l);
        infer1(&mut d);
        infer1(&mut ld);
        infer1(&mut l);
        infer1(&mut tul);
        infer2(&mut u);
        infer1(&mut tul);
    }

    eprintln!("Rounds until convergence: {}", rounds);

    infer2(&mut t);
    infer1(&mut pt);
    infer3(&mut p);
    infer1(&mut sp);
    infer3(&mut s);

    for (name, mess) in &players {
        let prior;
        let performance;

        prior = RefCell::borrow(&Weak::upgrade(mess).unwrap()).0.clone();
        performance = RefCell::borrow(&Weak::upgrade(mess).unwrap()).1.clone();

        *rating.get_mut(name).unwrap() = prior * performance;
    }
}


pub fn simulate_contest(rating_history: &mut RatingHistory, contest: &Contest, when: usize) {
    let mut contest_rating = Rating::new();
    load_rating(rating_history, &mut contest_rating, &contest, when);

    inference(&mut contest_rating, &contest);

    update_rating(&contest_rating, rating_history, &contest, when);
}
