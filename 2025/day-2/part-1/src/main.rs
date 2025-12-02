use std::fs;

fn main() {
    let input = fs::read_to_string("../input.txt").unwrap();

    fn is_valid(num: u64) -> bool {
        let num = num.to_string();
        let len = num.len();

        if len % 2 == 1 { return true; }

        let (l, r) = num.split_at(len / 2);
        l != r
    }

    let result: u64 = input
        .split(',')
        .map(|s| {
            let (start, end) = s.trim().split_once('-').unwrap();
            (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap())
        })
        .flat_map(|(s, e)| s..=e)
        .filter(|&n| !is_valid(n))
        .sum();

    println!("total: {result}");
}
