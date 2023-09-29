use crate::lines_for;

type Priority = u8;
type PrioritySum = i32;

fn solve() -> PrioritySum {
    priority_sum(lines_for("input_data/1"))
}

fn priority_sum(vals: impl IntoIterator<Item = String>) -> PrioritySum {
    1000
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it() {
        assert_eq!(10, solve());
    }
}
