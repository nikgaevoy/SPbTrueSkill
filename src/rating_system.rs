use std::collections::HashMap;

mod distributions;

use distributions::Gaussian;

// performance sigma
const BETA: f64 = 100.;
// epsilon used for ties
const EPS: f64 = 0.736;
// default player rating
const MU: f64 = 1500.;
// default player sigma
const SIGMA: f64 = MU / 3.;
// epsilon used for convergence loop
const CONVERGENCE_EPS: f64 = 1e-5;

pub type PlayerRating = Gaussian;
type Message = Gaussian;
pub type Player = String;
pub type Team = Vec<Player>;
pub type ContestPlace = Vec<Team>;
pub type Contest = Vec<ContestPlace>;
pub type Rating = HashMap<Player, PlayerRating>;

impl Default for PlayerRating {
    fn default() -> PlayerRating {
        PlayerRating {
            mu: MU,
            sigma: SIGMA,
        }
    }
}

fn update_rating(old: &Rating, new: &mut Rating, contest: &Contest) {
    for place in &contest[..] {
        for team in &place[..] {
            for player in &team[..] {
                new.insert(player.to_string(), old.get(player).cloned().unwrap_or(PlayerRating::default()));
            }
        }
    }
}


fn gen_team_message<T>(places: &Vec<Vec<T>>, default: &Message) -> Vec<Vec<Message>> {
    let mut ret = Vec::with_capacity(places.len());

    for place in places {
        ret.push(vec![default.clone(); place.len()]);
    }

    ret
}


fn gen_player_message<T>(places: &Vec<Vec<Vec<T>>>, default: &Message) -> Vec<Vec<Vec<Message>>> {
    let mut ret = Vec::with_capacity(places.len());

    for place in places {
        ret.push(Vec::with_capacity(place.len()));

        for team in place {
            ret.last_mut().unwrap().push(vec![default.clone(); team.len()]);
        }
    }

    ret
}


fn get_team_performance(player_performances: &Vec<Message>) -> Message {
    let mut ret = Message { mu: 0., sigma: 0. };

    for mess in player_performances {
        ret += mess;
    }

    ret
}


fn propagate_results(rating: &mut Rating, contest: &Contest, m_in_t_prime: &Vec<Vec<Message>>,
                     m_in_s: &Vec<Vec<Vec<Message>>>, m_in_p: &Vec<Vec<Vec<Message>>>) {
    for k in 0..contest.len() {
        for j in 0..contest[k].len() {
            for i in 0..contest[k][j].len() {
                let mut m_in_p_prime = m_in_t_prime[k][j].clone();
                for i_prime in 0..contest[k][j].len() {
                    if i_prime != i {
                        m_in_p_prime -= &m_in_p[k][j][i_prime];
                    }
                }
                let m_in_s_prime = m_in_p_prime + Message { mu: 0., sigma: BETA };
                *rating.get_mut(&contest[k][j][i]).unwrap() = &m_in_s_prime * &m_in_s[k][j][i];
            }
        }
    }
}


fn check_convergence(a: &Vec<Vec<Message>>, b: &Vec<Vec<Message>>) -> f64 {
    let mut diff = 0.;

    if a.len() != b.len() {
        return f64::MAX;
    }

    for i in 0..a.len() {
        assert_eq!(a[i].len(), b[i].len());

        for j in 0..a[i].len() {
            let dmu = (a[i][j].mu - b[i][j].mu).abs();
            let dsigma = (a[i][j].sigma - b[i][j].sigma).abs();

            diff = f64::max(diff, f64::max(dmu, dsigma));
        }
    }

    diff
}


