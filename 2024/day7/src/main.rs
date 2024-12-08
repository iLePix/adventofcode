use std::ops::{Add, Mul};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("./input.txt")?;
    let eqs: Vec<(i64, Vec<i64>)> = input
        .lines()
        .map(|l| {
            let (res, vals) = l.split_once(": ").unwrap();
            (
                res.parse::<i64>().unwrap(),
                vals.split(' ').map(|s| s.parse::<i64>().unwrap()).collect(),
            )
        })
        .collect();
    let task1 = check(&eqs, &[Mul::mul, Add::add]);
    dbg!(task1);
    let task2 = check(&eqs, &[Mul::mul, Add::add, concat]);
    dbg!(task2);
    Ok(())
}

fn check(eqs: &[(i64, Vec<i64>)], ops: &[fn(i64, i64) -> i64]) -> i64 {
    eqs.iter()
        .filter(|(res, vals)| eval_eq(*res, vals, ops))
        .map(|(res, _)| *res)
        .sum()
}

fn concat(x: i64, y: i64) -> i64 {
    format!("{x}{y}").parse().unwrap()
}

fn eval_eq(res: i64, vals: &[i64], ops: &[fn(i64, i64) -> i64]) -> bool {
    let mut results = vec![vals[0]];
    for &val in &vals[1..] {
        let mut new_results = Vec::new();
        for &result in &results {
            for op in ops {
                new_results.push(op(result, val));
            }
        }
        results = new_results;
    }
    results.contains(&res)
}
