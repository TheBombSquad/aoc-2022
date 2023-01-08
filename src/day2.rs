use std::str::FromStr;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, PartialEq)]
pub enum Outcome {
    Loss,
    Draw,
    Win,
}

impl FromStr for Outcome {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Loss),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err(()),
        }
    }
}

impl Shape {
    fn defeats(self) -> Self {
        match self {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        }
    }

    fn defeated_by(self, opponent: Shape) -> bool {
        opponent == self.defeats()
    }

    fn get_outcome(self, opponent: Shape) -> Outcome {
        if self == opponent {
            Outcome::Draw
        } else {
            match self.defeated_by(opponent) {
                true => Outcome::Win,
                false => Outcome::Loss,
            }
        }
    }

    fn value(self) -> u32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

impl FromStr for Shape {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" | "A" => Ok(Self::Rock),
            "Y" | "B" => Ok(Self::Paper),
            "Z" | "C" => Ok(Self::Scissors),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct Matchup(Shape, Shape);

impl Matchup {
    fn get_score(&self) -> u32 {
        let enemy_throw = self.0;
        let our_throw = self.1;

        let outcome = our_throw.get_outcome(enemy_throw);
        match outcome {
            Outcome::Loss => our_throw.value(),
            Outcome::Draw => our_throw.value() + 3,
            Outcome::Win => our_throw.value() + 6,
        }
    }
}

pub struct GamePlan(Shape, Outcome);

impl GamePlan {
    fn get_move(&self) -> Shape {
        let their_throw = self.0;
        match self.1 {
            Outcome::Loss => their_throw.defeats(),
            Outcome::Draw => their_throw,
            Outcome::Win => their_throw.defeats().defeats(),
        }
    }
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.split_whitespace().map(|x| x.to_owned()).collect()
}

#[aoc(day2, part1)]
pub fn solver_part1(input: &[String]) -> u32 {
    input
        .chunks(2)
        .map(|x| {
            Matchup(
                Shape::from_str(&x[0]).unwrap(),
                Shape::from_str(&x[1]).unwrap(),
            )
        })
        .map(|x| x.get_score())
        .sum()
}

#[aoc(day2, part2)]
pub fn solver_part2(input: &[String]) -> u32 {
    input
        .chunks(2)
        .map(|x| {
            GamePlan(
                Shape::from_str(&x[0]).unwrap(),
                Outcome::from_str(&x[1]).unwrap(),
            )
        })
        .map(|x| Matchup(x.0, x.get_move()).get_score())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn score_test() {
        let matchup = Matchup(Shape::Rock, Shape::Paper);
        assert_eq!(matchup.get_score(), 8);
        let matchup = Matchup(Shape::Paper, Shape::Rock);
        assert_eq!(matchup.get_score(), 1);
        let matchup = Matchup(Shape::Scissors, Shape::Scissors);
        assert_eq!(matchup.get_score(), 6);
    }
}
