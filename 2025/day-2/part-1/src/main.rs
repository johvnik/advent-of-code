use std::fs;

fn main() {
    let input = fs::read_to_string("../input.csv").unwrap();

    // let last: Vec<_> = input.split(',').rev().take(5).collect();
    // println!("{last:?}");

    let mut total = 0;

    fn is_not_valid(num: u64) -> bool {
        // let num = num.to_string();

        // for char in num.chars() {}

        true
    }

    input
        .split(',')
        .map(|s| {
            let (start, end) = s.trim().split_once('-').unwrap();
            let start = start.parse::<u64>().unwrap();
            let end = end.parse::<u64>().unwrap();
            (start, end)
        })
        .for_each(|(s, e)| {
            for i in s..=e {
                if is_not_valid(i) {
                    total += i;
                }
            }
        });

    println!("total: {total}");
}
