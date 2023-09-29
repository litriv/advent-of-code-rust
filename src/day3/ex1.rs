type Priority = u8;
type PrioritySum = i32;

fn solve() -> PrioritySum {
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
