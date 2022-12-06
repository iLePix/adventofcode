use std::{io::{BufReader, BufRead}, fs::File};

fn main() -> std::io::Result<()> {
    let path = "./input.txt";
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let lines = reader.lines().filter_map(|lr| lr.ok()).collect::<Vec<String>>();
    
    println!("Part 1: {:?}", find_marker(4, lines.first().unwrap()));
    println!("Part 2: {:?}", find_marker(14, lines.first().unwrap()));

    Ok(())
}

fn find_marker(l: usize, line: &String) -> Option<usize> {
    line.chars()
        .collect::<Vec<char>>()
        .windows(l).map(|w| {
            w.into_iter().enumerate().all(|(i,c)| !w.iter().enumerate().filter(|(n,c)| i != *n).map(|(_,c)| c).collect::<Vec<&char>>().contains(&c))
        })
        .enumerate()
        .find(|predicate| predicate.1).map(|(i, _)| i + l)
}
