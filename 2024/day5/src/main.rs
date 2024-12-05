use std::collections::HashMap;

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
    dbg!(&order_rules);
    let mut sum = 0;
    'upd_loop: for update in updates.split('\n') {
        let nums: Vec<i32> = update
            .split(',')
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();
        if nums.len() < 1 {
            break;
        }
        for i in 1..nums.len() {
            if order_rules
                .get(&nums[i])
                .is_some_and(|after| after.iter().any(|&item| nums[..i].contains(&item)))
            {
                continue 'upd_loop;
            }
        }
        let middle = nums[nums.len() / 2];
        sum += middle;
        println!("found valid update {middle}");
    }
    println!("{sum}");
    Ok(())
}
