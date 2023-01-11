// Day 1: Calorie Counting - https://adventofcode.com/2022/day/1
//
// Generator: Parses the input using iterators into a Vec<u32>
//
// Part 1: Sorts the Vec<u32> using sort_unstable (which is faster, preserving the original
// order does not matter for this situation), then returns the last (largest) element
//
// Part 2: Sorts the Vec<u32> using sort_unstable, then splits off the last three elements,
// using `reduce` on the resulting iterator to sum the elements together
//

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    let split: Vec<u32> = input
        .split("\n\n")
        .into_iter()
        .map(|x| {
            x.split('\n')
                .into_iter()
                .map(|x| x.parse::<u32>().unwrap_or_default())
                .reduce(|a, b| a + b)
                .unwrap()
        })
        .collect();

    split
}

#[aoc(day1, part1)]
pub fn solver_part1(input: &[u32]) -> u32 {
    let mut vec = input.to_owned();
    vec.sort_unstable();
    *vec.last().unwrap()
}

#[aoc(day1, part2)]
pub fn solver_part2(input: &[u32]) -> u32 {
    let mut vec = input.to_owned();
    vec.sort_unstable();
    let end = vec.len();
    vec.split_off(end - 3)
        .into_iter()
        .reduce(|a, b| a + b)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
}
