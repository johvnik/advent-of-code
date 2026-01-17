use std::fs;

fn main() {
    let input = fs::read_to_string("../input.txt").unwrap();

    fn to_joltage(bank: &str) -> u32 {
        // Monotonic Stack
        let mut stack: Vec<char> = Vec::new();

        for (i, battery) in bank.chars().enumerate() {
            while let Some(top) = stack.last() {
                // Check that we don't pop the very last value
                if i == bank.len() - 1 && stack.len() < 2 {
                    break;
                }

                if battery > *top {
                    stack.pop();
                } else {
                    break;
                }
            }

            if stack.len() < 2 {
                stack.push(battery);
            }
        }

        stack
            .into_iter()
            .map(|c| c.to_digit(10).unwrap())
            .fold(0, |acc, digit| acc * 10 + digit)
    }

    let result: u32 = input
        .lines()
        // .take(5)
        .map(to_joltage)
        .sum();

    println!("total joltage: {result}");
}
