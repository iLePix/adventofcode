use std::collections::{HashSet, HashMap};



const CARDS: [char; 14] = ['J','2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];


fn main() {
    let s = std::fs::read_to_string("i.txt").unwrap();
    let mut sum = 0;
    let lines: Vec<&str> = s.lines().collect();
    let mut cards: Vec<(Hand, u64)> = lines.iter().map(|l| {
        let split: Vec<&str> = l.split_whitespace().collect();
        let chars: Vec<char> = split[0].chars().collect();
        let cards = Hand::from_chars(&chars);
        let bet = split[1].parse::<u64>().unwrap();
        (cards, bet)
    }).collect();

    //cards.sort_by_key(|v| v.0);
    cards.sort_by(|a, b| a.0.cmp(&b.0));

    let mut rank = 1;
    for (_, bet) in cards {
        //println!("{:?}, {:?}", hand.chars, hand.score);
        sum += bet * rank;
        rank += 1;
    }
    println!("{sum}");
}




#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq)]
pub struct Hand {
    score: CardsScore,
    chars: [char; 5]
}

impl Hand {
    pub fn from_chars(chars: &[char]) -> Self {
        let cs: [char; 5] = chars.try_into().unwrap();
        Self { score: CardsScore::from_chars(chars), chars: cs }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.score > other.score {
            return std::cmp::Ordering::Greater;
        }
        if self.score == other.score && char_score_is_higher(&self.chars, &other.chars) {
            return std::cmp::Ordering::Greater;
        }
        return std::cmp::Ordering::Less;
    }
}



#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
enum CardsScore {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn card_score(c: char) -> usize {
    if let Some((index, _)) = CARDS.iter().enumerate().find(|(_, &x)| x == c) {
        return index;
    } else {
        return 0;
    }
}

///checks if individual char score of a is higher than b
pub fn char_score_is_higher(a: &[char], b: &[char]) -> bool {
    let iter = std::iter::zip(a, b);
    for (a, b) in iter {
        if card_score(*a) == card_score(*b) { continue; }
        return card_score(*a) > card_score(*b);
    }
    return true;
}

impl CardsScore {
    pub fn from_chars(chars: &[char]) -> Self {
        let mut set: HashMap<char, u32> = HashMap::new();
        for c in chars {
            if !set.contains_key(c) {
                set.insert(*c, 1);
            } else {
                *set.get_mut(c).unwrap() += 1;
            }
        }
        let mut has_pair = false;
        let mut has_tripple = false;
        let mut num_of_js = set.get(&'J').map_or(0, |v| *v);
        let mut sorted_chars: Vec<(char, u32)> = set.into_iter().collect();
        sorted_chars.sort_by(|a, b| b.1.cmp(&a.1));
        if num_of_js == 5 { return Self::FiveOfAKind }
        for (c, count) in sorted_chars {
            if c == 'J' { continue; }
            match count + num_of_js {
                5 => return Self::FiveOfAKind,
                4 => return Self::FourOfAKind,
                3 => {
                    num_of_js = 0;
                    if has_pair { return Self::FullHouse }
                    has_tripple = true;
                }
                2 => {
                    num_of_js = 0;
                    if has_tripple { return Self::FullHouse }
                    if has_pair { return Self::TwoPair }
                    has_pair = true;
                }
                _ => panic!("ERROR")
            }
        }
        if has_tripple { return Self::ThreeOfAKind }
        if has_pair { return Self::OnePair }

        Self::HighCard
    }
}



fn parse_nums_option(s: &str) -> Vec<u64> {
    s.split(":").filter_map(|i| i.parse::<u64>().ok()).collect()
}


fn parse_nums(s: &str) -> Vec<u64> {
    s.split_ascii_whitespace().map(|i| i.parse::<u64>().unwrap()).collect()
}
