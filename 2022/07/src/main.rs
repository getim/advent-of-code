use std::collections::HashMap;
use std::fs;
use std::io::BufReader;
use std::io::prelude::*;


#[derive(Debug)]
enum Command {
    Cd(String),
    Ls(Vec<Vec<String>>),
}

#[derive(Debug)]
struct Filesystem {
    files: Vec<File>,
    dirs: Vec<Dir>,
}

#[derive(Debug)]
struct File {
    size: usize,
}

#[derive(Debug)]
struct Dir {
    dirs: HashMap<String, usize>,
    files: HashMap<String, usize>,
    up: Option<usize>,
}

impl Dir {
    fn get_recursive_size(&self, filesystem: &Filesystem) -> usize {
        self.get_size(filesystem) +
            self.dirs.iter().map(|(_, d)| filesystem.dirs[*d].get_recursive_size(filesystem)).sum::<usize>()
    }

    fn get_size(&self, filesystem: &Filesystem) -> usize {
        self.files.iter().map(|(_, f)| filesystem.files[*f].size).sum()
    }
}


fn main() {
    let file = fs::File::open("input").unwrap();
    let mut lines = BufReader::new(file).lines();

    let commands = read_commands(&mut lines);
    let filesystem = create_filesystem(&commands);

    println!("Result 1: {}", solve1(&filesystem));
    println!("Result 2: {}", solve2(&filesystem));
}

fn solve1(filesystem: &Filesystem) -> usize {
    let mut result = 0;
    for dir in &filesystem.dirs {
        let size = dir.get_recursive_size(filesystem);
        if size <= 100000 {
            result += size;
        }
    }
    result
}

fn solve2(filesystem: &Filesystem) -> usize {
    let total_size = 70000000;
    let update_size = 30000000;
    let occupied_size = filesystem.dirs[0].get_recursive_size(filesystem);
    let to_be_freed_size = occupied_size + update_size - total_size;

    let mut best_size_to_remove = total_size + 1;
    for dir in &filesystem.dirs {
        let size = dir.get_recursive_size(filesystem);
        if size > to_be_freed_size && size < best_size_to_remove {
            best_size_to_remove = size;
        }
    }
    best_size_to_remove
}

fn read_commands<I>(lines: &mut I) -> Vec<Command>
where
    I: Iterator<Item=Result<String, std::io::Error>>
{
    let mut lines = lines.peekable();
    let mut commands = vec![];
    while let Some(line) = lines.next() {
        let line = line.unwrap();
        let words: Vec<&str> = line.split_whitespace().collect();
        if words.len() == 3 {
            commands.push(Command::Cd(words[2].to_owned()))
        } else {
            let mut entries: Vec<Vec<String>> = vec![];
            loop {
                let line_peek = lines.peek();
                match line_peek {
                    None => break,
                    Some(x) => {
                        match x {
                            Ok(y) => entries.push(
                                if y.starts_with('$') {
                                    break;
                                } else {
                                    lines.next().unwrap().unwrap().split_whitespace()
                                    .map(|s| s.to_owned()).collect()
                                }
                            ),
                            Err(_) => panic!(),
                        };
                    },
                }
            }
            commands.push(Command::Ls(entries));
        }
    }
    commands
}

fn create_filesystem(commands: &[Command]) -> Filesystem {
    let mut filesystem = Filesystem {
        dirs: vec![Dir {
            dirs: HashMap::new(),
            files: HashMap::new(),
            up: None,
        }],
        files: vec![],
    };

    let mut current_dir: usize = 0;

    for command in commands {
        match command {
            Command::Ls(entries) => {
                for entry in entries {
                    if entry[0] == "dir" {
                        let dir = Dir {
                            dirs: HashMap::new(),
                            files: HashMap::new(),
                            up: Some(current_dir),
                        };
                        filesystem.dirs.push(dir);
                        let new_dir_index = filesystem.dirs.len() - 1;
                        filesystem.dirs[current_dir].dirs.insert(
                            entry[1].to_owned(),
                            new_dir_index,
                        );
                    } else {
                        let file = File {
                            size: entry[0].parse::<usize>().unwrap(),
                        };
                        filesystem.files.push(file);
                        let new_file_index = filesystem.files.len() - 1;
                        filesystem.dirs[current_dir].files.insert(
                            entry[1].to_owned(),
                            new_file_index,
                        );
                    }
                }
            }
            Command::Cd(d) => {
                current_dir = if d == "/" {
                    0
                } else if d == ".." {
                    filesystem.dirs[current_dir].up.unwrap()
                } else {
                    filesystem.dirs[current_dir].dirs[d]
                }
            }
        }
    }

    filesystem
}
