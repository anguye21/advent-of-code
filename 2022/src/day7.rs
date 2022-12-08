use std::{str::FromStr, error::Error, fmt, rc::Rc, cell::RefCell, collections::HashMap};

#[derive(Debug)]
struct Folder {
    name: String,
    files: Vec<usize>,
    subfolders: Vec<Rc<RefCell<Folder>>>,
    parent: Option<Rc<RefCell<Folder>>>,
}

enum Command {
    Cd(String),
    Ls,
}

#[derive(Debug)]
struct ParseCommandError;

impl Error for ParseCommandError {}

impl fmt::Display for ParseCommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "cannot parse command")
    }
}

impl FromStr for Command {
    type Err = ParseCommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().nth(0) != Some('$') {
            return Err(ParseCommandError);
        }

        let command = s.split(" ").collect::<Vec<&str>>();

        return match command[1] {
            "cd" => {
                Ok(Command::Cd(command[2].to_string()))
            },
            "ls" => {
                Ok(Command::Ls)
            },
            _ => Err(ParseCommandError),
        }

    }
}

fn parse_input(input: &str) -> Rc<RefCell<Folder>> {
    let root = Rc::from(RefCell::from(Folder {
        name: String::from("/"),
        files: Vec::new(),
        subfolders: Vec::new(),
        parent: None,
    }));

    let mut curr_dir = Rc::clone(&root);


    for line in input.lines().skip(1) {
        if line.chars().nth(0) == Some('$') {
            let command = line.parse::<Command>().unwrap();

            if let Command::Cd(path) = command {
                let pwd = Rc::clone(&curr_dir);

                if path == ".." {
                    curr_dir = Rc::clone(&pwd.borrow_mut().parent.as_mut().unwrap());
                } else {
                    let name = curr_dir.borrow().name.to_string();

                    curr_dir = Rc::clone(pwd
                        .borrow_mut()
                        .subfolders
                        .iter()
                        .find(|&subfolder| {
                            let mut name = name.to_string();
                            name.push_str(&path);
                            subfolder.borrow().name == name
                        })
                        .unwrap());
                }
            }
        } else {
            let output = line.split_once(" ").unwrap();

            if output.0 == "dir" {
                let mut name = curr_dir.borrow().name.to_string();
                name.push_str(&output.1.to_string());

                curr_dir.borrow_mut().subfolders.push(Rc::from(RefCell::from(Folder {
                    name,
                    files: Vec::new(),
                    subfolders: Vec::new(),
                    parent: Some(Rc::clone(&curr_dir)),
                })));
            } else {
                curr_dir.borrow_mut().files.push(output.0.parse().unwrap());
            }

        }
    }

    return root;
}

fn find_sizes(root: &Rc<RefCell<Folder>>, res: &mut HashMap<String, usize>) -> usize {
    let root = root.borrow();

    if res.contains_key(&root.name) {
        return res[&root.name];
    }

    let mut size = root.files.iter().fold(0, |acc, x| acc + x);

    for folder in &root.subfolders {
        size += find_sizes(folder, res);
    }

    res.insert(root.name.to_string(), size);

    return size;
}

fn part1(input: &str) -> usize {
    let root = parse_input(input);
    let mut sizes: HashMap<String, usize> = HashMap::new();
    find_sizes(&root, &mut sizes);
    return sizes
        .iter()
        .map(|(_, size)| size)
        .filter(|&x| *x <= 100000)
        .sum();
}

fn part2(input: &str) -> usize {
    const TOTAL_SPACE: usize = 70000000;
    const NEEDED_EMPTY_SPACE: usize = 30000000;

    let root = parse_input(input);
    let mut sizes: HashMap<String, usize> = HashMap::new();
    let used_space = find_sizes(&root, &mut sizes);
    let space_left = TOTAL_SPACE - used_space;
    let space_to_remove = NEEDED_EMPTY_SPACE - space_left;

    return *sizes
        .iter()
        .map(|(_, size)| size)
        .filter(|&x| *x >= space_to_remove)
        .min()
        .unwrap();
}

fn main() {
    let input = include_str!("./input/day7.input");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "\
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

        assert_eq!(part1(input), 95437);
    }

    #[test]
    fn test_part2() {
        let input = "\
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

        assert_eq!(part2(input), 24933642);
    }
}
