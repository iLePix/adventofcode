use std::{cmp::Ordering, collections::HashMap};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("./input.txt")?;
    let (orders, updates) = input.split_once("\n\n").unwrap();
    let mut order_rules: HashMap<i32, Vec<i32>> = HashMap::new();
    for order in orders.split('\n') {
        let (before, after) = order.split_once('|').unwrap();
        let before = before.parse::<i32>().unwrap();
        let after = after.parse::<i32>().unwrap();
        order_rules
            .entry(before)
            .and_modify(|v| v.push(after))
            .or_insert(vec![after]);
    }
    let mut sum = 0;
    'upd_loop: for update in updates.split('\n') {
        let nums: Vec<i32> = update
            .split(',')
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();
        if nums.len() < 1 {
            break;
        }
        for i in 0..nums.len() {
            if order_rules
                .get(&nums[i])
                .is_some_and(|after| after.iter().any(|&item| nums[..i].contains(&item)))
            {
                let mut new_order = nums.clone();
                new_order.sort_by(|a, b| {
                    if order_rules.get(&a).is_some_and(|after| after.contains(b)) {
                        Ordering::Greater
                    } else {
                        Ordering::Less
                    }
                });
                let middle = new_order[nums.len() / 2];
                sum += middle;
                continue 'upd_loop;
                println!("found valid update {middle}");
            }
        }
    }
    println!("{sum}");
    Ok(())
}
