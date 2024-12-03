fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("./input.txt")?;
    let len = input.len();
    let mut i = 0;
    let mut insts = vec![];
    while i < len {
        if i + 4 < len && &input[i..i + 4] == "do()" {
            i += 4;
            insts.push(Instruction::Do);
            continue;
        }
        if i + 7 < len && &input[i..i + 7] == "don't()" {
            i += 7;
            insts.push(Instruction::Dont);
            continue;
        }

        if i + 4 < len && &input[i..i + 4] == "mul(" {
            i += 4;
            let start = i;
            while i < len
                && &input[i..i + 1] != ","
                && input[i..i + 1].chars().nth(0).unwrap().is_digit(10)
            {
                i += 1;
            }
            if &input[i..i + 1] != "," {
                continue;
            }
            if let Ok(first_number) = input[start..i].parse::<i32>() {
                i += 1;
                let start = i;
                while i < len
                    && &input[i..i + 1] != ")"
                    && input[i..i + 1].chars().nth(0).unwrap().is_digit(10)
                {
                    i += 1;
                }
                if &input[i..i + 1] != ")" {
                    continue;
                }
                if let Ok(second_number) = input[start..i].parse::<i32>() {
                    insts.push(Instruction::Mul(first_number, second_number));
                }
                i += 1;
            }
        } else {
            i += 1
        }
    }
    dbg!(task1(&insts));
    dbg!(task2(&insts));

    Ok(())
}

#[derive(Debug)]
enum Instruction {
    Do,
    Dont,
    Mul(i32, i32),
}

fn task1(insts: &[Instruction]) -> i32 {
    insts
        .iter()
        .map(|i| match i {
            Instruction::Mul(a, b) => a * b,
            _ => 0,
        })
        .sum()
}

fn task2(insts: &[Instruction]) -> i32 {
    let mut sum = 0;
    let mut work = true;
    for inst in insts {
        match inst {
            Instruction::Do => work = true,
            Instruction::Dont => work = false,
            Instruction::Mul(a, b) if work => sum += a * b,
            _ => {}
        }
    }
    sum
}
