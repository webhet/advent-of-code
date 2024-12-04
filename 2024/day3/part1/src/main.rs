use std::{env, fs};

use regex::Regex;

fn main() {
    let Some(filepath) = env::args().nth(1) else {
        println!("Usage: part1 <filepath>");
        return;
    };

    let input = fs::read_to_string(filepath).expect("Should have been able to read the file");

    let regex = Regex::new("mul\\(([0-9]{1,3}),([0-9]{1,3})\\)").expect("Valid regex");

    let res = regex
        .captures_iter(&input)
        .map(|c| c.extract())
        .map(|(_, [op1, op2])| {
            let num1 = op1.parse::<isize>().expect("Failed to parse num");
            let num2 = op2.parse::<isize>().expect("Failed to parse num");

            num1 * num2
        })
        .sum::<isize>();

    println!("Result: {res}");
}
