use std::{io::{BufReader, BufRead}, fs::File, slice::Iter};

fn main() -> std::io::Result<()> {
    let path = "./input.txt";
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let binding = reader.lines().filter_map(|lr| lr.ok()).collect::<Vec<String>>();
    let mut lines = binding.iter();
    let mut stacks = parse_stacks(&mut lines);
    /*for move_instruction in lines {
        move_instruction.split(' ')
        .enumerate()
        .filter(|(i, _)| i % 2 == 1)
        .map(|(_, i)| i.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
        .chunks(3)
        .for_each(|inst| {
            let (amount, from, to) = (inst[0], inst[1] - 1, inst[2] - 1);
            for _ in 0..amount {
                let elem = stacks[from].pop().unwrap();
                stacks[to].push(elem);
            }
        })
    }*/
    for move_instruction in lines {
        move_instruction.split(' ')
        .enumerate()
        .filter(|(i, _)| i % 2 == 1)
        .map(|(_, i)| i.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
        .chunks(3)
        .for_each(|inst| {
            let (amount, from, to) = (inst[0], inst[1] - 1, inst[2] - 1);
            let elems = &stacks.clone()[from][(stacks[from].len() - amount)..stacks[from].len()];
            stacks[to].append(&mut elems.to_vec());
            for _ in 0..amount {
                stacks[from].pop();
            }
        })
    }



    let first_elems = stacks.iter().filter_map(|stack| stack.last()).collect::<String>();
    println!("{}", first_elems);
    Ok(())
}


fn parse_stacks(iter: &mut Iter<String>) -> Vec<Vec<char>> {
    let mut stacks: Vec<Vec<char>> = vec![];
    for (level, line) in iter.enumerate() {
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
    iter.skip(1);
    for stack in &mut stacks {
        stack.reverse();
    }
    stacks
}