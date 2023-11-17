use std::collections::HashSet;

pub fn run(input: &str) -> String {
    let string = input.chars().collect::<Vec<char>>();

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
    let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
    assert_eq!(answer, run(input));
}
