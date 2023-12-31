use std::collections::HashMap;

use crate::lines_for;

type Priority = u8;
type PrioritySum = u32;

fn solve() -> PrioritySum {
    priority_sum(lines_for("input_data/1"))
}

fn priority_sum(vals: impl IntoIterator<Item = String>) -> PrioritySum {
    vals.into_iter()
        .map(|l| <u8 as Into<u32>>::into(priority(l)))
        .sum()
}

fn priority(l: String) -> Priority {
    let bs = l.as_bytes();
    let len = bs.len();
    if len % 2 != 0 {
        panic!("expected length to be even: {}", l);
    }
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it() {
        assert_eq!(10, solve());
    }
}
