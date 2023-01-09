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
            x.iter().for_each(|x| {
                if x.1 != b' ' {
                    all_stacks[x.0].v.push(x.1);
                }
            })
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
    let mut procs = input.procedures.clone();
    let mut stacks = input.stacks.clone();
    procs.iter_mut().for_each(|x| {
        let (amount, source, destination) = (x[0], x[1] - 1, x[2] - 1);
        for _i in 0..amount {
            if let Some(e) = stacks[source].v.pop() {
                stacks[destination].v.push(e);
            }
        }
    });
    let result: String = stacks
        .iter()
        .map(|x| *x.v.last().unwrap() as char)
        .collect();
    result
}

#[aoc(day5, part2)]
pub fn solver_part2(input: &Input) -> String {
    let mut procs = input.procedures.clone();
    let mut stacks = input.stacks.clone();
    procs.iter_mut().for_each(|x| {
        let (amount, source, destination) = (x[0], x[1] - 1, x[2] - 1);
        let source_len = stacks[source].v.len();

        let popped: Vec<u8> = stacks[source]
            .v
            .drain(source_len - amount..source_len)
            .collect();

        popped.iter().for_each(|x| stacks[destination].v.push(*x));
    });
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
