use std::collections::HashSet;

fn get_priority(character: u8) -> u8 {
    if character > 97 {
        return character - 96;
    } else {
        return character - 38;
    }
}

pub struct Rucksack {
    compartment_a: String,
    compartment_b: String,
}
impl Rucksack {
    fn get_shared(&self) -> u8 {
        let a_set: HashSet<u8> = self.compartment_a.as_bytes().iter().cloned().collect();
        let b_set: HashSet<u8> = self.compartment_b.as_bytes().iter().cloned().collect();
        return *a_set.intersection(&b_set).collect::<Vec<&u8>>()[0];
    }
}

fn get_group_shared(chunk: &[String]) -> u8 {
    let a_set: HashSet<u8> = chunk[0].as_bytes().iter().cloned().collect();
    let b_set: HashSet<u8> = chunk[1].as_bytes().iter().cloned().collect();
    let c_set: HashSet<u8> = chunk[2].as_bytes().iter().cloned().collect();
    let a_intersect_b = a_set.intersection(&b_set).cloned().collect::<HashSet<u8>>();
    *a_intersect_b.intersection(&c_set).collect::<Vec<&u8>>()[0]
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
        .map(|x| get_priority(x.get_shared()))
        .map(|x| u32::from(x))
        .sum()
}

#[aoc(day3, part2)]
pub fn solver_part2(input: &[Rucksack]) -> u32 {
    input
        .into_iter()
        .map(|x| x.compartment_a.clone() + &x.compartment_b)
        .collect::<Vec<String>>()
        .chunks(3)
        .map(|x| get_priority(get_group_shared(x)))
        .map(|x| u32::from(x))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
}
