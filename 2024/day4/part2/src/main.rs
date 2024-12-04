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

    let grid = reader
        .lines()
        .map(|line| {
            line.expect("Failed to read line")
                .chars()
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut sum = 0;

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            sum += if check_pos(&grid, x as isize, y as isize) {
                1
            } else {
                0
            };
        }
    }

    println!("Sum: {sum}")
}

fn check_pos(grid: &[Vec<char>], x: isize, y: isize) -> bool {
    let mut pos_sum = 0;

    if get_pos(grid, x, y) != Some('A') {
        return false;
    }

    // top-left -> bottom-right
    if get_pos(grid, x - 1, y - 1) == Some('M') && get_pos(grid, x + 1, y + 1) == Some('S') {
        pos_sum += 1;
    }

    // bottom-right -> top-left
    if get_pos(grid, x - 1, y - 1) == Some('S') && get_pos(grid, x + 1, y + 1) == Some('M') {
        pos_sum += 1;
    }

    // top-right -> bottom-left
    if get_pos(grid, x + 1, y - 1) == Some('M') && get_pos(grid, x - 1, y + 1) == Some('S') {
        pos_sum += 1;
    }

    // bottom-left -> top-right
    if get_pos(grid, x + 1, y - 1) == Some('S') && get_pos(grid, x - 1, y + 1) == Some('M') {
        pos_sum += 1;
    }

    if pos_sum == 2 {
        true
    } else {
        false
    }
}

fn get_pos(grid: &[Vec<char>], x: isize, y: isize) -> Option<char> {
    if x < 0 || y < 0 {
        None
    } else {
        grid.get(y as usize)
            .and_then(|r| r.get(x as usize).copied())
    }
}
