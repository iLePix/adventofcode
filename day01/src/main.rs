use std::{fs::File, io::{BufReader, BufRead, Lines}};

fn main() {
    let path = "./input.txt".to_string();
    
    println!("Elf with most calories: ");
    println!("{}", best_elf(&path));
    println!("");
    println!("Top 3 elves with most calories: ");
    println!("{}", best_3elves(&path));
}

fn read_lines(path: &String) -> Lines<BufReader<File>>{
    let file = File::open(path).expect("Failed to open File");
    BufReader::new(file).lines()
}


fn best_elf(path: &String) -> u32 {
    let mut best_elf = 0;
    let mut cur_elf = 0;
    for line in read_lines(path) {
        if let Ok(line) = line {
            if line.is_empty() {
                if cur_elf > best_elf {
                    best_elf = cur_elf;
                }
                cur_elf = 0;     
            } else {
                let cal = line.parse::<u32>().expect("Couldnt parse int, or negative value");
                cur_elf += cal;
            }
        }
    }
    best_elf as u32
}

fn best_3elves(path: &String) -> u32 {
    // 0 = lowest, 2 = highest
    let mut best_elves = [0; 3];
    let mut cur_elf = 0;

    fn compare(cur_elf: u32, best_elves: &mut [u32; 3], i: i32) {
        if i < 0 {
            return
        }

        if best_elves[i as usize] < cur_elf {
            compare(best_elves[i as usize], best_elves, i - 1);
            best_elves[i as usize] = cur_elf;
        } else {
            compare(cur_elf, best_elves, i - 1);
        }
    }

    for line in read_lines(path) {
        if let Ok(line) = line {
            if line.is_empty() {
                compare(cur_elf, &mut best_elves, 2);
                cur_elf = 0;     
            } else {
                let cal = line.parse::<u32>().expect("Couldnt parse int, or negative value");
                cur_elf += cal;
            }
        }
    }
    best_elves.iter().sum()
}