use std::collections::HashMap;



fn main() {
    let s = std::fs::read_to_string("i.txt").unwrap();
    let mut sum = 0;
    let mut copies: HashMap<u32, u32> = HashMap::new();
    copies.insert(1, 1);
    for i in 2..213 {//213
        copies.insert(i, 1);
    }
    for (ci, c) in s.lines().enumerate() {
        let a: Vec<&str> = c.split(":").collect();
        let nums: Vec<&str> = a[1].split("|").collect();
        let nyh = parse_nums(nums[0]);
        let wn = parse_nums(nums[1]);
        let mut card_score = 0;
        let mut score_num = 1;
        let copiess = copies.get(&(ci as u32 + 1)).map_or(0, |v| *v);
        println!("copies of {} {copiess}", ci + 1);
        for n in nyh.clone() {
            if wn.contains(&n) {
                if card_score == 0 {
                    card_score = 1;
                } else {
                    card_score *= 2;
                }
                if let Some(cgg) = copies.get_mut(&(ci as u32 + 1 + score_num)) {
                    //println!("higher score of {} in card {}", ci as u32 + 1 + score_num, ci + 1);
                    *cgg += copiess;
                }
                score_num += 1;
            }
        }
        //println!("{:?}", copies);
        sum += card_score;
    }

    println!("result: {:?}", copies.iter().map(|(k, v)| if *k < 213 { *v } else {0}).sum::<u32>())
}


fn parse_nums(s: &str) -> Vec<u32> {
    s.split_ascii_whitespace().map(|i| i.parse::<u32>().unwrap()).collect()
}
