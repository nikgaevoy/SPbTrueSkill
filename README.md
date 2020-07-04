# TrueSkill from St.Petersburg

![Rust](https://github.com/nikgaevoy/SPbTrueSkill/workflows/Rust/badge.svg?event=push)

The rating system for team competitions, just like TrueSkill™, but better.

[The ~~dark~~ math side.](https://logic.pdmi.ras.ru/~sergey/papers/NS11_Ratings.pdf)

Tests on [CodeForces](https://codeforces.com) history are [here](data/CFratings_actual.txt).

## Usage

To process all CodeForces with id's from [this](data/contest_ids.json) file.

    cargo run --release
  
Also cound be used as Rust library.
