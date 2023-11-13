use std::cmp::Ordering;
use std::str::Chars;

#[derive(Clone, Debug)]
enum Packet {
    Num(i32),
    List(Vec<Packet>),
}

impl Packet {
    fn unwrap_num(&self) -> i32 {
        match self {
            Packet::Num(x) => *x,
            _ => panic!("tried to unwrap a number while is a list"),
        }
    }
    fn unwrap_list(&self) -> &Vec<Packet> {
        match self {
            Packet::List(x) => x,
            _ => panic!("tried to unwrap a list while is a number"),
        }
    }
    fn is_num(&self) -> bool {
        match *self {
            Packet::Num(_) => true,
            Packet::List(_) => false,
        }
    }
    fn is_list(&self) -> bool {
        !self.is_num()
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        let res = test_left_and_right(self, other);
        if let Some(x) = res {
            if x {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        } else {
            Ordering::Greater
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        matches!(self.cmp(other), Ordering::Equal)
    }
}

impl Eq for Packet {}

fn test_left_and_right(left: &Packet, right: &Packet) -> Option<bool> {
    let l_list = left.unwrap_list();
    let mut l_iter = l_list.iter().peekable();
    let r_list = right.unwrap_list();
    let mut r_iter = r_list.iter().peekable();
    loop {
        if l_iter.peek().is_none() || r_iter.peek().is_none() {
            break;
        }
        let l = l_iter.next().unwrap();
        let r = r_iter.next().unwrap();

        if l.is_num() && r.is_num() {
            // both a number
            let lu = l.unwrap_num();
            let ru = r.unwrap_num();
            if lu < ru {
                return Some(true);
            } else if lu > ru {
                return Some(false);
            }
        } else if l.is_list() && r.is_list() {
            // both a list
            match test_left_and_right(l, r) {
                Some(true) => return Some(true),
                Some(false) => return Some(false),
                None => continue,
            }
        } else {
            // one of both is a list and the other a number
            let lu: Packet;
            let ru: Packet;
            if l.is_num() {
                // left is number, make list
                lu = Packet::List(vec![Packet::Num(l.unwrap_num())]);
                ru = r.clone();
            } else {
                // right is number, make list
                lu = l.clone();
                ru = Packet::List(vec![Packet::Num(r.unwrap_num())]);
            }
            match test_left_and_right(&lu, &ru) {
                Some(true) => return Some(true),
                Some(false) => return Some(false),
                None => continue,
            }
        }
    }
    if l_iter.peek().is_none() && r_iter.peek().is_some() {
        Some(true)
    } else if r_iter.peek().is_none() {
        Some(false)
    } else {
        None
    }
}

fn parse(input: Vec<String>) -> Vec<(Packet, Packet)> {
    fn parse_arrays(input: &mut Chars) -> Packet {
        let mut array = Vec::new();
        loop {
            let mut c = input.next().unwrap();
            if c == '[' {
                array.push(parse_arrays(input));
            } else if c.is_ascii_digit() {
                let mut num = vec![c];
                loop {
                    let x = input.next();
                    if x.is_none() {
                        break;
                    }
                    c = x.unwrap();
                    if !c.is_ascii_digit() {
                        break;
                    }
                    num.push(c);
                }
                array.push(Packet::Num(
                    num.into_iter().collect::<String>().parse::<i32>().unwrap(),
                ));
            }
            if c == ']' {
                break;
            }
        }
        Packet::List(array)
    }
    let mut packets = Vec::new();

    let mut packet = (None, None);
    for line in input {
        if line.is_empty() {
            packets.push((packet.0.unwrap(), packet.1.unwrap()));
            packet = (None, None);
            continue;
        }

        let mut char_iter = line.chars();
        char_iter.next(); // we do not care about the first and last character

        let val = Some(parse_arrays(&mut char_iter));
        if packet.0.is_none() {
            packet.0 = val;
        } else {
            packet.1 = val;
        }
    }

    if packet.0.is_some() && packet.1.is_some() {
        packets.push((packet.0.unwrap(), packet.1.unwrap()));
    }
    packets
}

pub fn run(input: Vec<String>) -> String {
    let parsed = parse(input);

    let mut res = vec![
        Packet::List(vec![Packet::List(vec![Packet::Num(2)])]),
        Packet::List(vec![Packet::List(vec![Packet::Num(6)])]),
    ];
    for (l, r) in parsed {
        res.push(l);
        res.push(r);
    }

    res.sort();

    res.iter()
        .enumerate()
        .filter_map(|(i, e)| {
            if e.is_list() {
                let a = e.unwrap_list();
                if a.len() == 1 && a[0].is_list() {
                    let c = a[0].unwrap_list();
                    if c.len() == 1 && c[0].is_num() {
                        let num = c[0].unwrap_num();
                        if num == 2 || num == 6 {
                            Some(i as i32 + 1)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        })
        .product::<i32>()
        .to_string()
}

#[test]
fn test() {
    let answer = "140".to_string();
    let input = vec![
        "[1,1,3,1,1]".to_string(),
        "[1,1,5,1,1]".to_string(),
        "".to_string(),
        "[[1],[2,3,4]]".to_string(),
        "[[1],4]".to_string(),
        "".to_string(),
        "[9]".to_string(),
        "[[8,7,6]]".to_string(),
        "".to_string(),
        "[[4,4],4,4]".to_string(),
        "[[4,4],4,4,4]".to_string(),
        "".to_string(),
        "[7,7,7,7]".to_string(),
        "[7,7,7]".to_string(),
        "".to_string(),
        "[]".to_string(),
        "[3]".to_string(),
        "".to_string(),
        "[[[]]]".to_string(),
        "[[]]".to_string(),
        "".to_string(),
        "[1,[2,[3,[4,[5,6,7]]]],8,9]".to_string(),
        "[1,[2,[3,[4,[5,6,0]]]],8,9]".to_string(),
        "".to_string(),
    ];
    assert_eq!(answer, run(input));
}
