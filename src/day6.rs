#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> String {
    input.to_owned()
}

#[aoc(day6, part1)]
pub fn solver_part1(input: &String) -> usize {
    input
            .as_bytes()
            .windows(4)
            .take_while(|x| {
                x[1..4].contains(&x[0]) ||
                x[2..4].contains(&x[1]) ||
                x[3] == x[2]
            })
            .count()+4
}

/*
#[aoc(day6, part2)]
pub fn solver_part2(input: &[]) -> () {

}
*/

#[cfg(test)]
mod tests {
    use super::*;
}
