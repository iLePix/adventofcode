use std::{fs::File, io::{BufReader, BufRead}, slice::Iter, collections::HashSet};

fn main() -> std::io::Result<()> {
    let path = "./input.txt";
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let lines = reader.lines().filter_map(|lr| lr.ok()).collect::<Vec<String>>();
    let steps = parse_steps(lines.iter());

    println!("Part 1: {}" , part1(&steps));
    println!("Part 2: {}" , part2(&steps));


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

fn part2(steps: &Vec<Step>) -> usize {
    let mut tails_pos = [(0,0); 9];
    let mut head_pos = (0,0);
    let mut pos_visited: HashSet<(i32, i32)> = HashSet::new();
    for step in steps {
        for _ in 0..step.amount {
            head_pos.0 += step.dir.0;
            head_pos.1 += step.dir.1;
            for t in 0..tails_pos.len() {
                let h_pos = if t == 0 {head_pos} else {tails_pos[t - 1]};
                let mut tail_pos = &mut tails_pos[t];
                let x_dst = h_pos.0 - tail_pos.0;
                let y_dst = h_pos.1 - tail_pos.1;
                if x_dst.abs() > 1 && y_dst == 0 {
                    tail_pos.0 += x_dst / 2
                } else if y_dst.abs() > 1 && x_dst == 0 {
                    tail_pos.1 += y_dst / 2
                } else if y_dst.abs() > 1 && x_dst.abs() == 1 {
                    tail_pos.1 += y_dst / 2;
                    tail_pos.0 += x_dst;
                } else if x_dst.abs() > 1 && y_dst.abs() == 1 {
                    tail_pos.1 += y_dst;
                    tail_pos.0 += x_dst / 2;
                }
            }
            pos_visited.insert(tails_pos[8]);
        }
    }
    pos_visited.len()
}

fn part1(steps: &Vec<Step>) -> usize {
    let mut tail_pos = (0,0);
    let mut head_pos = (0,0);
    //tail pos visited
    let mut pos_visited: HashSet<(i32, i32)> = HashSet::new();
    for step in steps {
        for _ in 0..=step.amount {
            head_pos.0 += step.dir.0;
            head_pos.1 += step.dir.1;
            let x_dst = head_pos.0 - tail_pos.0;
            let y_dst = head_pos.1 - tail_pos.1;
            if x_dst.abs() > 1 && y_dst == 0 {
                tail_pos.0 += x_dst / 2
            } else if y_dst.abs() > 1 && x_dst == 0 {
                tail_pos.1 += y_dst / 2
            } else if y_dst.abs() > 1 && x_dst.abs() == 1 {
                tail_pos.1 += y_dst / 2;
                tail_pos.0 += x_dst;
            } else if x_dst.abs() > 1 && y_dst.abs() == 1 {
                tail_pos.1 += y_dst;
                tail_pos.0 += x_dst / 2;
            }
            pos_visited.insert(tail_pos);
        }
    }
    pos_visited.len()
}

struct Step {
    dir: (i32, i32),
    amount: i32
}