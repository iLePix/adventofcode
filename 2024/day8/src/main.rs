use std::collections::{HashMap, HashSet};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("./input.txt")?;
    let map: Vec<Vec<u8>> = input.lines().map(|l| l.bytes().collect()).collect();
    let height = map.len();
    let width = map.first().unwrap().len();
    let mut freq_locs: HashMap<u8, Vec<(i32, i32)>> = HashMap::new();
    for x in 0..width {
        for y in 0..height {
            let freq = map[y][x];
            if freq != b'.' {
                freq_locs
                    .entry(freq)
                    .or_insert(vec![])
                    .push((x as i32, y as i32));
            }
        }
    }
    let task1 = task1(&freq_locs, width, height);
    let task2 = task2(&freq_locs, width, height);
    dbg!(task1);
    dbg!(task2);
    Ok(())
}

fn task1(freq_locs: &HashMap<u8, Vec<(i32, i32)>>, width: usize, height: usize) -> usize {
    let mut antinodes = HashSet::new();
    for locs in freq_locs.values() {
        for n in 0..locs.len() {
            for m in 0..locs.len() {
                if n == m {
                    continue;
                }
                let dist = (locs[n].0 - locs[m].0, locs[n].1 - locs[m].1);
                let antinode = (locs[n].0 + dist.0, locs[n].1 + dist.1);
                if in_bounds(antinode.0, antinode.1, width, height) {
                    antinodes.insert(antinode);
                }
            }
        }
    }
    antinodes.len()
}

fn task2(freq_locs: &HashMap<u8, Vec<(i32, i32)>>, width: usize, height: usize) -> usize {
    let mut antinodes = HashSet::new();
    for locs in freq_locs.values() {
        for n in 0..locs.len() {
            for m in 0..locs.len() {
                if n == m {
                    continue;
                }
                let dist = (locs[n].0 - locs[m].0, locs[n].1 - locs[m].1);
                let mut antinode = (locs[n].0, locs[n].1);
                while in_bounds(antinode.0, antinode.1, width, height) {
                    antinodes.insert(antinode);
                    antinode = (antinode.0 + dist.0, antinode.1 + dist.1);
                }
            }
        }
    }
    antinodes.len()
}

fn in_bounds(x: i32, y: i32, width: usize, height: usize) -> bool {
    x >= 0 && y >= 0 && x < width as i32 && y < height as i32
}
