use std::{collections::HashMap, ops::RangeInclusive, slice::Chunks};



fn main() {
    let s = std::fs::read_to_string("i.txt").unwrap();
    let mut sum = 0;
    let lines: Vec<&str> = s.lines().collect();
    let seeds_raw: Vec<&str> = lines.first().unwrap().split(":").collect();
    let mut seeds: Vec<u128> = parse_nums(&seeds_raw.last().unwrap().trim());
    let mut seed_ranges: Vec<RangeInclusive<u128>> = vec![];
    let mut i = 0;
    while i < seeds.len() {
        let range: RangeInclusive<u128> = seeds[i]..=(seeds[i] + seeds[i + 1] - 1);
        seed_ranges.push(range);
        i += 2;
    }

    let mut src_ranges: Vec<RangeInclusive<u128>> = vec![];
    let mut dst_ranges: Vec<RangeInclusive<u128>> = vec![];
    for l in lines[1..].iter() {
        let nums = parse_nums_option(l);
        if nums.len() < 1 {
            for seed_range in seed_ranges.iter_mut() {
                for i in 0..src_ranges.len() {
                    let src_range: RangeInclusive<u128> = src_ranges[i].clone();
                    let dst_range: RangeInclusive<u128> = dst_ranges[i].clone();
                    let start = src_range.start().max(seed_range.start());
                    let end = src_range.end().min(seed_range.end());
                    if start < end {
                        let offset: i128 = *src_range.start() as i128 - *dst_range.start() as i128;
                        *seed_range = RangeInclusive::new((*start as i128 - offset) as u128, (*end as i128 - offset) as u128 - 1);
                        break;
                    }
                }
            }
            //println!("src: {:?}", src_ranges);
            //println!("dst: {:?}", dst_ranges);
            //println!("{:?}", seed_ranges);
            src_ranges.clear();
            dst_ranges.clear();
        } else {
            let dst_range: RangeInclusive<u128> = nums[0]..=(nums[0] + nums[2] - 1);
            let src_range: RangeInclusive<u128> = nums[1]..=(nums[1] + nums[2] - 1);
            src_ranges.push(src_range);
            dst_ranges.push(dst_range);
        }
    }
    println!("{:?}", seed_ranges);
    let mut min = u128::MAX;
    for range in seed_ranges {
        if *range.start() < min {
            min = *range.start();
        }
    }
    println!("{min}");
}



fn parse_nums_option(s: &str) -> Vec<u128> {
    s.split_ascii_whitespace().filter_map(|i| i.parse::<u128>().ok()).collect()
}


fn parse_nums(s: &str) -> Vec<u128> {
    s.split_ascii_whitespace().map(|i| i.parse::<u128>().unwrap()).collect()
}


fn f() {
    let s = std::fs::read_to_string("i.txt").unwrap();
    let mut sum = 0;
    let lines: Vec<&str> = s.lines().collect();
    let seeds_raw: Vec<&str> = lines.first().unwrap().split(":").collect();
    let mut seeds: Vec<u128> = parse_nums(&seeds_raw.last().unwrap().trim());
    let mut seed_ranges: Vec<RangeInclusive<u128>> = vec![];
    let mut i = 0;
    while i < seeds.len() {
        let range: RangeInclusive<u128> = seeds[i]..=(seeds[i] + seeds[i + 1] - 1);
        seed_ranges.push(range);
        i += 2;
    }

    let mut src_ranges: Vec<RangeInclusive<u128>> = vec![];
    let mut dst_ranges: Vec<RangeInclusive<u128>> = vec![];
    for l in lines[1..].iter() {
        let nums = parse_nums_option(l);
        if nums.len() < 1 {
            for seed_range in seed_ranges.iter_mut() {
                for i in 0..src_ranges.len() {
                    let src_range: RangeInclusive<u128> = src_ranges[i].clone();
                    let dst_range: RangeInclusive<u128> = dst_ranges[i].clone();
                    let start = src_range.start().max(seed_range.start());
                    let end = src_range.end().min(seed_range.end());
                    if start < end {
                        let offset: i128 = *src_range.start() as i128 - *dst_range.start() as i128;
                        *seed_range = RangeInclusive::new((*start as i128 - offset) as u128, (*end as i128 - offset) as u128 - 1);
                        break;
                    }
                }
            }
            //println!("src: {:?}", src_ranges);
            //println!("dst: {:?}", dst_ranges);
            //println!("{:?}", seed_ranges);
            src_ranges.clear();
            dst_ranges.clear();
        } else {
            let dst_range: RangeInclusive<u128> = nums[0]..=(nums[0] + nums[2] - 1);
            let src_range: RangeInclusive<u128> = nums[1]..=(nums[1] + nums[2] - 1);
            src_ranges.push(src_range);
            dst_ranges.push(dst_range);
        }
    }
    println!("{:?}", seed_ranges);
    let mut min = u128::MAX;
    for range in seed_ranges {
        if *range.start() < min {
            min = *range.start();
        }
    }
    println!("{min}");
}
