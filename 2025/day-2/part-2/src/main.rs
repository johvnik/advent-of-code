use std::fs;

fn main() {
    let input = fs::read_to_string("../input.txt").unwrap();

    // let input = "2121212118-2121212124";

    fn is_valid(num: u64) -> bool {
        let num: Vec<char> = num.to_string().chars().collect();
        let len = num.len();

        for i in 1..=len/2 {
            if len % i == 0 {
                let chunk_size = i;
                let mut chunks = num.chunks(chunk_size);
                let first = chunks.next().unwrap();
                if chunks.all(|c| c == first) { return false };
            }
        }

        true
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

