use std::io;

use crate::lines_for;

type Calories = u32;

pub fn solve() -> io::Result<Calories> {
    let mut is_skipping = true;
    let mut max = Calories::MIN;
    let mut lmax = Calories::MIN;

    fn parse(s: &str) -> Calories {
        s.parse::<Calories>().unwrap()
    }
    for l in lines_for("input_data/1") {
        let l = l.unwrap();
        match (is_skipping, l.is_empty()) {
            (false, false) => lmax += parse(&l),
            (false, true) => {
                max = lmax.max(max);
                is_skipping = true;
            }
            (true, false) => {
                lmax = parse(&l).max(Calories::MIN);
                is_skipping = false;
            }
            (true, true) => (),
        }
    }
    Ok(lmax.max(max))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it() {
        assert_eq!(66306, solve().unwrap());
    }
}