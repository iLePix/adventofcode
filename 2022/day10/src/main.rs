use std::{fs::File, io::{BufReader, BufRead}, slice::Iter, collections::HashSet};

fn main() -> std::io::Result<()> {
    let path = "./input.txt";
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let lines = reader.lines().filter_map(|lr| lr.ok()).collect::<Vec<String>>();
    let insts = parse_steps(lines.iter());
    let mut x = 1;
    let mut y = 0;
    let mut cycles: Vec<i32> = vec![];
    let mut screen = [[' '; 40]; 6];
    for inst in insts.iter() {
        match inst {
            Inst::Addx(a) => {
                for _ in 0..inst.cycles() {
                    cycles.push(x)
                }
                x += a;
            },
            Inst::Noop => cycles.push(x),
        }
    }

    for (i, c) in cycles.iter().enumerate() {
        if (i+1) % 40 == 0 && i > 0 {
            y += 1;
        }
        let draw_pos = (i) % 40;
        for p in 0..3 {
            let pos = p+c;
            if pos >= 0 && pos <=39 && pos - 1 == draw_pos as i32 {
                screen[y][draw_pos] = '█';
            }
        }
    }


    let v = cycles.iter()
        .enumerate()
        .filter(|(i, _)| ((*i as i32 - 19) % 40 == 0))
        .map(|(i,x)| {
            (i + 1) as i32 * x
        })
        .sum::<i32>();

    println!("Part 1: {}" , v);
    

    println!("Part 2:");
    for y in screen {
        for x in y {
            print!("{}", x)
        }
        println!();
    }
    Ok(())
}


fn parse_steps(lines: Iter<String>) -> Vec<Inst> {
    lines.map(|inst| {
        let s: Vec<&str> = inst.split(" ").collect();
        match s[0] {
            "noop" => Inst::Noop,
            "addx" => {
                let amount = s[1].parse::<i32>().unwrap(); 
                Inst::Addx(amount)
            },
            c => panic!("Unknown dir: {}", c)
        }
    }).collect::<Vec<Inst>>()
}

#[derive(Clone, Copy)]
enum Inst {
    Addx(i32),
    Noop
}

impl Inst {
    pub fn cycles(self) -> usize {
        match self {
            Inst::Addx(_) => 2,
            Inst::Noop => 1,
        }
    }
}