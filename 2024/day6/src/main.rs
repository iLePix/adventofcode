use std::{collections::HashSet, usize};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn to_tuple(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
    pub fn rotate_90_degrees_right(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("./input.txt")?;
    let map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut set = HashSet::new();
    let height = map.len();
    let width = map.first().unwrap().len();
    let mut guard_position = (0, 0);
    let mut guard_direction = Direction::Up;
    let mut guard_starting_position = guard_position;
    for x in 0..width {
        for y in 0..height {
            if map[y][x] == '^' {
                guard_position = (x as i32, y as i32);
                guard_starting_position = guard_position;
            }
        }
    }
    while in_bounds(guard_position, height, width) {
        set.insert((guard_position.0, guard_position.1, guard_direction));
        let direction = guard_direction.to_tuple();
        let new_position = (
            guard_position.0 + direction.0,
            guard_position.1 + direction.1,
        );
        if !in_bounds(new_position, height, width) {
            break;
        }
        if map[new_position.1 as usize][new_position.0 as usize] == '#' {
            guard_direction = guard_direction.rotate_90_degrees_right();
        } else {
            guard_position = new_position
        }
    }
    let task1 = set
        .iter()
        .map(|&(a, b, _)| (a, b))
        .collect::<HashSet<_>>()
        .len();
    dbg!(task1);

    let task2 = set
        .iter()
        .filter_map(|&(a, b, _)| {
            let mut path = HashSet::new();
            let obs = (a, b);
            let mut pos = guard_starting_position;
            let mut direction = Direction::Up;
            while in_bounds(pos, width, height) {
                if !path.insert((pos.0, pos.1, direction)) {
                    return Some(obs);
                }
                let dir = direction.to_tuple();
                let new = (pos.0 + dir.0, pos.1 + dir.1);
                if in_bounds(new, height, width)
                    && (map[new.1 as usize][new.0 as usize] == '#' || new == obs)
                {
                    direction = direction.rotate_90_degrees_right();
                } else {
                    pos = new;
                }
            }
            None
        })
        .collect::<HashSet<(i32, i32)>>()
        .len();
    dbg!(task2);
    Ok(())
}

fn in_bounds(pos: (i32, i32), height: usize, width: usize) -> bool {
    pos.0 >= 0 && pos.0 < width as i32 && pos.1 >= 0 && pos.1 < height as i32
}
