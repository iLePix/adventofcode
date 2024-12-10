fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("./input.txt")?;
    let mut fs = vec![];
    for (id, sizes) in input.as_bytes().chunks(2).enumerate() {
        for _ in 0..(sizes[0] - b'0') {
            fs.push(Some(id as u64));
        }
        if sizes.get(1).is_none_or(|v| *v < b'0') {
            break;
        }
        for _ in 0..(sizes[1] - b'0') {
            fs.push(None);
        }
    }
    let mut part1_fs = fs.clone();
    reformat_part1(&mut part1_fs);
    dbg!(checksum(&part1_fs));
    let mut part2_fs = fs;
    reformat_part2(&mut part2_fs);
    dbg!(checksum(&part2_fs));
    Ok(())
}

fn reformat_part1(fs: &mut [Option<u64>]) {
    'f: for i in (0..fs.len()).rev() {
        if let Some(id) = fs[i] {
            let mut j = 0;
            while j < i {
                if fs[j].is_none() {
                    fs[j] = Some(id);
                    fs[i] = None;
                    continue 'f;
                }
                j += 1;
            }
        }
    }
}

fn reformat_part2(fs: &mut Vec<Option<u64>>) {
    let mut block_id = None;
    let mut end = fs.len();
    for i in (0..fs.len()).rev() {
        match (block_id, fs[i]) {
            (None, None) => {
                continue;
            }
            (None, Some(id)) => {
                block_id = Some(id);
                end = i + 1;
                continue;
            }
            (Some(_), None) => {}
            (Some(b_id), Some(fs_id)) => {
                if b_id == fs_id {
                    continue;
                }
                block_id = Some(fs_id);
            }
        }
        let start = i + 1;
        let block_len = end - start;
        let mut free_space_len = 0;
        for j in 0..=i {
            if fs[j].is_none() {
                free_space_len += 1;
            } else {
                free_space_len = 0;
            }
            if free_space_len == block_len {
                fs.splice((j - free_space_len + 1)..=j, fs[start..end].to_vec());
                fs[start..end].fill(None);
                break;
            }
        }
        end = start;
    }
}

fn checksum(fs: &[Option<u64>]) -> u64 {
    fs.iter()
        .enumerate()
        .filter_map(|(i, id)| id.map(|id| i as u64 * id))
        .sum()
}
