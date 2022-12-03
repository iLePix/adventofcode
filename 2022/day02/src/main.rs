use std::{fs::File, io::{BufReader, BufRead}};

fn main() -> std::io::Result<()> {
    let path = "./input.txt";
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().filter_map(|lr| lr.ok()).collect();
    
    let total1_score: u32 = lines.iter().map(|l| {
        let mut chars = l.chars();
        let opponent = chars.next().unwrap();
        let me =  chars.skip(1).next().unwrap();
        let score = game1_score(Choice::from_opponent(opponent), Choice::from_me(me)) as u32;
        score
    }).sum();

    let total2_score: u32 = lines.iter().map(|l| {
        let mut chars = l.chars();
        let opponent = chars.next().unwrap();
        let res =  chars.skip(1).next().unwrap();
        let score = game2_score(Choice::from_opponent(opponent), GameResult::from_c(res)) as u32;
        score
    }).sum();

    println!("Resulting score of the first understanding");
    println!("{}", total1_score);


    println!("Resulting score after the elf's correction");
    println!("{}", total2_score);

    Ok(())
}

fn game2_score(opponent: Choice, my_res: GameResult) -> u8 {

    use GameResult::*;
    use Choice::*;
    let me = match (opponent, my_res) {
        (Rock, Win) => Paper,
        (Rock, Loss) => Scissors,
        (Paper, Win) => Scissors,
        (Paper, Loss) => Rock,
        (Scissors, Win) => Rock,
        (Scissors, Loss) => Paper,
        _ => opponent
    };
    return game1_score(opponent, me)
}


fn game1_score(opponent: Choice, me: Choice) -> u8 {

    use GameResult::*;
    let game_res = match (opponent, me) {
        (Choice::Rock, Choice::Paper) => Win,
        (Choice::Rock, Choice::Scissors) => Loss,
        (Choice::Paper, Choice::Rock) => Loss,
        (Choice::Paper, Choice::Scissors) => Win,
        (Choice::Scissors, Choice::Rock) => Win,
        (Choice::Scissors, Choice::Paper) => Loss,
        _ => Draw
    };
    return game_res.to_score() + me as u8
}

#[repr(u8)]
#[derive(PartialEq, Clone, Copy)]
enum Choice {
    Rock = 1, 
    Paper = 2,
    Scissors = 3
}

impl Choice {
    pub fn from_opponent(c: char) -> Self {
        use Choice::*;
        match c {
            'A' => Rock,
            'B' => Paper,
            'C' => Scissors,
            _ => panic!("Unknown hand encountered")
        }
    }

    pub fn from_me(c: char) -> Self {
        use Choice::*;
        match c {
            'X' => Rock,
            'Y' => Paper,
            'Z' => Scissors,
            _ => panic!("Unknown hand encountered")
        }
    }

}




enum GameResult {
    Win, 
    Draw,
    Loss
}

impl GameResult {
    pub fn to_score(self) -> u8 {
        match self {
            GameResult::Win => 6,
            GameResult::Draw => 3,
            GameResult::Loss => 0,
        }
    }

    pub fn from_c(c: char) -> Self {
        use GameResult::*;
        match c {
            'X' => Loss,
            'Y' => Draw,
            'Z' => Win,
            _ => panic!("Unknown hand encountered")
        }
    }
}