use std::collections::{HashMap, HashSet};

const INPUT: &str = "$ cd /
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

pub fn solve_first(input: String) {
    let tree = parse_input(input);
    println!("{:?}", tree);
    let mut dirs = Vec::new();
    for dir_path in tree.keys() {
        let total_size = compute_total(dir_path, &tree);
        dirs.push((dir_path, total_size));
    }
    dirs.sort_by_key(|(_, size)| *size);
    let sum: usize = dirs.into_iter().take(3).map(|(_, size)| size).sum();
    println!("sum: {sum}");
}

fn compute_total(path: &DirPath, tree: &HashMap<DirPath, Dir>) -> usize {
    let mut sum: usize = 0;
    let dir = tree.get(path).unwrap();
    sum += dir.files.iter().map(|(_, size)| *size).sum::<usize>();
    sum += dir
        .dirs
        .iter()
        .map(|dir_path| compute_total(dir_path, &tree))
        .sum::<usize>();
    sum
}

type DirPath = String;
type FilePath = String;
#[derive(Default, Debug)]
struct Dir {
    files: Vec<(FilePath, usize)>,
    dirs: Vec<DirPath>,
}

fn parse_input(input: String) -> HashMap<DirPath, Dir> {
    let mut current_dir = Vec::new();
    let mut file_system = HashMap::new();
    let mut lines = input.lines().peekable();
    while lines.peek().is_some() {
        let line = lines.next().unwrap();

        if line.starts_with("$ cd ") {
            // line is cd command
            cd(line, &mut current_dir);
        } else if line == "$ ls" {
            while lines.peek().is_some() && !lines.peek().unwrap().starts_with("$") {
                let next = lines.next().unwrap();

                let mut cd_clone = current_dir.clone();
                let directory_content = file_system
                    .entry(current_dir_string(&current_dir))
                    .or_insert(Dir::default());

                if next.starts_with("dir ") {
                    // is directory and should be added to map of directories
                    let dir_name = next.trim_start_matches("dir ");

                    // the new directory added to current dir
                    cd(dir_name, &mut cd_clone);
                    let new_dir_path = current_dir_string(&cd_clone);

                    directory_content.dirs.push(new_dir_path.clone());

                    // we also add the new directory to the file_system
                    file_system.entry(new_dir_path).or_insert(Dir::default());
                } else {
                    // the row is a file
                    let (num_str, file_name) = next.split_once(" ").unwrap();
                    cd(file_name, &mut cd_clone);
                    let file_path = current_dir_string(&cd_clone);

                    directory_content
                        .files
                        .push((file_path, num_str.parse().unwrap()));
                }
            }
        }
    }
    file_system
}

fn current_dir_string(current_dir: &Vec<&str>) -> String {
    let mut string = String::new();

    for s in current_dir.iter() {
        string.push_str(s);
        if *s != "/" {
            string.push('/');
        }
    }
    string
}

fn cd<'a>(line: &'a str, current_dir: &mut Vec<&'a str>) {
    let dir = line.trim_start_matches("$ cd ");
    if dir == ".." {
        current_dir.pop().unwrap();
    } else {
        current_dir.push(dir);
    }
}
