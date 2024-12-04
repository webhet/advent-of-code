use std::{env, fs};

use regex::Regex;

fn main() {
    let Some(filepath) = env::args().nth(1) else {
        println!("Usage: part1 <filepath>");
        return;
    };

    let input = fs::read_to_string(filepath).expect("Should have been able to read the file");

    let regex = Regex::new("(?:mul\\(([0-9]{1,3}),([0-9]{1,3})\\))|(?:do\\(\\)|don't\\(\\))")
        .expect("Valid regex");

    let mut enabled = true;
    let mut sum = 0;

    for captures in regex.captures_iter(&input) {
        let match_str = captures.get(0).expect("Capture expected").as_str();

        match match_str {
            "do()" => {
                enabled = true;
            }
            "don't()" => {
                enabled = false;
            }
            _ if enabled => {
                let num1 = captures
                    .get(1)
                    .expect("Capture expected")
                    .as_str()
                    .parse::<isize>()
                    .expect("Failed to parse num");
                let num2 = captures
                    .get(2)
                    .expect("Capture expected")
                    .as_str()
                    .parse::<isize>()
                    .expect("Failed to parse num");

                sum += num1 * num2;
            }
            _ => {}
        }
    }

    println!("Result: {sum}");
}
