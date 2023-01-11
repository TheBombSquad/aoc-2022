// Day 5: Supply Stacks - https://adventofcode.com/2022/day/5
//
// Generator: Parses the input by splitting it into a 'stack' section and a 'procedure' section.
// The stack section (the first 8 lines) is parsed by splitting the drawing into lines, reversing
// them (since we're going to be pushing things onto a stack, we want the thing on the top to be
// pushed last), splitting it into chunks of four characters, taking the second character as our
// input, and deciding whether or not to push a character depending on if the character is a space
// or not. We have nine stacks, and therefore nine chunks of four characters to split from, so we
// use the index of the chunk to determine which stack to push to. Procedures are parsed by
// splitting the list of procedures into a vector of numbers which we will handle in our solver.
//
// Part 1: Execute our list of procedures on the active stacks. When we want to move N items from
// one stack to another, we pop N items off the source, and push N items onto the destination.
//
// Part 2: We execute the same list of procedures, but to preserve order, we use `drain` to store
// all the popped items in the same order, and `extend` to add them to the destination in order.
//
#[derive(Default, Clone)]
pub struct Stack {
    v: Vec<u8>,
}

#[derive(Default)]
pub struct Input {
    stacks: Vec<Stack>,
    procedures: Vec<Vec<usize>>,
}
#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Input {
    let mut parsed = Input::default();
    let mut all_stacks: [Stack; 9] = Default::default();

    let lines: Vec<&str> = input.lines().collect();

    // Parse stacks
    lines
        .iter()
        .take(8)
        .rev()
        .map(|x| {
            x.as_bytes()
                .chunks(4)
                .map(|x| x[1])
                .enumerate()
                .collect::<Vec<(usize, u8)>>()
        })
        .for_each(|x| {
            for enumerated_char in &x {
                if enumerated_char.1 != b' ' {
                    all_stacks[enumerated_char.0].v.push(enumerated_char.1);
                }
            }
        });

    parsed.stacks = all_stacks.to_vec();

    // Parse procedures
    parsed.procedures = lines
        .into_iter()
        .skip(10)
        .map(|x| {
            x.split(' ')
                .filter_map(|x| x.parse::<usize>().ok())
                .collect()
        })
        .collect();

    parsed
}

#[aoc(day5, part1)]
pub fn solver_part1(input: &Input) -> String {
    let procs = input.procedures.clone();
    let mut stacks = input.stacks.clone();
    for arg_list in &procs {
        let (amount, source, destination) = (arg_list[0], arg_list[1] - 1, arg_list[2] - 1);
        for _i in 0..amount {
            if let Some(e) = stacks[source].v.pop() {
                stacks[destination].v.push(e);
            }
        }
    }
    let result: String = stacks
        .iter()
        .map(|x| *x.v.last().unwrap() as char)
        .collect();
    result
}

#[aoc(day5, part2)]
pub fn solver_part2(input: &Input) -> String {
    let procs = input.procedures.clone();
    let mut stacks = input.stacks.clone();
    for arg_list in &procs {
        let (amount, source, destination) = (arg_list[0], arg_list[1] - 1, arg_list[2] - 1);
        let source_len = stacks[source].v.len();

        let popped: Vec<u8> = stacks[source]
            .v
            .drain(source_len - amount..source_len)
            .collect();

        stacks[destination].v.extend(popped);
    }
    let result: String = stacks
        .iter()
        .map(|x| *x.v.last().unwrap() as char)
        .collect();
    result
}

#[cfg(test)]
mod tests {
    use super::*;
}
