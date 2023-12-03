use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

const NUM_STRS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

enum SearchDirection {
    Left,
    Right,
}

fn main() {
    let Some(filepath) = env::args().nth(1) else {
        println!("Usage: part2 <filepath>");
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

        let mut leftmost = None;
        let mut rightmost = None;

        let line_bytes = line.as_bytes();

        for (idx, c) in line_bytes.iter().enumerate() {
            if c.is_ascii_digit() {
                leftmost = Some(c - 0x30);
                break;
            }

            if let Some(num) = matches_num_str(line_bytes, idx, SearchDirection::Left) {
                leftmost = Some(num);
                break;
            }
        }

        for (idx, c) in line_bytes.iter().enumerate().rev() {
            if c.is_ascii_digit() {
                rightmost = Some(c - 0x30);
                break;
            }

            if let Some(num) = matches_num_str(line_bytes, idx, SearchDirection::Right) {
                rightmost = Some(num);
                break;
            }
        }

        let number = match (leftmost, rightmost) {
            (Some(leftmost), Some(rightmost)) => leftmost * 10 + rightmost,
            _ => {
                println!("Error: Didn't find a single digit in the line!");
                return;
            }
        };

        sum += number as u64;
    }

    println!("The sum is: {sum}");
}

fn matches_num_str(line_bytes: &[u8], idx: usize, search_direction: SearchDirection) -> Option<u8> {
    for (num_idx, num_str) in NUM_STRS.iter().enumerate() {
        let search_range = match search_direction {
            SearchDirection::Left => {
                let rbound = std::cmp::min(idx + num_str.len(), line_bytes.len() - 1);
                &line_bytes[idx..rbound]
            }
            SearchDirection::Right => {
                let lbound = std::cmp::max(idx as isize - (num_str.len() - 1) as isize, 0) as usize;
                &line_bytes[lbound..=idx]
            }
        };

        if num_str.as_bytes() == search_range {
            return Some(num_idx as u8 + 1);
        }
    }

    None
}
