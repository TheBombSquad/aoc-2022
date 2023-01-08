use std::collections::HashSet;

pub struct Rucksack {
    compartment_a: String,
    compartment_b: String,
}

impl Rucksack {
    fn get_priority(character: u8) -> u8 {
        if character > 97 {
            return character - 96
        }
        else {
            return character - 38
        }
    }
    fn get_shared(&self) -> u8 {
        let a_set: HashSet<u8> = self.compartment_a.as_bytes().iter().cloned().collect();
        let b_set: HashSet<u8> = self.compartment_b.as_bytes().iter().cloned().collect();
        return *a_set.intersection(&b_set).collect::<Vec<&u8>>()[0];
    }
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Rucksack> {
    let rucksack_split: Vec<Rucksack> = input
        .split_whitespace()
        .map(|x| x.split_at(x.len() / 2))
        .map(|x| Rucksack {
            compartment_a: x.0.to_owned(),
            compartment_b: x.1.to_owned(),
        })
        .collect();
    rucksack_split
}

#[aoc(day3, part1)]
pub fn solver_part1(input: &[Rucksack]) -> u32 {
    input
        .into_iter()
        .map(|x| Rucksack::get_priority(x.get_shared()))
        .map(|x| u32::from(x))
        .sum()
}

/*
#[aoc(day3, part2)]
pub fn solver_part2(input: &[]) -> () {

}*/

#[cfg(test)]
mod tests {
    use super::*;
}
