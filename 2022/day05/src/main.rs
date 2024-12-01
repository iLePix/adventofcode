use std::{io::{BufReader, BufRead}, fs::File, slice::Iter};

fn main() -> std::io::Result<()> {
    let path = "./input.txt";
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let binding = reader.lines().filter_map(|lr| lr.ok()).collect::<Vec<String>>();
    let mut lines = binding.iter();
    let stacks = parse_stacks(&mut lines);
    let instructions = parse_instructions(&mut lines);
    println!("Part 1: {}", part1(stacks.clone(), instructions.clone()));
    println!("Part 2: {}", part2(stacks, instructions));
    Ok(())
}

fn part1(mut stacks: Vec<Vec<char>>, instructions: Vec<Instruction>) -> String {
    for inst in instructions {
        for _ in 0..inst.amount {
            let elem = stacks[inst.from].pop().unwrap();
            stacks[inst.to].push(elem);
        }
    }
    stacks.iter().filter_map(|stack| stack.last()).collect::<String>()
}

fn part2(mut stacks: Vec<Vec<char>>, instructions: Vec<Instruction>) -> String {
    for inst in instructions {
            let elems = &stacks.clone()[inst.from][(stacks[inst.from].len() - inst.amount)..stacks[inst.from].len()];
            stacks[inst.to].append(&mut elems.to_vec());
            for _ in 0..inst.amount {
                stacks[inst.from].pop();
            }
    }
    stacks.iter().filter_map(|stack| stack.last()).collect::<String>()
}

#[derive(Clone, Copy, Debug)]
struct Instruction {
    amount: usize,
    from: usize,
    to: usize
}


fn parse_instructions(lines: &mut Iter<String>) -> Vec<Instruction> {
    lines.filter(|l| !l.is_empty()).map(|inst| {
        let elems = inst.split(' ')
        .enumerate()
        .filter(|(i, _)| i % 2 == 1)
        .map(|(_, i)| i.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
        .chunks(3)
        .map(|inst| Instruction {amount: inst[0], from: inst[1] - 1, to: inst[2] - 1})
        .collect::<Vec<Instruction>>();
        *elems.first().unwrap()
    }).collect()
}


fn parse_stacks(lines: &mut Iter<String>) -> Vec<Vec<char>> {
    let mut stacks: Vec<Vec<char>> = vec![];
    for (level, line) in lines.enumerate() {
        if level == 0 {
            stacks = vec![vec![]; (line.len() / 4) + 1]
        }
        let chars = line.chars().collect::<Vec<char>>();
        if chars.contains(&'1') {
            break;
        }
        for (stack, cr8) in chars.chunks(4).enumerate() {
            if cr8.len() > 2 {
                let c = cr8[1];
                if c != ' ' {
                    stacks[stack].push(c);
                } else {
                }
            } else {
                break;
            }
        }
    }
    for stack in &mut stacks {
        stack.reverse();
    }
    stacks
}
