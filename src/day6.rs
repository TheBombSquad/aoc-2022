// Day 6: Tuning Trouble - https://adventofcode.com/2022/day/6
//
// Generator: Nothing. We parse the input differently depending on the part.
//
// Part 1: We want to find sequences of four characters that are all different, so we use
// `take_while` on the input, split into bytes with rolling windows of size 4, and perform at most
// three checks on the input to check if the characters are all unique, stopping checks once we
// find a window with all unqiue characters. We return the amount of non-matching windows, plus
// four (since the puzzle wants us to look at the number of characters that need to be processed
// before the marker is found). 
//
// Part 2: This is solved in the exact same way, except I've generalized part 1 to accept window
// sizes of any length, performing at most `range-1` comparisions on each window.
//
#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> String {
    input.to_owned()
}

#[aoc(day6, part1)]
pub fn solver_part1(input: &String) -> usize {
    input
        .as_bytes()
        .windows(4)
        .take_while(|x| x[1..4].contains(&x[0]) || x[2..4].contains(&x[1]) || x[3] == x[2])
        .count()
        + 4
}

#[aoc(day6, part2)]
pub fn solver_part2(input: &String) -> usize {
    let range = 14;
    input
        .as_bytes()
        .windows(range)
        .take_while(|x| {
            for i in 0..range - 2 {
                if x[i + 1..range].contains(&x[i]) {
                    return true;
                }
            }
            x[range - 1] == x[range - 2]
        })
        .count()
        + range
}

#[cfg(test)]
mod tests {
    use super::*;
}
