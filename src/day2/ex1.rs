use crate::lines_for;

use super::{outcome::Outcome, shape::Shape, MyShape, Round, Score};

impl From<char> for MyShape {
    fn from(value: char) -> Self {
        match value {
            'X' => Self(Shape::Rock),
            'Y' => Self(Shape::Paper),
            'Z' => Self(Shape::Scissors),
            _ => panic!("invalid value: {}", value),
        }
    }
}

impl From<String> for Round {
    fn from(l: String) -> Self {
        Round::new(l.as_bytes()[0] as char, l.as_bytes()[2] as char)
    }
}
impl From<&Round> for Outcome {
    fn from(r: &Round) -> Self {
        if r.is_win() {
            Self::IWon
        } else if r.is_draw() {
            Self::Draw
        } else {
            Self::ILost
        }
    }
}

pub fn solve() -> Score {
    calc_score(lines_for("input_data/2"))
}

fn calc_score(vals: impl IntoIterator<Item = String>) -> Score {
    vals.into_iter()
        .map(|l| <String as Into<Round>>::into(l).score())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_win() {
        // opponent, me
        assert!(Round::new('A', 'Y').is_win());
        assert!(Round::new('B', 'Z').is_win());
        assert!(Round::new('C', 'X').is_win());
    }

    #[test]
    fn test_draw() {
        // opponent, me
        assert!(Round::new('A', 'X').is_draw());
        assert!(Round::new('B', 'Y').is_draw());
        assert!(Round::new('C', 'Z').is_draw());
    }

    #[test]
    fn test_lose() {
        // opponent, me
        assert_eq!(Round::new('A', 'Z').is_win(), false);
        assert_eq!(Round::new('B', 'X').is_win(), false);
        assert_eq!(Round::new('C', 'Y').is_win(), false);
    }

    #[test]
    fn test_it() {
        assert_eq!(13675, solve());
    }
}
