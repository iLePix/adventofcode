use std::collections::{HashMap, HashSet};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("./input.txt")?;
    let map: Vec<Vec<u8>> = input.lines().map(|l| l.bytes().collect()).collect();
    let (height, width) = (map.len(), map[0].len());
    let mut freq_locs: HashMap<u8, Vec<(i32, i32)>> = HashMap::new();
    for (y, row) in map.iter().enumerate() {
        for (x, &freq) in row.iter().enumerate() {
            if freq != b'.' {
                freq_locs
                    .entry(freq)
                    .or_default()
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
        for (i, &a) in locs.iter().enumerate() {
            for (j, &b) in locs.iter().enumerate() {
                if i != j {
                    let antinode = (a.0 + (a.0 - b.0), a.1 + (a.1 - b.1));
                    if in_bounds(antinode.0, antinode.1, width, height) {
                        antinodes.insert(antinode);
                    }
                }
            }
        }
    }
    antinodes.len()
}

fn task2(freq_locs: &HashMap<u8, Vec<(i32, i32)>>, width: usize, height: usize) -> usize {
    let mut antinodes = HashSet::new();
    for locs in freq_locs.values() {
        for (i, &a) in locs.iter().enumerate() {
            for (j, &b) in locs.iter().enumerate() {
                if i != j {
                    let (dx, dy) = (a.0 - b.0, a.1 - b.1);
                    let mut antinode = (a.0, a.1);
                    while in_bounds(antinode.0, antinode.1, width, height) {
                        antinodes.insert(antinode);
                        antinode = (antinode.0 + dx, antinode.1 + dy);
                    }
                }
            }
        }
    }
    antinodes.len()
}
fn in_bounds(x: i32, y: i32, width: usize, height: usize) -> bool {
    x >= 0 && y >= 0 && x < width as i32 && y < height as i32
}
