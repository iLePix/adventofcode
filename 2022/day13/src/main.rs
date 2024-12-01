

use std::{fs::File, io::{BufReader, BufRead}, slice::Iter, collections::{HashSet, HashMap}, iter, cmp::max};


fn main() -> std::io::Result<()> {
    let path = "./input.txt";
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let lines = reader.lines().filter_map(|lr| lr.ok()).collect::<Vec<String>>();
    let mut packets = parse_packets(lines);

    println!("Part 1: {}", part1(&packets));
    println!("Part 2: {}", part2(&mut packets));

    Ok(())
}

fn part1(packets: &Vec<Packet>) -> usize {
    let mut sum = 0;
    for (i, packet) in packets.iter().enumerate() {
        if packet.right > packet.left {
            sum += i + 1;
        }
    }
    sum
}

fn part2(packets: &mut Vec<Packet>) -> usize {
    use PacketContent::*;
    let divider = Packet {left: List(vec![List(vec![Value(2)])]), right: List(vec![List(vec![Value(6)])]) };
    packets.push(divider.clone());
    let mut sorted = packets.iter().flat_map(|p| vec![p.left.clone(), p.right.clone()]).collect::<Vec<PacketContent>>();
    sorted.sort();
    let mut product = 1;
    for (i, pc) in sorted.iter().enumerate() {
        if *pc == divider.left || *pc == divider.right {
            product *= i + 1;
        }
    }
    product
}


fn parse_packets(lines: Vec<String>) -> Vec<Packet> {
    lines.iter()
        .filter(|l| !l.is_empty())
        .collect::<Vec<&String>>()
        .chunks(2)
        .map(|pair| {
            Packet { 
                left: build_packet_content(pair[0]).unwrap(),
                right: build_packet_content(pair[1]).unwrap()
            }
        })
        .collect::<Vec<Packet>>()
}

fn build_packet_content(packet_str: &String) -> Option<PacketContent> {
    if packet_str.starts_with("[") {
        let mut l_brackets = 0;
        let mut r_brackets = 0;
        let mut start = 1;
        let mut end = 0;
        let mut content = vec![];
        for l in packet_str.chars() {
            match l {
                '[' => l_brackets += 1,
                ']' => r_brackets += 1,
                ',' => {
                    if l_brackets - r_brackets == 1 {
                        let p = build_packet_content(&packet_str[start..end].to_string());
                        content.push(p);
                        start = end + 1;
                    }
                }
                _ => {}
            };
            end += 1;
        }
        content.push(build_packet_content(&packet_str[start..end - 1].to_string()));
        return Some(PacketContent::List(content.into_iter().filter_map(|x| x).collect()))
    }
    if packet_str.len() > 0 {
        return Some(PacketContent::Value(packet_str.parse::<i32>().unwrap()))
    }
    None
}

#[derive(Debug, Clone)]
struct Packet {
    pub left: PacketContent,
    pub right: PacketContent
}

#[derive(Debug, Clone, PartialEq, Eq, Ord)]
enum PacketContent {
    List(Vec<PacketContent>),
    Value(i32)
}

impl PartialOrd for PacketContent {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use std::cmp::Ordering::*;
        use PacketContent::*;
        match (self, other) {
            (List(s), List(o)) => {
                for i in 0..max(s.len(), o.len()) {
                    let s = s.get(i);
                    let o = o.get(i);
                    match (s, o) {
                        (None, None) => return Some(Equal),
                        (None, Some(_)) => return Some(Less),
                        (Some(_), None) => return Some(Greater),
                        (Some(s), Some(o)) => {
                            let res = s.partial_cmp(o);
                            if res != Some(Equal) {
                                return res
                            }
                        },
                    }
                }
            },
            (List(s), Value(o)) => return self.partial_cmp(&List(vec![other.clone()])),
            (Value(_), List(o)) => return List(vec![self.clone()]).partial_cmp(other),
            (Value(s), Value(o)) => {
                if s == o {
                    return Some(Equal);
                } else if s > o {
                    return Some(Greater)
                } else {
                    return Some(Less)
                }
            },
        }
        Some(Equal)
    }
}