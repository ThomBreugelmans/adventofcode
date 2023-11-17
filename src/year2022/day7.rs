use crate::tree;

fn parse(input: Vec<String>) -> Vec<(String, Vec<String>)> {
    let mut parsed = Vec::new();
    let mut command: Option<(String, Vec<String>)> = None;
    for line in input {
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

pub fn run(input: Vec<String>) -> String {
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
    let input = vec![
        "$ cd /".to_string(),
        "$ ls".to_string(),
        "dir a".to_string(),
        "14848514 b.txt".to_string(),
        "8504156 c.dat".to_string(),
        "dir d".to_string(),
        "$ cd a".to_string(),
        "$ ls".to_string(),
        "dir e".to_string(),
        "29116 f".to_string(),
        "2557 g".to_string(),
        "62596 h.lst".to_string(),
        "$ cd e".to_string(),
        "$ ls".to_string(),
        "584 i".to_string(),
        "$ cd ..".to_string(),
        "$ cd ..".to_string(),
        "$ cd d".to_string(),
        "$ ls".to_string(),
        "4060174 j".to_string(),
        "8033020 d.log".to_string(),
        "5626152 d.ext".to_string(),
        "7214296 k".to_string(),
    ];

    assert_eq!(answer, run(input));
}
