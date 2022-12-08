use std::{io::{BufReader, BufRead}, fs::File, slice::Iter, collections::HashMap};

fn main() -> std::io::Result<()> {
    let path = "./input.txt";
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let lines = reader.lines().filter_map(|lr| lr.ok()).collect::<Vec<String>>();
    let (dirs, files) = parse_dir_tree(&mut lines.iter());

    //Part 1
    let s: u32 = (1..dirs.len()).map(|i| dir_size(i, &dirs, &files)).filter(|s| s <= &100_000).sum();
    println!("Part 1: {}", s);

    //Part 2
    let disk_size = 70_000_000;
    let update = 30_000_000;
    let used_space = dir_size(0, &dirs, &files);
    let space_req = used_space + update - disk_size;
    let binding = (1..dirs.len()).map(|i| dir_size(i, &dirs, &files)).filter(|s| *s >= space_req).collect::<Vec<u32>>();
    let min = binding.iter().min();
    println!("Part 2: {}", min.unwrap());



    Ok(())
}

fn dir_size(dir: usize, dirs: &Vec<Dir>, files: &Vec<Fil>) -> u32{
    let cur_dir = &dirs[dir];
    let mut size = cur_dir.files.iter().map(|f| files[*f.1].size).sum::<u32>();
    size += cur_dir.children.iter().map(|d| dir_size(*d.1, dirs, files)).sum::<u32>();
    size
}


fn parse_dir_tree(lines: &mut Iter<String>) -> (Vec<Dir>, Vec<Fil>) {
    let mut cur_dir = 0;
    let mut files: Vec<Fil> = Vec::new();
    let mut dirs = vec![Dir::new("/", None)];
    while let Some(line) =  lines.next() {
        if line.starts_with("$") {
            match &line[..4] {
                "$ cd" => {
                    match &line[4..] {
                        " .." => {
                            if let Some(p) = dirs[cur_dir].parent {
                                cur_dir = p;
                            } else {
                                panic!("parent doesnt exist: {}", dirs[cur_dir].name)
                            }
                        },
                        name => {
                            if let Some(dir) = dirs[cur_dir].children.get("name") {
                                cur_dir = *dir;
                            } else {
                                let i = dirs.len();
                                dirs[cur_dir].children.insert(name.to_string(), i);
                                dirs.push(Dir::new(name.trim(), Some(cur_dir)));
                                cur_dir = i;
                            }
                        }
                    }
                },
                "$ ls" => {
                }
                c => {panic!("Undefined command: {}", c)}
            }
        } else {
            let elems: Vec<&str> = line.split(" ").collect();
            let name = elems[1];
            match elems[0] {
                "dir" => {
                    let i = dirs.len();
                    dirs[cur_dir].children.insert(name.to_string(), i);
                    dirs.push(Dir::new(name, Some(cur_dir)));
                }
                size => {
                    let i = files.len();
                    let f = Fil {size: size.parse::<u32>().unwrap(), name: name.to_string()};
                    dirs[cur_dir].files.insert(name.to_string(), i);
                    files.push(f);
                }
            }
        }
    }
    (dirs, files)
}


struct Dir {
    pub parent: Option<usize>,
    pub name: String,
    pub children: HashMap<String, usize>,
    pub files: HashMap<String, usize>,
}

struct Fil {
    name: String,
    pub size: u32
}

impl Dir {
    pub fn new(name: &str, parent: Option<usize>) -> Self {
        Self { name: name.to_string(), children: HashMap::new(), files: HashMap::new(), parent }
    }
}
