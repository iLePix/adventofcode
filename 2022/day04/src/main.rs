use std::{io::{BufReader, BufRead}, fs::File, ops::RangeInclusive, cmp::{max, min}};

fn main() -> std::io::Result<()> {
    let path = "./input.txt";
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().filter_map(|lr| lr.ok()).collect();
    let ranges: Vec<Vec<RangeInclusive<i32>>> = lines.iter().map(|line| {
        let ranges = line.split(',').into_iter().map(|range| {
            let r: Vec<i32> = range.split('-').map(|i| i.parse::<i32>().unwrap()).collect();
            RangeInclusive::new(r[0], r[1])
        }).collect::<Vec<RangeInclusive<i32>>>();
        ranges
    }).collect();


    let contains = lines.iter().filter(|line| {
        let mut ranges = line.split(',').into_iter().map(|range| {
            let r: Vec<i32> = range.split('-').map(|i| i.parse::<i32>().unwrap()).collect();
            (RangeInclusive::new(r[0], r[1]), r[1] - r[0])
        }).collect::<Vec<(RangeInclusive<i32>, i32)>>();
        ranges.sort_by(|a, b| b.1.cmp(&a.1));
        ranges[0].0.start() <= ranges[1].0.start() &&  ranges[0].0.end() >= ranges[1].0.end()
    }).count();

    let overlaps = lines.iter().filter(|line| {
        let ranges = line.split(',').into_iter().map(|range| {
            let r: Vec<i32> = range.split('-').map(|i| i.parse::<i32>().unwrap()).collect();
            RangeInclusive::new(r[0], r[1])
        }).collect::<Vec<RangeInclusive<i32>>>();
        ranges[0].start() <= ranges[1].end() && ranges[0].end() >= ranges[1].start()
    }).count();

    println!("Number of assignment pairs where one range fully contains the other:");
    println!("{}", contains);

    println!("Number of assignment pairs where ranges overlap:");
    println!("{}", overlaps);

    Ok(())
}
