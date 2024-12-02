#![feature(iter_map_windows)]

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("./input.txt")?;
    let levels: Vec<Vec<i32>> = input
        .lines()
        .map(|l| l.split(' ').map(str::parse).collect())
        .collect::<Result<_, _>>()?;

    let task1: usize = valid_levels(&levels);

    //task2 brute force
    let mut task2 = 0;
    for (i, lvls) in levels.clone().iter().enumerate() {
        let mut new_levels = vec![];
        for i in 0..lvls.len() {
            new_levels.push(lvls.clone());
            new_levels[i].remove(i);
        }
        if valid_levels(&new_levels) > 0 {
            task2 += 1
        }
    }
    dbg!(task1);
    dbg!(task2);
    Ok(())
}

fn valid_levels(levels: &[Vec<i32>]) -> usize {
    levels
        .iter()
        .filter(|lvls| {
            lvls.iter()
                .map_windows(|[a, b]| *a - *b)
                .map_windows(|[da, db]| valid_diffs(*da, *db))
                .all(|c| c)
        })
        .count()
}

fn valid_diffs(da: i32, db: i32) -> bool {
    da.abs() < 4 && db.abs() < 4 && db * da > 0
}
