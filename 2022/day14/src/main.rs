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