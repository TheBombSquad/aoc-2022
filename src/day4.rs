// Day 4: Camp Cleanup - https://adventofcode.com/2022/day/4
//
// Generator: Splits the input on commas (one range for each pair), then splits on dashes (for
// lower and higher end of ranges). Constructs an ElfPair struct for each line that consists of two
// ranges.
//
// Part 1: Check if the second range is entirely contained within the first range, or if the first
// range is entirely contained within the second range, using four comparisions.
//
// Part 2: Check if the higher end of the first range exceeds the lower end of the second range, or
// if the higher end of the second range exceeds the lower end of the first range, to determine if
// there is any overlap. In hindsight, I could have checked if the first range contained either of
// the endpoints of the second range, but that might lead to more comparisions. 
//
pub struct Range {
    lower_end: u32,
    higher_end: u32,
}

pub struct ElfPair(Range, Range);

impl ElfPair {
    fn fully_overlaps(&self) -> bool {
        let range_1 = &self.0;
        let range_2 = &self.1;

        range_1.lower_end <= range_2.lower_end && range_1.higher_end >= range_2.higher_end
            || range_2.lower_end <= range_1.lower_end && range_2.higher_end >= range_1.higher_end
    }

    fn any_overlap(&self) -> bool {
        let range_1 = &self.0;
        let range_2 = &self.1;
        !((range_1.higher_end < range_2.lower_end) || (range_2.higher_end < range_1.lower_end))
    }
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<ElfPair> {
    input
        .lines()
        .map(|x| x.split_once(',').unwrap())
        .map(|x| (x.0.split_once('-').unwrap(), x.1.split_once('-').unwrap()))
        .map(|x| {
            ElfPair(
                Range {
                    lower_end: x.0 .0.parse().unwrap(),
                    higher_end: x.0 .1.parse().unwrap(),
                },
                Range {
                    lower_end: x.1 .0.parse().unwrap(),
                    higher_end: x.1 .1.parse().unwrap(),
                },
            )
        })
        .collect()
}

#[aoc(day4, part1)]
pub fn solver_part1(input: &[ElfPair]) -> usize {
    input.iter().filter(|x| x.fully_overlaps()).count()
}

#[aoc(day4, part2)]
pub fn solver_part2(input: &[ElfPair]) -> usize {
    input.iter().filter(|x| x.any_overlap()).count()
}

#[cfg(test)]
mod tests {
    use super::*;
}
