use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
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

    let mut gear_numbers = Vec::new();

    let mut prev_prev_line = None;
    let mut prev_line = None;

    let mut line_idx = 0;

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
            line_idx = idx - 1;

            gear_numbers.append(&mut get_numbers_from_line(
                line_idx,
                prev_prev_line.as_ref(),
                prev_line.as_ref().unwrap(),
                Some(&line),
            ));

            prev_prev_line = prev_line.take();
            prev_line = Some(line);
        }
    }

    gear_numbers.append(&mut get_numbers_from_line(
        line_idx + 1,
        prev_prev_line.as_ref(),
        prev_line.as_ref().unwrap(),
        None,
    ));

    let mut gear_map = HashMap::new();

    for gear_number in gear_numbers {
        for gear in gear_number.gears {
            match gear_map.entry(gear) {
                Entry::Vacant(e) => {
                    let mut set = HashSet::new();
                    set.insert(gear_number.num);

                    e.insert(set);
                }
                Entry::Occupied(mut e) => {
                    e.get_mut().insert(gear_number.num);
                }
            }
        }
    }

    let mut sum = 0;

    for number_set in gear_map.into_values() {
        if number_set.len() == 2 {
            sum += number_set.into_iter().map(|n| n.num).product::<u64>();
        }
    }

    println!("Sum: {sum}");
}

#[derive(Debug)]
struct GearNumber {
    num: NumberPos,
    gears: Vec<GearPos>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct NumberPos {
    num: u64,
    x: u64,
    y: u64,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct GearPos {
    x: u64,
    y: u64,
}

fn get_numbers_from_line(
    line_idx: usize,
    prev_line: Option<&String>,
    line: &String,
    next_line: Option<&String>,
) -> Vec<GearNumber> {
    let mut numbers = Vec::new();

    let mut num_buf = Vec::new();
    let mut gears = Vec::new();

    for (i, c) in line.chars().enumerate() {
        match c {
            c if c.is_ascii_digit() => {
                if num_buf.is_empty() && i > 0 {
                    // Check above/below char before number
                    if line_idx > 0 {
                        add_gear_from_pos_if_present(&mut gears, line_idx - 1, i - 1, prev_line);
                    }
                    add_gear_from_pos_if_present(&mut gears, line_idx + 1, i - 1, next_line);
                    // Check immediately before number
                    add_gear_from_pos_if_present(&mut gears, line_idx, i - 1, Some(line));
                }

                // Check above/below char in number
                if line_idx > 0 {
                    add_gear_from_pos_if_present(&mut gears, line_idx - 1, i, prev_line);
                }
                add_gear_from_pos_if_present(&mut gears, line_idx + 1, i, next_line);

                num_buf.push(c as u8);
            }
            _ => {
                if !num_buf.is_empty() {
                    if i < line.len() {
                        // Check above/below char after number
                        if line_idx > 0 {
                            add_gear_from_pos_if_present(&mut gears, line_idx - 1, i, prev_line);
                        }
                        add_gear_from_pos_if_present(&mut gears, line_idx + 1, i, next_line);
                        // Check immediately after number
                        add_gear_from_pos_if_present(&mut gears, line_idx, i, Some(line));
                    }

                    if !gears.is_empty() {
                        let num = String::from_utf8(std::mem::take(&mut num_buf))
                            .unwrap()
                            .parse::<u64>()
                            .unwrap();

                        numbers.push(GearNumber {
                            num: NumberPos {
                                num,
                                x: i as u64,
                                y: line_idx as u64,
                            },
                            gears: std::mem::take(&mut gears),
                        });
                    } else {
                        num_buf.clear();
                    }
                }
            }
        }
    }

    if !num_buf.is_empty() && !gears.is_empty() {
        let num = String::from_utf8(num_buf).unwrap().parse::<u64>().unwrap();
        numbers.push(GearNumber {
            num: NumberPos {
                num,
                x: (line.len() - 1) as u64,
                y: line_idx as u64,
            },
            gears,
        });
    }

    numbers
}

fn add_gear_from_pos_if_present(
    gears: &mut Vec<GearPos>,
    line_idx: usize,
    pos: usize,
    line: Option<&String>,
) {
    match line.map(|line| line.as_bytes()[pos]) {
        Some(c) if is_gear(c) => gears.push(GearPos {
            x: pos as u64,
            y: line_idx as u64,
        }),
        _ => {}
    }
}

fn is_gear(c: u8) -> bool {
    c == b'*'
}
