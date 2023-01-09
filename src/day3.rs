fn get_priority(character: u8) -> u8 {
    if character > 97 {
        character - 96
    } else {
        character - 38
    }
}

pub struct Rucksack {
    side_a: String,
    side_b: String,
}
impl Rucksack {
    fn get_shared(&self) -> u8 {
        let a_collection = self.side_a.as_bytes();
        let intersection: Vec<&u8> = self
            .side_b
            .as_bytes()
            .iter()
            .filter(|x| a_collection.contains(x))
            .collect();
        *intersection[0]
    }
}

fn get_group_shared(chunk: &[String]) -> u8 {
    let a_collection = chunk[0].as_bytes();
    let intersection_a_b: Vec<&u8> = chunk[1]
        .as_bytes()
        .iter()
        .filter(|x| a_collection.contains(x))
        .collect();
    let intersection_a_b_c: Vec<&u8> = chunk[2]
        .as_bytes()
        .iter()
        .filter(|x| intersection_a_b.contains(x))
        .collect();
    *intersection_a_b_c[0]
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Rucksack> {
    let rucksack_split: Vec<Rucksack> = input
        .split_whitespace()
        .map(|x| x.split_at(x.len() / 2))
        .map(|x| Rucksack {
            side_a: x.0.to_owned(),
            side_b: x.1.to_owned(),
        })
        .collect();
    rucksack_split
}

#[aoc(day3, part1)]
pub fn solver_part1(input: &[Rucksack]) -> u32 {
    input
        .iter()
        .map(|x| get_priority(x.get_shared()))
        .map(u32::from)
        .sum()
}

#[aoc(day3, part2)]
pub fn solver_part2(input: &[Rucksack]) -> u32 {
    input
        .iter()
        .map(|x| x.side_a.clone() + &x.side_b)
        .collect::<Vec<String>>()
        .chunks(3)
        .map(|x| get_priority(get_group_shared(x)))
        .map(u32::from)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
}
