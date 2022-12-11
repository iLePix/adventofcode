use std::{fs::File, io::{BufReader, BufRead}, slice::Iter, collections::HashSet};

fn main() -> std::io::Result<()> {
    let path = "./input.txt";
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines().filter_map(|lr| lr.ok()).collect::<Vec<String>>();
    let mut monkies = parse_monkies(lines);

    println!("Part 1: {}", monkey_business(&mut monkies.clone(), true, 20));
    println!("Part 2: {}", monkey_business(&mut monkies, false, 10_000));
    Ok(())
}

fn monkey_business(monkies: &mut Vec<Monkey>, reduce_worry: bool, rounds: usize) -> u64 {
    let mut modulos = 1;
    for m in 0..monkies.len()  {
        modulos *= monkies[m].test;
    }
    for r in 0..rounds {
        for m in 0..monkies.len() {
            let monkey = monkies[m].clone();
            for item in monkey.items.iter() {
                let op_num = monkey.op.last().map(|o| if *o == "old" {*item} else {o.parse::<u64>().unwrap()}).unwrap();
                let div = if reduce_worry {3} else {1};
                let res = ((match monkey.op[1].as_ref() {
                    "*" => item * op_num,
                    "+" => item + op_num,
                    o => panic!("Unknown op: {}", o)
                }) / div ) % modulos as u64; 
                if res % monkey.test as u64 == 0 {
                    monkies[monkey.dsts[0] as usize].items.push(res);
                } else {
                    monkies[monkey.dsts[1] as usize].items.push(res);
                }
                monkies[m].inspections += 1;
            }
            monkies[m].items.clear();
        }
    }
    let mut inspections = monkies.iter().map(|m| m.inspections).collect::<Vec<u64>>();
    inspections.sort_by(|a, b| b.partial_cmp(a).unwrap());
    inspections[0] * inspections[1]
}


fn parse_monkies(lines: Vec<String>) -> Vec<Monkey> {
    let mut monkies: Vec<Monkey> = vec![];
    let mut i = 0;
    while i < lines.len() {
        let starting_items: Vec<u64> = lines[i + 1].trim()
            .split(' ')
            .collect::<Vec<&str>>()[2..]
            .iter().map(|s| {
                s.replace(',', "").parse::<u64>().unwrap()
            }).collect::<Vec<u64>>();
        let op: Vec<String> = lines[i + 2].trim()
            .split(' ')
            .map(|s| s.to_string())
            .collect::<Vec<String>>()[3..].to_vec();
        let test: u32 = lines[i + 3].trim()
            .split(' ').last().unwrap().parse::<u32>().unwrap();
        let dsts: Vec<u32> = lines[(i + 4)..=(i + 5)].iter()
            .map(|c| 
                c.trim()
                .split(' ').last().unwrap().parse::<u32>().unwrap()
            ).collect::<Vec<u32>>();
        monkies.push(Monkey { items: starting_items, op, test, dsts, inspections: 0});
        i += 7;
    }
    monkies
}

#[derive(Clone)]
struct Monkey {
    items: Vec<u64>,
    op: Vec<String>,
    test: u32,
    dsts: Vec<u32>,
    inspections: u64
}