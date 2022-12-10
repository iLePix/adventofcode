use std::{io::{BufReader, BufRead}, fs::File, slice::Iter, vec};

fn main() -> std::io::Result<()> {
    let path = "./input.txt";
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let lines = reader.lines().filter_map(|lr| lr.ok()).collect::<Vec<String>>();
    let mut trees = parse_trees(lines.iter());
    let trees = trees.iter_mut().map(|tree_row| {
        let mut b = visible(tree_row);
        b.reverse();
        visible(&mut b)
    }).collect::<Vec<Vec<(u8, bool)>>>();

    let mut n_trees: Vec<Vec<(u8, bool)>> = vec![];
    for i in 0..trees[0].len() {
        let mut column = vec![];
        for row in &trees {
            let tree = row[i];
            column.push(tree)
        }
        n_trees.push(column);
    }

    let c = n_trees.iter_mut().map(|tree_row| {
        let mut b = visible(tree_row);
        b.reverse();
        visible(&mut b)
    }).collect::<Vec<Vec<(u8, bool)>>>();

    println!("Part 1: {}", c.iter().map(|column| column.iter().filter(|t| t.1).count() as u32).sum::<u32>());

    let mut best = 0;
    let size = (c.len(), c[0].len());
    for x in 0..size.0 {
        for y in 0..size.1 {
            let mut cur = 1;
            let c_h = c[x][y].0;
            let cur_pos = (x,y);
            let dirs = [
                (1,0),
                (-1,0),
                (0,1),
                (0,-1),
            ];

            let move_in_dir = |dir: (i32, i32)| -> bool {
                let new_pos = (cur_pos.0 as i32 + dir.0, cur_pos.1 as i32 + dir.1);
                if new_pos.0 >= 0 && new_pos.0 < size.0 as i32 - 1 && new_pos.1 >= 0 && new_pos.1 < size.1 as i32 - 1 {
                    return c_h > c[new_pos.0 as usize ][new_pos.1 as usize].0;
                }
                return false;
            };

            for dir in dirs {
                let mut dir_score = 0;
                let mut i = 1;
                let mut new_dir = dir;
                while move_in_dir(new_dir) {
                    i += 1;
                    dir_score += 1;
                    new_dir = (dir.0 * i, dir.1 * i)
                }
                let new_pos = (cur_pos.0 as i32 + new_dir.0, cur_pos.1 as i32 + new_dir.1);
                if new_pos.0 >= 0 && new_pos.0 < size.0 as i32&& new_pos.1 >= 0 && new_pos.1 < size.1 as i32  {
                    dir_score += 1;
                }
                cur *= dir_score;
            }
            if cur > best {
                best = cur;
            }
        }
    }

    println!("Part 2: {}", best);




    fn visible(trees: &mut Vec<(u8, bool)>) -> Vec<(u8, bool)> {
        let mut cur_height: i8 = -1;

        let mut t_c = |tree: (u8, bool)| -> (u8, bool) {
            let mut t = tree.clone();
            if t.0 as i8 > cur_height {
                cur_height = tree.0 as i8;
                t.1 = true;
            }
            t
        };

        trees.iter_mut().map(|tree| {
            t_c(*tree)
        }).collect::<Vec<(u8, bool)>>()
    }

    Ok(())
}

fn parse_trees(lines: Iter<String>) -> Vec<Vec<(u8, bool)>> {
    //first: y, x;
    let mut trees: Vec<Vec<(u8, bool)>> = vec![vec![]; lines.len()];
    for (y, line) in lines.enumerate() {
        for (x, tree_height) in line.chars().enumerate() {
            let h = tree_height as u8 - 48;
            trees[y].push((h, false))
        }    
    }
    trees
}