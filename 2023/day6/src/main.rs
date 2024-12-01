use std::{ops::{RangeInclusive, Range}, slice::Chunks};



fn main() {
    let s = std::fs::read_to_string("i.txt").unwrap();
    let mut sum = 1;
    let lines: Vec<&str> = s.lines().collect();
    let times = parse_nums_option(&lines[0].replace(" ", ""));
    let distances = parse_nums_option(&lines[1].replace(" ", ""));
    let acc = 1; //ms/s2
    for i in 0..times.len() {
        let mut race_score = 0;
        let time = times[i];
        let distance = distances[i];
        for ms in 0..=time {
            let speed = acc * ms;
            let distance_traveled = speed * (time - ms);
            if distance_traveled > distance {
                race_score += 1;
            }
        }
        if race_score > 0 {
            sum *= race_score
        }
    }
    println!("{sum}")
}



fn parse_nums_option(s: &str) -> Vec<u64> {
    s.split(":").filter_map(|i| i.parse::<u64>().ok()).collect()
}


fn parse_nums(s: &str) -> Vec<u64> {
    s.split_ascii_whitespace().map(|i| i.parse::<u64>().unwrap()).collect()
}
