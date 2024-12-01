use std::{collections::{HashSet, HashMap, VecDeque}, ops::{Add, Neg, Sub}};

use color_format::{cprint, cformat};


#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Hash)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn fill(fill: i32) -> Self {
        Self { x: fill,  y: fill }
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}


impl Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self { x: -self.x, y: -self.y }
    }
}

//takes char outputs input dir and output dir
fn pipe(c: char) -> Option<(Vec2, Vec2)> {
    match c {
        '|' => Some((Vec2{x: 0, y: 1}, Vec2{x: 0, y: 1})),
        '-' => Some((Vec2{x: 1, y: 0}, Vec2{x: 1, y: 0})),
        'L' => Some((Vec2{x: 0, y: 1}, Vec2{x: 1, y: 0})),
        'J' => Some((Vec2{x: 0, y: 1}, Vec2{x: -1, y: 0})),
        '7' => Some((Vec2{x: 0, y: -1}, Vec2{x: -1, y: 0})),
        'F' => Some((Vec2{x: 0, y: -1}, Vec2{x: 1, y: 0})),
        '.' => None,
        _ => panic!("Unknown pipe for char {c}")
    }
}



fn main() {
    let s = std::fs::read_to_string("i.txt").unwrap();

    //[y][x]
    let map: Vec<Vec<char>> = s.lines().map(|l| l.chars().collect()).collect();

    //let instructions: Vec<char> = lines[0].chars().collect();
    let mut visited: HashMap<Vec2, u32> = HashMap::new();


    let mut current = Vec2::fill(0);

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            let cur = map[y][x];
            if cur == 'S' {
                current = Vec2 { x: x as i32, y: y as i32 };
            }
        }
    }

    let mut next = VecDeque::from([
        (current, Vec2 {x: 0, y: 1}),
        (current, Vec2 {x: 1, y: 0}),
        (current, Vec2 {x: 0, y: -1}),
        (current, Vec2 {x: -1, y: 0}),
    ]);

    visited.insert(current, 0);
    while let Some((current, n)) = next.pop_front() {
        let pos = current + n;
        if pos.x < 0 || pos.x >= map[0].len() as i32 || pos.y < 0 || pos.y >= map.len() as i32 {
            continue;
        }
        let c = map[pos.y as usize][pos.x as usize];
        let Some(p) = pipe(c) else {
            continue;
        };

        let dist = visited.get(&current).unwrap().clone();
        visited.insert(pos, dist + 1);

        if n == p.0 && *visited.get(&(pos + p.1)).unwrap_or(&u32::MAX) > dist {
            next.push_back((pos, p.1));
        } else if n == p.1 && *visited.get(&(pos + p.0)).unwrap_or(&u32::MAX) > dist  {
            next.push_back((pos, p.0));
        } else if n == -p.0 && *visited.get(&(pos - p.1)).unwrap_or(&u32::MAX) > dist  {
            next.push_back((pos, -p.1));
        } else if n == -p.1 && *visited.get(&(pos - p.0)).unwrap_or(&u32::MAX) > dist {
            next.push_back((pos, -p.0));
        }
    }

    let max_distance = visited.iter().map(|(_, v)| v).max();

    //println!("{:?}", visited);

    let mut distances_map = map.clone();
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            let pos = Vec2 { x: x as i32, y: y as i32 };
            if let Some(d) = visited.get(&pos) {
                if d == max_distance.unwrap() {
                    let message = cformat!("#green<{}>", distances_map[y][x]); 
                    cprint!("{}", message);
                    continue;
                }
                if pos == current {
                    let message = cformat!("#blue<{}>", distances_map[y][x]); 
                    cprint!("{}", message);
                    continue;
                }
                //distances_map[y][x] = (*d as u8 + '0' as u8) as char ;
                let message = cformat!("#red<{}>", distances_map[y][x]);
                cprint!("{}", message);
            } else {
                cprint!("{}", distances_map[y][x]);
            }
        }
        println!("");
    }




    println!("{:?}", max_distance.unwrap() / 2);
}


fn lcm(first: u64, second: u64) -> u64 {
    first * second / gcd(first, second)
}

fn gcd(first: u64, second: u64) -> u64 {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}





fn parse_nums_option(s: &str) -> Vec<u64> {
    s.split(":").filter_map(|i| i.parse::<u64>().ok()).collect()
}


fn parse_nums(s: &str) -> Vec<u64> {
    s.split_ascii_whitespace().map(|i| i.parse::<u64>().unwrap()).collect()
}
