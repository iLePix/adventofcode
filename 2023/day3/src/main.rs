use std::collections::HashMap;

fn main() {
    let s = std::fs::read_to_string("i.txt").unwrap();
    let matrix: Vec<Vec<char>> = s.lines().map(|l| l.chars().collect()).collect();
    let mut sum = 0;
    let mut gear_sum = 0;
    let mut possible_gears: HashMap<(usize, usize), u32> = HashMap::new();
    for (li, l) in matrix.iter().enumerate() {
        let mut number = 0;
        let mut number_length = 0;
        for (ci, c) in l.iter().enumerate() {
            if c.is_numeric() {
                number *= 10;
                number += c.to_digit(10).unwrap();
                number_length += 1;
            }
            if !c.is_numeric() && number > 0 || (number_length != 0 && number > 0 && ci + 1 == l.len()) {
                let x_start = if ci - number_length > 0 { ci - number_length - 1 } else { 0 };
                let x_end = ci;
                let y_start = if li > 0 { li-1 } else {0};
                let y_end = if li < matrix.len() - 1 { li + 1 } else { li };
                'outer: for x in x_start..=x_end {
                    for y in y_start..=y_end {
                        let l = matrix[y][x];
                        if let Some(n) = possible_gears.get(&(y,x)) {
                            gear_sum += n * number;
                            sum += number;
                            break 'outer;
                        }
                        if !l.is_digit(10) && l != '.' {
                            if l == '*' {
                                possible_gears.insert((y, x), number);
                            }
                            sum += number;
                            break 'outer;
                        }
                    }
                }
                number_length = 0;
                number = 0;
            }
        }
    }
    println!("task1: {} | task2: {} ", sum, gear_sum)
}