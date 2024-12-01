use std::collections::{HashSet, HashMap};

fn main() {
    let s = std::fs::read_to_string("i.txt").unwrap();
    let mut sum = 0;
    let lines: Vec<&str> = s.lines().collect();

    let instructions: Vec<char> = lines[0].chars().collect();

    let mut map: HashMap<String, (String, String)> = HashMap::new();

    let mut currents = vec![];

    for l in lines {
        if l.len() < 1 {
            continue;
        }
        let splits: Vec<&str> = l.split("=").collect();
        if splits.len() == 1 {
            continue;
        }
        let options: Vec<&str> = splits[1].split(",").collect();
        let ident = splits[0].trim().to_string();
        if ident.clone().chars().last().unwrap() == 'A' {
            currents.push(ident.clone());
        }
        let left = options[0].to_string().replace("(", "").to_string();
        let right = options[1].to_string().replace(")", "").to_string();
        map.insert(ident, (left.to_owned().trim().to_owned(), right.to_owned().trim().to_owned()));
    }

    let mut instructions_index = 0;
    let mut first_hits = vec![];
    let mut hits: HashSet<usize> = HashSet::new();
    'outer: loop {
        sum += 1;
        if instructions_index == instructions.len() { instructions_index = 0; }
        for (i, current) in currents.iter_mut().enumerate() {
            if hits.contains(&i) {
                continue;
            }
            let (left, right) = map.get(&*current).unwrap();
            if instructions[instructions_index] == 'L' {
                *current = left.clone();
            } else {
                *current = right.clone();
            }
            if current.chars().last().unwrap() == 'Z' {
                first_hits.push(sum);
                hits.insert(i);
            }
        }

        if first_hits.len() == currents.len() {
            break;
        }
        instructions_index += 1;
    }

    println!("{:?}", first_hits);

    let res = first_hits.iter().map(|v| *v as u64).fold(1, |a, b| lcm(a, b));


    println!("{:?}", res);
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
