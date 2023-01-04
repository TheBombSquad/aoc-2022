use std::{str::FromStr, error::Error, string::ParseError};

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3
}

#[derive(Debug)]
pub struct Matchup(Shape, Shape); 

impl Matchup {
    // Assume we are self.0, and the opponent is self.1
    fn get_score(&self) -> u32 {
        let value_1 = self.1 as u32;
        let value_2 = self.0 as u32;

        // Draw
        if value_1 == value_2 {
            return value_1 + 3
        }
        // Victory
        else if (value_1 == 1 && value_2 == 3) ||
                (value_1 == 2 && value_2 == 1) ||
                (value_1 == 3 && value_2 == 2) {
            return value_1 + 6
        }
        // Loss
        else {
            return value_1
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
    let matchups_new: 
        Vec<Matchup> = split
        .chunks(2)
        .map(|x| Matchup(
                Shape::from_str(x[0]).unwrap(), 
                Shape::from_str(x[1]).unwrap()
                ))
        .collect();

    matchups_new
}

#[aoc(day2, part1)] 
pub fn solver_part1(input: &Vec<Matchup>) -> u32 {
    let input_mut = input.clone();
    input_mut.iter().map(|x| x.get_score()).sum()
}


/*
#[aoc(day2, part2)] 
pub fn solver_part2(input: &Vec<u32>) -> u32 {
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
