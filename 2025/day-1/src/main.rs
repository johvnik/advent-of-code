// PART 2
use std::fs;

fn main() {
    let input: String = fs::read_to_string("input.txt").unwrap();

    let mut pos: i32 = 50;
    let mut zero_count = 0;

    input.lines()
        .map(|s: &str| {
            let (dir, val) = s.split_at(1);
            let val = val.trim().parse::<i32>().unwrap();
            (dir, val)
        })
        .for_each(|(dir, val)| {

            for _ in 0..val {
                match dir {
                    "L" => pos -= 1,
                    "R" => pos += 1,
                    _ => unreachable!()
                }

                pos = pos.rem_euclid(100);
                if pos == 0 { zero_count += 1; }
            }

        });

    println!("{zero_count}");
}

// PART 1
// use std::fs;

// fn main() {
//     let input: String = fs::read_to_string("input.txt").unwrap();
//     // let first5: Vec<_> = input.lines().take(5).collect();

//     let mut pos = 50;
//     let mut zero_count = 0;

//     input.lines()
//         .map(|s: &str| {
//             let (dir, val) = s.split_at(1);
//             let val = val.trim().parse::<i32>().unwrap();
//             (dir, val)
//         })
//         .for_each(|(dir, val)| {
//             match dir {
//                 "L" => pos -= val,
//                 "R" => pos += val,
//                 _ => unreachable!()
//             }

//             // pos = (pos % 100 + 100) % 100;
//             pos = pos.rem_euclid(100);

//             if pos == 0 { zero_count += 1; }
//         });

//     println!("{zero_count}");
// }
