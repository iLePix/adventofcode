use std::collections::{HashSet, HashMap};

fn main() {
    let s = std::fs::read_to_string("i.txt").unwrap();
    let mut sum_first = 0;
    let mut sum_last = 0;
    let lines: Vec<&str> = s.lines().collect();


    let mut map: HashMap<String, (String, String)> = HashMap::new();


    for l in lines {
        let nums = parse_nums_option(l);
        println!("{:?}", nums);
        let mut diffrencess: Vec<Vec<i64>> = vec![nums];
        loop {
            let diffrences: Vec<i64> = diffrencess.last().unwrap().windows(2).map(|a| a[1] - a[0]).collect();
            diffrencess.push(diffrences.clone());
            if diffrences.iter().filter(|v| **v == 0).map(|v| *v).collect::<Vec<i64>>().len() == diffrences.len() {
                break;
            }
        }

        let len = diffrencess.len();
        for i in (1..len).rev() {
            let increment = diffrencess[i].last().unwrap().clone();
            let last = diffrencess[i - 1].last().unwrap().clone();
            diffrencess[i - 1].push(last + increment);
        }

        for i in (1..len).rev() {
            let increment = diffrencess[i].first().unwrap().clone();
            let last = diffrencess[i - 1].first().unwrap().clone();
            diffrencess[i - 1].insert(0, last - increment);
        }



        sum_first += diffrencess.first().unwrap().first().unwrap();
        sum_last += diffrencess.first().unwrap().last().unwrap();
    }



    println!("{:?} {:?}", sum_last, sum_first);
}





fn parse_nums_option(s: &str) -> Vec<i64> {
    s.split(" ").filter_map(|i| i.parse::<i64>().ok()).collect()
}


fn parse_nums(s: &str) -> Vec<u64> {
    s.split_ascii_whitespace().map(|i| i.parse::<u64>().unwrap()).collect()
}
