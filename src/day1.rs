#[aoc_generator(day1)] 
pub fn input_generator(input: &str) -> Vec<u32> {
    let split: Vec<u32> = input.split("\n\n")
        .into_iter()
        .map(|x| x.split("\n")
             .into_iter()
             .map(|x| x.parse::<u32>().unwrap_or_default())
             .reduce(|a, b| a+b).unwrap())
        .collect();

    split
}

#[aoc(day1, part1)] 
pub fn solver_part1(input: &Vec<u32>) -> u32 {
    let mut vec = input.clone();
    vec.sort();
    *vec.last().unwrap()
}

#[aoc(day1, part2)] 
pub fn solver_part2(input: &Vec<u32>) -> u32 {
    let mut vec = input.clone();
    vec.sort();
    let end = vec.len(); 
    vec.split_off(end-3).into_iter().reduce(|a, b| a+b).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
}
