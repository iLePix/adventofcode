use std::{fs::File, io::{BufReader, BufRead}, slice::Iter, collections::{HashSet, HashMap}, iter};
use colored::Colorize;

fn main() -> std::io::Result<()> {
    let path = "./input.txt";
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines().filter_map(|lr| lr.ok()).collect::<Vec<String>>();
    let hm = parse_height_map(lines);
    println!("Part 1: {:?}", bfs(hm.start, hm.dst, hm.height_map.clone()).unwrap().len());
    println!("Part 2: {:?}", part2(hm.dst, hm.height_map.clone()));
    Ok(())
}

fn part2(dst: (usize, usize), map: Vec<Vec<u8>>) -> usize {
    let start_positions = map.iter()
        .enumerate()
        .flat_map(|(x, column)| 
            column.iter()
            .enumerate()
            .filter(|(_, h)| **h == 'a' as u8)
            .map(move |(y, _)| (x, y))
        ).collect::<Vec<(usize, usize)>>();
    let mut a = Vec::new();
    for s_pos in start_positions {
        if let Some(p) = bfs(s_pos, dst, map.clone()) {
            a.push(p.len())
        }
    }
    *a.iter().min().unwrap()
}

fn bfs(start: (usize, usize), dst: (usize, usize), map: Vec<Vec<u8>>) -> Option<Vec<(usize, usize)>> {
    let mut visited = vec![vec![false; map[0].len()]; map.len()];
    visited[start.0][start.1] = true;
    let mut queue: Vec<(usize, usize)> = vec![];
    queue.push(start);
    let mut prevs = vec![vec![None; map[0].len()]; map.len()];
    let dirs: [(i32, i32); 4] = [
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1)
    ];
    let is_in_map= |pos: (i32, i32)| -> bool {
        map.len() as i32 > pos.0 && pos.0 >= 0 && map[0].len() as i32 > pos.1 && pos.1 >= 0
    };

    while !queue.is_empty() {
        let node = queue[0];
        queue.remove(0);
        for dir in dirs {
            let new_pos = (node.0 as i32 + dir.0, node.1 as i32 + dir.1);
            if !is_in_map(new_pos){
                continue;
            }
            let new_pos_i = (new_pos.0 as usize, new_pos.1 as usize);
            let cur_height = map[node.0][node.1] as i32;
            let dst_height = map[new_pos_i.0][new_pos_i.1] as i32;
            if ((cur_height - dst_height).abs() <= 1 || dst_height < cur_height) && !visited[new_pos_i.0][new_pos_i.1] {
                queue.push(new_pos_i);
                prevs[new_pos_i.0][new_pos_i.1] = Some(node);
                visited[new_pos_i.0][new_pos_i.1] = true;
            }
        }
    }

    let mut path = vec![];
    let mut at = dst;
    while let Some(n) = prevs[at.0][at.1] {
        at = n;
        path.push(n);
    }
    path.reverse();
    if path.len() > 0 && path[0] == start {
        return Some(path)
    }
    None
}



fn parse_height_map(lines: Vec<String>) -> HeightMap {
    let size = (lines[0].len(), lines.len());
    let mut height_map = HeightMap { start: (0, 0), dst: (0, 0), height_map: vec![vec![0; size.1]; size.0] };
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                'S' => {
                    height_map.start = (x, y);
                    height_map.height_map[x][y] = 'a' as u8
                },
                'E' => {
                    height_map.dst = (x, y);
                    height_map.height_map[x][y] = 'z' as u8
                },
                c => height_map.height_map[x][y] = c as u8
            }
        }
    }
    height_map
}

#[derive(Debug)]
struct HeightMap {
    pub start: (usize, usize),
    pub dst: (usize, usize),
    pub height_map: Vec<Vec<u8>>,
}