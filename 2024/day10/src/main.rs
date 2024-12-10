use std::collections::HashSet;

const DIRS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("./input.txt")?;
    let height_map: Vec<Vec<u8>> = input
        .lines()
        .map(|l| l.as_bytes().iter().map(|b| b - b'0').collect())
        .collect();
    let height = height_map.len();
    let width = height_map[0].len();
    let mut trail_heads = Vec::new();
    for x in 0..width {
        for y in 0..height {
            if height_map[y][x] == 0 {
                trail_heads.push((x as i32, y as i32));
            }
        }
    }
    let task1: u32 = trail_heads
        .iter()
        .map(|t| eval_trail(*t, &height_map, width, height, &mut Some(HashSet::new())))
        .sum();
    dbg!(task1);
    let task2: u32 = trail_heads
        .iter()
        .map(|t| eval_trail(*t, &height_map, width, height, &mut None))
        .sum();
    dbg!(task2);
    Ok(())
}

fn eval_trail(
    pos: (i32, i32),
    map: &[Vec<u8>],
    map_width: usize,
    map_height: usize,
    visited: &mut Option<HashSet<(i32, i32)>>,
) -> u32 {
    let height = map[pos.1 as usize][pos.0 as usize];
    if height == 9 {
        return (visited.is_none() || visited.as_mut().is_some_and(|v| v.insert(pos))) as u32;
    }
    let mut available_trails = 0;
    for dir in DIRS {
        let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
        if !in_bounds(new_pos, map_width, map_height) {
            continue;
        }
        let new_height = map[new_pos.1 as usize][new_pos.0 as usize];
        if new_height == height + 1 {
            available_trails += eval_trail(new_pos, map, map_width, map_height, visited);
        }
    }
    return available_trails;
}

fn in_bounds(pos: (i32, i32), width: usize, height: usize) -> bool {
    pos.0 >= 0 && pos.1 >= 0 && pos.0 < width as i32 && pos.1 < height as i32
}
