pub mod ex1;
pub mod ex2;

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
}
