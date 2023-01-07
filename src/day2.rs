use std::str::FromStr;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Debug)]
pub struct Matchup(Shape, Shape);

impl Matchup {
    // Assume we are self.0, and the opponent is self.1
    fn get_score(&self) -> u32 {
        let enemy_throw = self.1 as u32;
        let our_throw = self.0 as u32;

        // Draw
        if enemy_throw == our_throw {
            enemy_throw + 3
        }
        // Victory
        else if (our_throw != 3 && enemy_throw > our_throw)
            || (enemy_throw == 1 && our_throw == 3)
        {
            enemy_throw + 6
        }
        // Loss
        else {
            enemy_throw
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

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Matchup> {
    let split: Vec<_> = input.split_whitespace().collect();
    let matchups_new: Vec<Matchup> = split
        .chunks(2)
        .map(|x| {
            Matchup(
                Shape::from_str(x[0]).unwrap(),
                Shape::from_str(x[1]).unwrap(),
            )
        })
        .collect();

    matchups_new
}

#[aoc(day2, part1)]
pub fn solver_part1(input: &[Matchup]) -> u32 {
    input.iter().map(|x| x.get_score()).sum()
}

/*
#[aoc(day2, part2)]
pub fn solver_part2(input: &[Matchup]) -> u32 {
}*/

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
