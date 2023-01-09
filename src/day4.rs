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
