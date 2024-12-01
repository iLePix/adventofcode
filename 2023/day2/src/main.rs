use std::collections::HashMap;


fn task1(s: String) -> u32 {
    let mut cubes: HashMap<String, u32> = HashMap::new();
    let mut sum = 0;
    'outer: for (game_id, line) in s.lines().into_iter().enumerate() {
        let start = line.find(":").unwrap();
        let moves = line[(start+1)..].split(";");
        for m in moves {
            let pulls = m.trim().split(",");
            for pull in pulls {
                let a: Vec<&str> = pull.trim().split(" ").collect();
                let num = a[0].parse::<u32>().unwrap();
                let color = a[1].trim();
                //println!("c {} {}", color, num);
                if cubes.get(color).map_or(0, |v| *v) < num {
                    cubes.insert(color.to_string(), num);
                }
            }

            //println!("{:?} - {} - {} - {}", cubes, cubes.get("red").is_some_and(|v| *v < 13) , cubes.get("green").is_some_and(|v| *v < 14) , cubes.get("blue").is_some_and(|v| *v < 15));
            /*if cubes.get("red").map_or(0, |v| *v) > 12 || cubes.get("green").map_or(0, |v| *v) > 13|| cubes.get("blue").map_or(0, |v| *v) > 14 {
                //println!("imppossbile: {}", game_id + 1);
                println!("impossbile: {}", game_id + 1);
                cubes.clear();
                continue 'outer;
            }*/
        }

        println!("{}", cubes.get("red").map_or(0, |v| *v) * cubes.get("green").map_or(0, |v| *v) * cubes.get("blue").map_or(0, |v| *v));
        sum += cubes.get("red").map_or(1, |v| *v) * cubes.get("green").map_or(1, |v| *v) * cubes.get("blue").map_or(1, |v| *v);
        cubes.clear();
        //sum += game_id as u32 + 1;
    }
    sum
    //.map(|l| l.matches(char::is_numeric).collect())
    //.map(|cv: Vec<&str>| cv.first().unwrap().parse::<u32>().unwrap() * 10 + cv.last().unwrap().parse::<u32>().unwrap())
    //.sum::<u32>()
}


fn task2(s: String) -> u32 {
    s.lines();
    1
    //.map(|l| l.matches(char::is_numeric).collect())
    //.map(|cv: Vec<&str>| cv.first().unwrap().parse::<u32>().unwrap() * 10 + cv.last().unwrap().parse::<u32>().unwrap())
    //.sum::<u32>()
}

fn main() {
    let s = std::fs::read_to_string("i.txt").unwrap();
    println!("task1: {} | task2: {} ", task1(s.clone()), task2(s))
}