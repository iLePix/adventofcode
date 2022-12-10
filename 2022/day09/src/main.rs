use std::{fs::File, io::{BufReader, BufRead}, slice::Iter, collections::HashSet};

fn main() -> std::io::Result<()> {
    let path = "./input.txt";
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let lines = reader.lines().filter_map(|lr| lr.ok()).collect::<Vec<String>>();
    let steps = parse_steps(lines.iter());
    println!("Part 1: {}" , count_uniq_tail_pos(&steps, 2));
    println!("Part 2: {}" , count_uniq_tail_pos(&steps, 10));

    Ok(())
}

fn parse_steps(lines: Iter<String>) -> Vec<Step> {
    lines.map(|inst| {
        let s: Vec<&str> = inst.split(" ").collect();
        let amount = s[1].parse::<i32>().unwrap(); 
        let dir = match s[0] {
            "L" => (-1, 0),
            "R" => (1, 0),
            "U" => (0, 1),
            "D" => (0, -1),
            c => panic!("Unknown dir: {}", c)
        };
        Step {dir, amount}
    }).collect::<Vec<Step>>()
}

fn count_uniq_tail_pos(steps: &Vec<Step>, len: usize) -> usize {
    let mut tails_pos = vec![(0,0); len];
    let mut pos_visited: HashSet<(i32, i32)> = HashSet::new();
    for step in steps {
        for _ in 0..step.amount {
            tails_pos[0].0 += step.dir.0;
            tails_pos[0].1 += step.dir.1;
            for t in 1..tails_pos.len() {
                let h_pos = tails_pos[t - 1];
                let mut tail_pos = &mut tails_pos[t];
                let x_dst = h_pos.0 - tail_pos.0;
                let y_dst = h_pos.1 - tail_pos.1;
                if x_dst.abs() > 1 || y_dst.abs() > 1 {
                    if h_pos.0 == tail_pos.0 {
                        tail_pos.1 += y_dst / 2;
                    } else if h_pos.1 == tail_pos.1 {
                        tail_pos.0 += x_dst / 2;
                    } else {
                        tail_pos.1 += y_dst / y_dst.abs();
                        tail_pos.0 += x_dst / x_dst.abs();
                    }
                }
            }
            pos_visited.insert(*tails_pos.last().unwrap());
        }
    }
    pos_visited.len()
}

struct Step {
    dir: (i32, i32),
    amount: i32
}