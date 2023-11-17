use std::collections::HashSet;

pub fn run(input: Vec<String>) -> String {
    let string = input
        .iter()
        .rfold("".to_string(), |_, b| b.clone())
        .chars()
        .collect::<Vec<char>>();

    for i in 0..string.len() - 14 {
        let mut set = HashSet::new();
        for j in i..i + 14 {
            set.insert(string[j]);
        }
        if set.len() == 14 {
            return (i as i32 + 14).to_string();
        }
    }

    (string.len() as i32 + 1).to_string()
}

#[test]
fn test() {
    let answer = "26".to_string();
    let input = vec!["zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string()];
    assert_eq!(answer, run(input));
}
