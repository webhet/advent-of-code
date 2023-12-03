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

    let mut sum: u64 = 0;

    let mut prev_prev_line = None;
    let mut prev_line = None;

    for (idx, line) in reader.lines().enumerate() {
        let line = match line {
            Ok(line) => line,
            Err(err) => {
                println!("Error reading line {err}");
                return;
            }
        };

        if idx == 0 {
            prev_line = Some(line);
        } else {
            let valid_numbers_in_line = get_numbers_from_line(
                prev_prev_line.as_ref(),
                prev_line.as_ref().unwrap(),
                Some(&line),
            );
            sum += valid_numbers_in_line.into_iter().sum::<u64>();

            prev_prev_line = prev_line.take();
            prev_line = Some(line);
        }
    }

    let valid_numbers_in_last_line =
        get_numbers_from_line(prev_prev_line.as_ref(), prev_line.as_ref().unwrap(), None);
    sum += valid_numbers_in_last_line.into_iter().sum::<u64>();

    println!("Sum: {sum}");
}

fn get_numbers_from_line(
    prev_line: Option<&String>,
    line: &str,
    next_line: Option<&String>,
) -> Vec<u64> {
    let mut numbers = Vec::new();

    let mut num_relevant = false;
    let mut num_buf = Vec::new();

    for (i, c) in line.chars().enumerate() {
        match c {
            c if c.is_ascii_digit() => {
                if !num_relevant {
                    if num_buf.is_empty() && i > 0 {
                        // Check above/below char before number
                        num_relevant = check_if_pos_relevant(i - 1, prev_line, next_line);
                        // Check immediately before number
                        num_relevant = num_relevant || is_symbol(line.as_bytes()[i - 1]);
                    }

                    // Check above/below char in number
                    num_relevant = num_relevant || check_if_pos_relevant(i, prev_line, next_line);
                }

                num_buf.push(c as u8);
            }
            _ => {
                if !num_buf.is_empty() {
                    if !num_relevant && i < line.len() {
                        // Check above/below char after number
                        num_relevant = check_if_pos_relevant(i, prev_line, next_line);
                        // Check immediately after number
                        num_relevant = num_relevant || is_symbol(line.as_bytes()[i]);
                    }

                    if num_relevant {
                        let num = String::from_utf8(std::mem::take(&mut num_buf))
                            .unwrap()
                            .parse::<u64>()
                            .unwrap();

                        numbers.push(num);
                    } else {
                        num_buf.clear();
                    }

                    num_relevant = false;
                }
            }
        }
    }

    if !num_buf.is_empty() && num_relevant {
        let num = String::from_utf8(num_buf).unwrap().parse::<u64>().unwrap();
        numbers.push(num);
    }

    numbers
}

fn check_if_pos_relevant(
    pos: usize,
    prev_line: Option<&String>,
    next_line: Option<&String>,
) -> bool {
    match prev_line.map(|prev_line| prev_line.as_bytes()[pos]) {
        Some(c) if is_symbol(c) => {
            return true;
        }
        _ => {}
    }

    match next_line.map(|next_line| next_line.as_bytes()[pos]) {
        Some(c) if is_symbol(c) => {
            return true;
        }
        _ => {}
    }

    false
}

fn is_symbol(c: u8) -> bool {
    !c.is_ascii_digit() && c != b'.'
}
