use crate::utils::tree;
use macros::solution;

fn parse(input: &str) -> Vec<(String, Vec<String>)> {
    let mut parsed = Vec::new();
    let mut command: Option<(String, Vec<String>)> = None;
    for line in input.lines() {
        let mut chars = line.chars();
        if let Some(c) = chars.next() {
            if c == '$' {
                chars.next(); // next char is space
                if let Some(_command) = command {
                    parsed.push(_command);
                }
                command = Some((String::from_iter(chars), Vec::new()));
            } else {
                command
                    .as_mut()
                    .unwrap()
                    .1
                    .push(format!("{}{}", c, String::from_iter(chars)));
            }
        }
    }
    if let Some(_command) = command {
        parsed.push(_command);
    }
    parsed
}

#[solution(year = 2022, day = 7, part = 2)]
pub fn run(input: &str) -> String {
    let parsed = parse(input);

    struct NodeInfo {
        name: String,
        size: i32,
        is_dir: bool,
    }

    let mut tree = tree::Tree::new(NodeInfo {
        name: "/".to_string(),
        size: -1,
        is_dir: true,
    });

    let mut cur_dir = 0 as usize;

    for (command, output) in parsed {
        if command == "ls" {
            for f in output {
                if f.get(..3).unwrap() == "dir" {
                    // we have a directory
                    tree.add_to_id(
                        cur_dir,
                        NodeInfo {
                            name: f.get(4..).unwrap().to_string(),
                            size: 0,
                            is_dir: true,
                        },
                    );
                } else {
                    // we have a file
                    let a = f.split(' ').collect::<Vec<&str>>();
                    let node_name = a[1].to_string();
                    let node_size = a[0].parse::<i32>().unwrap();
                    tree.add_to_id(
                        cur_dir,
                        NodeInfo {
                            name: node_name,
                            size: node_size,
                            is_dir: false,
                        },
                    );
                    // update parents size recursively
                    let mut parent = Some(cur_dir);
                    while let Some(parent_id) = parent {
                        tree.get_mut_data_id(parent_id).size += node_size;
                        parent = tree.get_parent(parent_id);
                    }
                }
            }
        } else {
            let dir = command.split(' ').collect::<Vec<&str>>()[1];
            cur_dir = match dir {
                ".." => tree.get_parent(cur_dir).unwrap(),
                "/" => 0 as usize,
                d => tree
                    .find_child_with(cur_dir, |t| t.name == d && t.is_dir)
                    .unwrap(),
            };
        }
    }

    // diff minimum needed size to delete
    let diff = (tree.get_data_id(0).size + 30000000) - 70000000;

    let nodes_with_cond = tree.find_nodes_with(|t| t.size >= diff && t.is_dir);

    nodes_with_cond
        .iter()
        .map(|&i| tree.get_data_id(i).size)
        .min()
        .unwrap()
        .to_string()
}

#[test]
fn test() {
    let answer = "24933642".to_string();
    let input = "$ cd /
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

    assert_eq!(answer, run(input));
}
