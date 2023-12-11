/// BONUS: a very 'beautiful' oneliner in python (the last parameter of the lambda is 1 for part 1 and 999_999 for part 2):
/// print((lambda f,e: (lambda gs,exs,eys: sum([sum([abs(g[0]-a[0])+abs(g[1]-a[1])+(len([u for u in exs[min(g[0],a[0]):max(g[0],a[0])] if u==1])+len([v for v in eys[min(g[1],a[1]):max(g[1],a[1])] if v==1]))*e for a in gs[i+1:]]) for i,g in enumerate(gs)]))([p for p,c in sum(f,[]) if c=='#'],[1 if len(x) == 0 else 0 for x in [[d for d in c if d[1]=='#'] for c in list(map(list, zip(*f)))]],[1 if len(y)==0 else 0 for y in [[d for d in c if d[1]=='#'] for c in f]]))([[((x,y),c) for x,c in enumerate(r)] for y,r in [(l[0],l[1].split('\n')[0]) for l in enumerate(open('input.txt').readlines())]],1))

fn dist(a: &(usize, usize), b: &(usize, usize)) -> u64 {
    (a.0.max(b.0) - a.0.min(b.0) + a.1.max(b.1) - a.1.min(b.1)) as u64
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn compute(input: &str, expansion: u64) -> u64 {
    let grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let expansion_y: Vec<bool> = grid.iter().map(|r| r.iter().all(|c| *c == '.')).collect();
    let expansion_x: Vec<bool> = transpose(grid.clone())
        .iter()
        .map(|col| col.iter().all(|c| *c == '.'))
        .collect();

    let galaxies: Vec<(usize, usize)> = grid
        .into_iter()
        .enumerate()
        .flat_map(|(y, r)| {
            r.into_iter()
                .enumerate()
                .filter_map(|(x, c)| (c == '#').then_some((x, y)))
                .collect::<Vec<_>>()
        })
        .collect();

    galaxies
        .iter()
        .enumerate()
        .flat_map(|(i, g)| {
            galaxies.iter().skip(i).map(|g2| {
                dist(g, g2)
                    + (expansion_x
                        .iter()
                        .skip(g.0.min(g2.0))
                        .take(g.0.max(g2.0) - g.0.min(g2.0))
                        .filter(|a| **a)
                        .count() as u64
                        + expansion_y
                            .iter()
                            .skip(g.1.min(g2.1))
                            .take(g.1.max(g2.1) - g.1.min(g2.1))
                            .filter(|a| **a)
                            .count() as u64)
                        * (expansion - 1)
            })
        })
        .sum()
}

pub fn run(input: &str) -> String {
    format!("Part 1: {}\nPart 2: {}", run_part1(input), run_part2(input))
}

fn run_part1(input: &str) -> String {
    compute(input, 2).to_string()
}

fn run_part2(input: &str) -> String {
    compute(input, 1000000).to_string()
}

#[allow(dead_code)]
const TEST_INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

#[test]
fn test_compute_1() {
    let answer = 374;
    assert_eq!(answer, compute(TEST_INPUT, 2));
}

#[test]
fn test_compute_10() {
    let answer = 1030;
    assert_eq!(answer, compute(TEST_INPUT, 10));
}

#[test]
fn test_compute_100() {
    let answer = 8410;
    assert_eq!(answer, compute(TEST_INPUT, 100));
}
