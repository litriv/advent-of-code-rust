use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub mod day1;
pub mod day2;

fn lines_for(s: &str) -> impl IntoIterator<Item = String> {
    let f = File::open(s).unwrap();
    let r = BufReader::new(f);
    r.lines().map(Result::unwrap)
}
