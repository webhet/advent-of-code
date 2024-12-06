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

    let mut map = Vec::new();

    let mut guard_pos = (0, 0);
    let mut guard_dir = (0_isize, -1_isize);

    for (y, line) in reader.lines().enumerate() {
        let line = line.expect("Failed to read line");

        if let Some(x) = line.as_bytes().iter().position(|b| *b == b'^') {
            guard_pos = (x as isize, y as isize);
        }

        map.push(line.chars().collect::<Vec<_>>());
    }

    let mut steps = 0;

    loop {
        match get_pos(&guard_pos, &map) {
            Some(c) => {
                if c != 'X' {
                    steps += 1;
                    map[guard_pos.1 as usize][guard_pos.0 as usize] = 'X';
                }

                step(&mut guard_dir, &mut guard_pos, &map);
            }
            None => break,
        }
    }

    println!("Steps: {steps}")
}

fn step(dir: &mut (isize, isize), pos: &mut (isize, isize), map: &[Vec<char>]) {
    loop {
        let new_pos = next_pos(dir, pos);

        if get_pos(&new_pos, map) == Some('#') {
            *dir = turn(dir);
        } else {
            *pos = new_pos;
            break;
        }
    }
}

fn next_pos(dir: &(isize, isize), pos: &(isize, isize)) -> (isize, isize) {
    (pos.0 + dir.0, pos.1 + dir.1)
}

fn turn(dir: &(isize, isize)) -> (isize, isize) {
    match dir {
        (0, 1) => (-1, 0),
        (-1, 0) => (0, -1),
        (0, -1) => (1, 0),
        (1, 0) => (0, 1),
        _ => unreachable!(),
    }
}

fn get_pos(pos: &(isize, isize), map: &[Vec<char>]) -> Option<char> {
    if pos.0 < 0 || pos.1 < 0 {
        None
    } else {
        map.get(pos.1 as usize)
            .and_then(|r| r.get(pos.0 as usize))
            .copied()
    }
}
