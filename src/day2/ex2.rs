use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use self::{outcome::Outcome, shape::Shape};

type Score = u16;

mod shape {
    type ShapeWeight = u8;

    #[derive(PartialEq, Eq)]
    pub enum Shape {
        Rock,
        Paper,
        Scissors,
    }
    impl Shape {
        fn weight(&self) -> ShapeWeight {
            match self {
                Self::Rock => 1,
                Self::Paper => 2,
                Self::Scissors => 3,
            }
        }
        pub fn does_beat(&self, other: &Shape) -> bool {
            match (self, other) {
                (Self::Rock, Self::Scissors)
                | (Self::Paper, Self::Rock)
                | (Self::Scissors, Self::Paper) => true,
                _ => false,
            }
        }
        pub fn score(&self) -> super::Score {
            self.weight() as u16
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_does_beat() {
            assert_eq!(Shape::Rock.does_beat(&Shape::Rock), false);
            assert_eq!(Shape::Rock.does_beat(&Shape::Paper), false);
            assert_eq!(Shape::Rock.does_beat(&Shape::Scissors), true);
            assert_eq!(Shape::Paper.does_beat(&Shape::Rock), true);
            assert_eq!(Shape::Paper.does_beat(&Shape::Paper), false);
            assert_eq!(Shape::Paper.does_beat(&Shape::Scissors), false);
            assert_eq!(Shape::Scissors.does_beat(&Shape::Rock), false);
            assert_eq!(Shape::Scissors.does_beat(&Shape::Paper), true);
            assert_eq!(Shape::Scissors.does_beat(&Shape::Scissors), false);
        }
    }
}

mod outcome {
    use super::Round;

    pub enum Outcome {
        IWon,
        Draw,
        ILost,
    }
    impl Outcome {
        pub fn score(&self) -> super::Score {
            match self {
                Self::IWon => 6,
                Self::Draw => 3,
                Self::ILost => 0,
            }
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
}

pub struct Round {
    opponent: Shape,
    me: Shape,
}
impl Round {
    fn new(opponent: char, me: char) -> Self {
        Round {
            opponent: Shape::from(opponent),
            me: Shape::from(me),
        }
    }
    fn is_win(&self) -> bool {
        self.me.does_beat(&self.opponent)
    }
    fn is_draw(&self) -> bool {
        self.me == self.opponent
    }
    fn score(&self) -> Score {
        Outcome::from(self).score() + self.me.score()
    }
}
// impl From<(Shape, Shape)> for Round {
//     fn from(shapes: (Shape, Shape)) -> Self {
//         Round {
//             opponent: shapes.0,
//             me: shapes.1,
//         }
//     }
// }

impl From<char> for Shape {
    fn from(value: char) -> Self {
        match value {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissors,
            _ => panic!("bad input: {}", value),
        }
    }
}

pub fn solve() -> Score {
    let f = File::open("input_data/2").unwrap();
    let r = BufReader::new(f);
    let mut score: Score = 0;

    for l in r.lines() {
        let l = l.unwrap();
        let r = Round::new(l.as_bytes()[0] as char, l.as_bytes()[2] as char);
        score += r.score();
    }
    score
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
        assert!(Round::new('A', 'A').is_draw());
        assert!(Round::new('B', 'B').is_draw());
        assert!(Round::new('C', 'C').is_draw());
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
