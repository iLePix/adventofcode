
fn sum(s: String) -> u32 {
    s.lines()
    .map(|l| l.matches(char::is_numeric).collect())
    .map(|cv: Vec<&str>| cv.first().unwrap().parse::<u32>().unwrap() * 10 + cv.last().unwrap().parse::<u32>().unwrap())
    .sum::<u32>()
}

fn str_sum(s: String) -> u32 {
    sum(["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"].iter().enumerate().fold(s, |acc, (i, n)| { acc.replace(n, &format!("{n}{}{n}", i + 1))}))
}

fn main() {
    let s = std::fs::read_to_string("i.txt").unwrap();
    println!("task1: {} | task2: {} ", sum(s.clone()), str_sum(s))
}