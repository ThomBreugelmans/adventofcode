pub fn run(input: Vec<String>) -> String {
    let mut elves: Vec<i32> = vec![0];
    for cal in input {
        if cal.is_empty() {
            elves.push(0);
            continue;
        }
        let cal_i = cal.parse::<i32>().unwrap();
        if let Some(last) = elves.last_mut() {
            *last += cal_i;
        }
    }

    elves.sort();
    elves.reverse();
    (elves[0] + elves[1] + elves[2]).to_string()
}

#[test]
fn test() {
    let answer = "45000".to_string();
    let input = vec![
        "1000".to_string(),
        "2000".to_string(),
        "3000".to_string(),
        "".to_string(),
        "4000".to_string(),
        "".to_string(),
        "5000".to_string(),
        "6000".to_string(),
        "".to_string(),
        "7000".to_string(),
        "8000".to_string(),
        "9000".to_string(),
        "".to_string(),
        "10000".to_string(),
    ];
    assert_eq!(answer, run(input));
}
