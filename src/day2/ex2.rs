use crate::{day2::Score, lines_for};

use super::{outcome::Outcome, shape::Shape, MyShape, OpponentShape, Round};

struct DesiredOutcome(Outcome);
impl From<char> for DesiredOutcome {
    fn from(value: char) -> Self {
        match value {
            'X' => DesiredOutcome(Outcome::ILost),
            'Y' => DesiredOutcome(Outcome::Draw),
            'Z' => DesiredOutcome(Outcome::IWon),
            _ => panic!("invalid value: {}", value),
        }
    }
}

pub struct Round2 {
    round: Round,
}
impl Round2 {
    fn new<T, U>(opponent: T, desired_outcome: U) -> Self
    where
        T: Into<OpponentShape>,
        U: Into<DesiredOutcome>,
    {
        let opponent_shape = opponent.into();
        Self {
            round: Round {
                opponent: opponent_shape,
                me: MyShape(match desired_outcome.into().0 {
                    Outcome::Draw => opponent_shape.0,
                    Outcome::IWon => match opponent_shape.0 {
                        Shape::Rock => Shape::Paper,
                        Shape::Paper => Shape::Scissors,
                        Shape::Scissors => Shape::Rock,
                    },
                    Outcome::ILost => match opponent_shape.0 {
                        Shape::Rock => Shape::Scissors,
                        Shape::Paper => Shape::Rock,
                        Shape::Scissors => Shape::Paper,
                    },
                }),
            },
        }
    }
}
impl From<String> for Round2 {
    fn from(l: String) -> Self {
        Round2::new(l.as_bytes()[0] as char, l.as_bytes()[2] as char)
    }
}

pub fn solve() -> Score {
    calc_score(lines_for("input_data/2"))
}

fn calc_score(vals: impl IntoIterator<Item = String>) -> Score {
    vals.into_iter()
        .map(|l| <String as Into<Round2>>::into(l).round.score())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it() {
        assert_eq!(14184, solve());
    }
}
