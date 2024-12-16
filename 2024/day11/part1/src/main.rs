use std::{env, fs};

fn main() {
    let Some(filepath) = env::args().nth(1) else {
        println!("Usage: part1 <filepath>");
        return;
    };

    let input = fs::read_to_string(filepath).expect("Should have been able to read the file");

    let mut stones = input
        .split(" ")
        .into_iter()
        .map(|s| s.parse::<isize>().expect("Failed to parse num"))
        .collect::<Vec<_>>();

    stones.reserve(10_000_000);

    for _ in 0..25 {
        let mut i = 0;

        while stones.len() > i {
            if stones[i] == 0 {
                stones[i] = 1;
            } else {
                let str = stones[i].to_string();

                if str.len() % 2 == 0 {
                    stones[i] = str[0..(str.len() / 2)]
                        .parse::<isize>()
                        .expect("Failed to parse num");
                    stones.insert(
                        i + 1,
                        str[(str.len() / 2)..]
                            .parse::<isize>()
                            .expect("Failed to parse num"),
                    );

                    i += 1;
                } else {
                    stones[i] *= 2024;
                }
            }

            i += 1;
        }
    }

    println!("Stones: {}", stones.len());
}
