use std::{ops::{RangeInclusive, Range}, slice::Chunks};



fn main1() {
    let s = std::fs::read_to_string("l.txt").unwrap();
    let mut sum = 0;
    let lines: Vec<&str> = s.lines().collect();
    let seeds_raw: Vec<&str> = lines.first().unwrap().split(":").collect();
    let mut seed_ranges: Vec<u64> = parse_nums(&seeds_raw.last().unwrap().trim());
    let mut seeds = vec![];
    let mut i = 0;
    while i < seed_ranges.len() {
        for f in 0..seed_ranges[i + 1] {
            seeds.push(seed_ranges[i] + f);
        }
        i += 2;
    }



    let mut src_ranges: Vec<RangeInclusive<u64>> = vec![];
    let mut dst_ranges: Vec<RangeInclusive<u64>> = vec![];
    let mut lf = 0;
    for l in lines[1..].iter() {
        lf += 1;
        println!("{:.2}%", lf as f32 / lines.len() as f32 * 100.0);
        let nums = parse_nums_option(l);
        if nums.len() < 1 {
            for seed in seeds.iter_mut() {
                for i in 0..src_ranges.len() {
                    let src_range = src_ranges[i].clone();
                    let dst_range = dst_ranges[i].clone();
                    if src_range.contains(seed) {
                        *seed = *seed - src_range.start() + dst_range.start();
                        break;
                    }
                }
            }
            //println!("src: {:?}", src_ranges);
            //println!("dst: {:?}", dst_ranges);
            //println!("seeds: {:?}", seeds);
            src_ranges.clear();
            dst_ranges.clear();
        } else {
            let dst_range = nums[0]..=(nums[0] + nums[2] - 1);
            let src_range = nums[1]..=(nums[1] + nums[2] - 1);
            src_ranges.push(src_range);
            dst_ranges.push(dst_range);
        }
    }
    println!("result: {:?}", seeds.iter().min());
}





fn main() {
    let s = std::fs::read_to_string("i.txt").unwrap();
    let mut sum = 0;
    let lines: Vec<&str> = s.lines().collect();
    let seeds_raw: Vec<&str> = lines.first().unwrap().split(":").collect();
    let mut seeds: Vec<u64> = parse_nums(&seeds_raw.last().unwrap().trim());
    let mut seed_ranges: Vec<RangeInclusive<u64>> = vec![];
    let mut i = 0;
    while i < seeds.len() {
        let range: RangeInclusive<u64> = seeds[i]..=(seeds[i] + seeds[i + 1] - 1);
        seed_ranges.push(range);
        i += 2;
    }

    let mut src_ranges: Vec<RangeInclusive<u64>> = vec![];
    let mut dst_ranges: Vec<RangeInclusive<u64>> = vec![];
    for l in lines[1..].iter() {
        let nums = parse_nums_option(l);
        if nums.len() < 1 {
            let mut new_seed_ranges: Vec<RangeInclusive<u64>> = vec![];
            for seed_range in seed_ranges.iter_mut() {
                for i in 0..src_ranges.len() {
                    let src_range: RangeInclusive<u64> = src_ranges[i].clone();
                    let dst_range: RangeInclusive<u64> = dst_ranges[i].clone();
                    let seed_range_start = *seed_range.start();
                    let seed_range_end = *seed_range.end();
                    let start = src_range.start().max(&seed_range_start);
                    let end = src_range.end().min(&seed_range_end);
                    if start < end {
                        let offset: i64 = *src_range.start() as i64 - *dst_range.start() as i64;
                        *seed_range = RangeInclusive::new((*start as i64 - offset) as u64, (*end as i64 - offset) as u64 - 1);
                        if seed_range.start() < start {
                            new_seed_ranges.push(RangeInclusive::new(seed_range_end, *start));
                        }
                        if end < seed_range.end() {
                            new_seed_ranges.push(RangeInclusive::new(*end, seed_range_end));
                        }
                        break;
                    }
                }
            }
            seed_ranges.extend_from_slice(&new_seed_ranges);
            //println!("src: {:?}", src_ranges);
            //println!("dst: {:?}", dst_ranges);
            //println!("{:?}", seed_ranges);
            src_ranges.clear();
            dst_ranges.clear();
        } else {
            let dst_range: RangeInclusive<u64> = nums[0]..=(nums[0] + nums[2] - 1);
            let src_range: RangeInclusive<u64> = nums[1]..=(nums[1] + nums[2] - 1);
            src_ranges.push(src_range);
            dst_ranges.push(dst_range);
        }
    }
    println!("{:?}", seed_ranges);
    let mut min = u64::MAX;
    for range in seed_ranges {
        if *range.start() < min {
            min = *range.start();
        }
    }
    println!("{min}");
}




fn parse_nums_option(s: &str) -> Vec<u64> {
    s.split_ascii_whitespace().filter_map(|i| i.parse::<u64>().ok()).collect()
}


fn parse_nums(s: &str) -> Vec<u64> {
    s.split_ascii_whitespace().map(|i| i.parse::<u64>().unwrap()).collect()
}
