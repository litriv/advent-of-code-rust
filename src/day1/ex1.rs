use crate::lines_for;

type Calories = u32;

pub fn solve() -> Calories {
    find_max(lines_for("input_data/1"))
}

fn find_max(vals: impl IntoIterator<Item = String>) -> Calories {
    let mut is_skipping = true;
    let mut max = Calories::MIN;
    let mut lmax = Calories::MIN;

    fn parse(s: &str) -> Calories {
        s.parse::<Calories>().unwrap()
    }

    for l in vals {
        match (is_skipping, l.is_empty()) {
            (false, false) => lmax += parse(&l),
            (false, true) => {
                max = lmax.max(max);
                is_skipping = true;
            }
            (true, false) => {
                lmax = parse(&l);
                is_skipping = false;
            }
            (true, true) => (),
        }
    }
    lmax.max(max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it() {
        assert_eq!(66306, solve());
    }
}
