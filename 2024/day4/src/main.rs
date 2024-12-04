use std::usize;

const TARGET: [char; 4] = ['X', 'M', 'A', 'S'];
type Map = Vec<Vec<char>>;
const DIRS: &[(i32, i32)] = &[
    (-1, -1),
    (-1, 1),
    (1, -1),
    (1, 1),
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1),
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input: Map = std::fs::read_to_string("./input.txt")?
        .trim()
        .split('\n')
        .map(|r| r.chars().collect())
        .collect();
    let height = input.len();
    let width = input.first().unwrap().len();
    let mut task1 = 0;
    let mut task2 = 0;
    for x in 0..width {
        for y in 0..height {
            if input[y][x] == 'X' {
                task1 += check_dirs(&input, width as i32, height as i32, x as i32, y as i32);
            }
            if input[y][x] == 'A' {
                task2 += found_xmas(&input, width as i32, height as i32, x as i32, y as i32) as u32;
            }
        }
    }
    dbg!(task1);
    dbg!(task2);
    Ok(())
}
fn found_xmas(input: &Map, width: i32, height: i32, x: i32, y: i32) -> bool {
    let mut s_cnt = (0, 0);
    let mut m_cnt = (0, 0);
    for (xdir, ydir) in DIRS[..4].iter() {
        let nx = x + xdir;
        let ny = y + ydir;
        if nx < 0 || nx >= width || ny < 0 || ny >= height {
            return false;
        }
        let c = input[ny as usize][nx as usize];
        s_cnt.0 += (c == 'S') as u32;
        m_cnt.0 += (c == 'M') as u32;
        m_cnt.1 += (c == 'M') as i32 * xdir;
        s_cnt.1 += (c == 'S') as i32 * ydir;
    }
    return s_cnt.0 > 1 && m_cnt.0 > 1 && (m_cnt.1 != 0 || s_cnt.1 != 0);
}

fn check_dirs(input: &Map, width: i32, height: i32, x: i32, y: i32) -> u32 {
    DIRS.iter()
        .filter(|&&(xdir, ydir)| {
            (1..TARGET.len()).all(|i| {
                let nx = x + xdir * i as i32;
                let ny = y + ydir * i as i32;
                nx >= 0
                    && nx < width
                    && ny >= 0
                    && ny < height
                    && input[ny as usize][nx as usize] == TARGET[i]
            })
        })
        .count() as u32
}
