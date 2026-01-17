use std::fs;

fn main() {
    let input = fs::read_to_string("../input.txt").unwrap();

    fn to_joltage(bank: &str) -> u64 {
        let len = bank.len();
        let stack = Vec::from(bank[0..12]);
        let bank = bank.chars();

        for i in 12..len {
            // for every value, run through the stack, and compare
            // and remove any values that aren't in increasing order.

            for w in stack.windows(2) {
                if w[0] > 0 && w[1] > 0 && w[0] < w[1] {
                    *w[0] *= -1;
                    break;
                }
            }

            stack.push(bank[i]);
        }

        println!("{stack:?}");
        1
    }

    let result: u64 = input
        .lines()
        .take(1)
        .map(to_joltage)
        .sum();

    println!("total joltage: {result}");
}

