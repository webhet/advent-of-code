use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let Some(filepath) = env::args().nth(1) else {
        println!("Usage: part1 <filepath>");
        return;
    };

    let file = match File::open(filepath) {
        Ok(file) => file,
        Err(err) => {
            println!("Error opening input file: {err}");
            return;
        }
    };

    let reader = BufReader::new(file);

    let mut sum = 0;

    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(err) => {
                println!("Error reading line {err}");
                return;
            }
        };

        let leftmost = line.chars().find(|c| c.is_ascii_digit());
        let rightmost = line.chars().rfind(|c| c.is_ascii_digit());

        let number = match (leftmost, rightmost) {
            (Some(leftmost), Some(rightmost)) => {
                leftmost.to_digit(10).unwrap() * 10 + rightmost.to_digit(10).unwrap()
            }
            _ => {
                println!("Error: Didn't find a single digit in the line!");
                return;
            }
        };

        sum += number;
    }

    println!("The sum is: {sum}");
}
