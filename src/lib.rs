use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
};

pub mod day1;
pub mod day2;

fn lines_for(s: &str) -> Lines<BufReader<File>> {
    let f = File::open(s).unwrap();
    let r = BufReader::new(f);
    r.lines()
}
