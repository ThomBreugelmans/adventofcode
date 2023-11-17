pub fn run(input: &str) -> String {
    let mut elves: Vec<i32> = vec![0];
    for cal in input.lines() {
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
    let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
    assert_eq!(answer, run(input));
}
