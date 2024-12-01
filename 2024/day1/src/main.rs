use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
};

type Splits = (Vec<u32>, Vec<u32>);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::File::open("./input.txt")?;
    let reader = BufReader::new(file);
    let mut splits = reader
        .lines()
        .map(|l| {
            let l = l?;
            let (ls, rs) = l.split_once("   ").ok_or("split failed")?;
            Ok((ls.parse::<u32>()?, rs.parse::<u32>()?))
        })
        .collect::<Result<Splits, Box<dyn std::error::Error>>>()?;
    dbg!(task1(&mut splits));
    dbg!(task2(&splits));
    Ok(())
}

fn task1((left, right): &mut Splits) -> u32 {
    left.sort();
    right.sort();
    left.iter()
        .zip(right.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .sum()
}

fn task2((left, right): &Splits) -> u32 {
    let mut counter: HashMap<u32, u32> = HashMap::new();
    for &r in right {
        *counter.entry(r).or_insert(1) += 1;
    }
    left.iter()
        .map(|l| *l * counter.get(l).copied().unwrap_or(0))
        .sum()
}