fn inference(rating: &mut Rating, contest: &Contest) {
    assert!(!contest.is_empty());

    let default_message = Message { mu: 0., sigma: SIGMA };

    // could be optimized, written that way for simplicity
    let mut m_in_s = gen_player_message(contest, &default_message);
    let mut m_in_p = gen_player_message(contest, &default_message);
    let mut m_in_t = gen_team_message(contest, &default_message);
    let mut m_in_u = gen_team_message(contest, &default_message);
    let mut m_out_u = gen_team_message(contest, &default_message);
    let mut m_out_t = gen_team_message(contest, &default_message);
    let mut m_in_l = vec![default_message.clone(); contest.len()];
    let mut m_out_l = vec![default_message.clone(); contest.len()];
    let mut m_l2d_l = vec![default_message.clone(); contest.len() - 1];
    let mut m_l2d_r = vec![default_message.clone(); contest.len() - 1];
    let mut m_in_d = vec![default_message.clone(); contest.len() - 1];
    let mut m_out_d = vec![default_message.clone(); contest.len() - 1];
    let mut m_d2l_l = vec![default_message.clone(); contest.len() - 1];
    let mut m_d2l_r = vec![default_message.clone(); contest.len() - 1];
    let mut m_l2u = gen_team_message(contest, &default_message);
    let mut m_u2l = gen_team_message(contest, &default_message);


    // initialization
    for k in 0..contest.len() {
        for j in 0..contest[k].len() {
            for i in 0..contest[k][j].len() {
                m_in_s[k][j][i] = rating.get(&contest[k][j][i]).unwrap().clone();
                m_in_p[k][j][i] = &m_in_s[k][j][i] + Gaussian { mu: 0., sigma: BETA };
            }

            m_in_t[k][j] = get_team_performance(&m_in_p[k][j]);
            m_in_u[k][j] = Message { mu: 0., sigma: m_in_t[k][j].sigma };
            m_out_u[k][j] = m_in_u[k][j].leq_eps(EPS);
            m_out_t[k][j] = &m_out_u[k][j] + &m_in_t[k][j]; // seems to be a bug in the article
        }

        assert!(!m_out_t[k].is_empty());
        assert!(!m_out_u[k].is_empty());

        m_in_l[k] = &m_out_t[k][0] - &m_out_u[k][0];

        for j in 1..contest[k].len() {
            m_in_l[k] *= &m_out_t[k][j] - &m_out_u[k][j];
        }
    }

    // approximate inference
    for k in 0..m_l2d_l.len() {
        m_l2d_l[k] = m_in_l[k].clone();
        m_l2d_r[k] = m_in_l[k + 1].clone();
    }

    let mut tmp_m_l2u = Vec::new();
    let mut tmp_m_out_u = Vec::new();
    let mut rounds = 0;

    while f64::max(check_convergence(&tmp_m_l2u, &m_l2u),
                   check_convergence(&tmp_m_out_u, &m_out_u)) >= CONVERGENCE_EPS {
        tmp_m_l2u = m_l2u.clone();
        tmp_m_out_u = m_out_u.clone();
        rounds += 1;

        for k in 0..m_l2d_l.len() {
            m_in_d[k] = &m_in_l[k] - &m_in_l[k + 1];
            m_out_d[k] = m_in_d[k].greater_eps(2. * EPS);
            m_d2l_l[k] = &m_out_d[k] + &m_l2d_r[k];
            m_d2l_r[k] = &m_l2d_l[k] - &m_out_d[k];
        }
        for k in 0..m_l2u.len() {
            for j in 0..m_l2u[k].len() {
                if k == 0 {
                    m_l2u[k][j] = m_d2l_l[k].clone();
                } else if k == m_l2u.len() - 1 {
                    m_l2u[k][j] = m_d2l_r[k - 1].clone();
                } else {
                    m_l2u[k][j] = &m_d2l_l[k] * &m_d2l_r[k - 1];
                }

                for i in 0..m_l2u[k].len() {
                    if i != j {
                        m_l2u[k][j] *= &m_u2l[k][i];
                    }
                }

                m_in_u[k][j] = &m_in_l[k] - &m_out_t[k][j];
                m_out_u[k][j] = m_in_u[k][j].leq_eps(EPS);
                m_u2l[k][j] = &m_out_t[k][j] - &m_out_u[k][j];
            }

            m_out_l[k] = m_u2l[k][0].clone();

            for j in 1..m_u2l[k].len() {
                m_out_l[k] *= &m_u2l[k][j];
            }
        }

        m_l2d_l[0] = m_out_l[0].clone();
        for k in 1..m_l2d_l.len() {
            m_l2d_r[k - 1] = &m_out_l[k] * &m_d2l_l[k];
            m_l2d_l[k] = &m_out_l[k] * &m_d2l_r[k - 1];
        }
        *m_l2d_r.last_mut().unwrap() = m_out_l.last().unwrap().clone();
    }

    eprintln!("rounds: {}", rounds);

    for k in 0..m_in_t.len() {
        for j in 0..m_in_t[k].len() {
            m_in_t[k][j] = &m_l2u[k][j] - &m_out_u[k][j];
        }
    }

    // propagating the results
    propagate_results(rating, contest, &m_in_t, &m_in_s, &m_in_p);
}


pub fn simulate_contest(rating: &mut Rating, contest: &Contest) {
    let mut contest_rating = Rating::new();
    update_rating(rating, &mut contest_rating, contest);

    if contest.len() >= 2 {
        inference(&mut contest_rating, contest);
    }

    update_rating(&contest_rating, rating, contest);
}
