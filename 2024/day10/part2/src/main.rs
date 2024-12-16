use std::{
    collections::HashSet,
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

    let mut map = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");

        map.push(
            line.chars()
                .map(|c| c.to_digit(10).expect("Failed to parse num"))
                .collect::<Vec<_>>(),
        );
    }

    let mut sum = 0;

    for (y, row) in map.iter().enumerate() {
        for (x, elem) in row.iter().enumerate() {
            if *elem == 0 {
                sum += count_trailheads(x as isize, y as isize, 0, &map);
            }
        }
    }

    println!("Sum: {sum}");
}

fn count_trailheads(x: isize, y: isize, acc: usize, map: &[Vec<u32>]) -> usize {
    let num_at_pos = map[y as usize][x as usize];

    let mut new_acc = acc;

    if num_at_pos == 9 {
        return 1;
    }

    // Top
    if let Some(num) = get_pos(x, y - 1, map) {
        if num == num_at_pos + 1 {
            new_acc += count_trailheads(x, y - 1, acc, map);
        }
    }

    // Right
    if let Some(num) = get_pos(x + 1, y, map) {
        if num == num_at_pos + 1 {
            new_acc += count_trailheads(x + 1, y, acc, map);
        }
    }

    // Bottom
    if let Some(num) = get_pos(x, y + 1, map) {
        if num == num_at_pos + 1 {
            new_acc += count_trailheads(x, y + 1, acc, map);
        }
    }

    // Left
    if let Some(num) = get_pos(x - 1, y, map) {
        if num == num_at_pos + 1 {
            new_acc += count_trailheads(x - 1, y, acc, map);
        }
    }

    new_acc
}

fn get_pos(x: isize, y: isize, map: &[Vec<u32>]) -> Option<u32> {
    if x < 0 || y < 0 {
        None
    } else {
        map.get(y as usize).and_then(|r| r.get(x as usize)).copied()
    }
}
