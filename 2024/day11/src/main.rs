use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("./input.txt")?;
    let nums: Vec<i64> = input
        .split(' ')
        .map(|s| s.parse())
        .collect::<Result<_, _>>()?;
    let mut stones: HashMap<i64, u64> = HashMap::new();
    for num in nums {
        *stones.entry(num).or_insert(0) += 1;
    }
    let task1 = blink(stones.clone(), 25);
    dbg!(task1);
    let task2 = blink(stones, 75);
    dbg!(task2);
    Ok(())
}

fn blink(mut stones: HashMap<i64, u64>, times: u32) -> u64 {
    for _ in 0..times {
        let mut new_stones = HashMap::new();
        for (&stone, &count) in stones.iter() {
            if stone != 0 {
                let n_digits = ((stone.abs() as f64).log10().floor() as usize) + 1;
                if n_digits % 2 == 0 {
                    let left = stone / 10_i64.pow(n_digits as u32 / 2);
                    let right = stone - left * 10_i64.pow(n_digits as u32 / 2);
                    *new_stones.entry(left).or_default() += count;
                    *new_stones.entry(right).or_default() += count;
                } else {
                    *new_stones.entry(2024 * stone).or_default() += count;
                }
            } else {
                *new_stones.entry(1).or_default() += count;
            }
        }
        stones = new_stones;
    }
    stones.values().sum()
}
