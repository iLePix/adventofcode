use std::{fs::File, io::{BufReader, BufRead}, collections::HashSet};

fn main() -> std::io::Result<()> {
    let path = "./input.txt";
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().filter_map(|lr| lr.ok()).collect();
    
    let uniq_items_val: u32 = lines.iter().map(|l| {
        let (first, last) = l.split_at(l.len() / 2);
        let h: HashSet<char> = HashSet::from_iter(first.chars());
        for c in last.chars() {
            if h.contains(&c) {
                return value(c) as u32
            }
        }
        0
    }).sum();

    let total_badge_val: u32 = lines.chunks(3).map(|l| {
        let h1: HashSet<char> = HashSet::from_iter(l[0].chars());
        let mut h2:  HashSet<char> = HashSet::new();
        for c in l[1].chars() {
            if h1.contains(&c) {
                h2.insert(c);
            }
        }
        for c in l[2].chars() {
            if h2.contains(&c) {
                return value(c) as u32
            }
        }
        0
    }).sum();



    println!("Unique item value:");
    println!("{}", uniq_items_val);
    println!("Unique badge value:");
    println!("{}", total_badge_val);
    Ok(())
}



fn value(c: char) -> u8 {
    match c {
        'a'..='z' => c as u8 - 'a' as u8 + 1 ,
        'A'..='Z' => c as u8 - 'A' as u8 + 27 ,
        _ => panic!("Undefined char")
    }
}