use std::{
    fs::File,
    io::{self, prelude::*, BufReader},
};

type Calories = u32;

pub fn solve() -> io::Result<Calories> {
    let f = File::open("input_data/1")?;
    let r = BufReader::new(f);
    let mut is_skipping = true;
    let mut maxs = Vec::new();
    let mut lmax = Calories::MIN;

    fn parse(s: &str) -> Calories {
        s.parse::<Calories>().unwrap()
    }
    let mut l = String::new();
    for rl in r.lines() {
        l = rl.unwrap();
        match (is_skipping, l.is_empty()) {
            (false, false) => lmax += parse(&l),
            (false, true) => {
                maxs.push(lmax);
                is_skipping = true;
            }
            (true, false) => {
                lmax = parse(&l);
                is_skipping = false;
            }
            (true, true) => (),
        }
    }
    if !l.is_empty() {
        maxs.push(lmax);
    }
    maxs.sort_by_key(|w| std::cmp::Reverse(*w));
    Ok(maxs.into_iter().take(3).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it() {
        assert_eq!(195292, solve().unwrap());
    }
}
