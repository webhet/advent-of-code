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

    let mut guard_starting_pos = (0, 0);

    for (y, line) in reader.lines().enumerate() {
        let line = line.expect("Failed to read line");

        if let Some(x) = line.as_bytes().iter().position(|b| *b == b'^') {
            guard_starting_pos = (x as isize, y as isize);
        }

        map.push(line.chars().collect::<Vec<_>>());
    }

    let possible_positions = get_possible_positions(guard_starting_pos, map.clone());

    let mut loops = 0;

    for obstacle_pos in possible_positions {
        let mut current_map = map.clone();
        current_map[obstacle_pos.1 as usize][obstacle_pos.0 as usize] = '#';

        let is_loop = check_map_for_loop(guard_starting_pos, current_map);

        if is_loop {
            loops += 1;
        }
    }

    println!("Loops: {loops}")
}

fn check_map_for_loop(mut guard_pos: (isize, isize), mut map: Vec<Vec<char>>) -> bool {
    map[guard_pos.1 as usize][guard_pos.0 as usize] = '|';

    let mut guard_dir = (0_isize, -1_isize);

    loop {
        match get_pos(&guard_pos, &map) {
            Some(c) => {
                if c == '+' {
                    let next_pos = next_pos(&guard_dir, &guard_pos);
                    if get_pos(&next_pos, &map) == Some('#') {
                        break true;
                    }
                }

                // If now traveling perpendicular
                if c == '-' && (guard_dir == (0, 1) || guard_dir == (0, -1)) {
                    map[guard_pos.1 as usize][guard_pos.0 as usize] = '+';
                }
                // If now traveling perpendicular
                else if c == '|' && (guard_dir == (1, 0) || guard_dir == (-1, 0)) {
                    map[guard_pos.1 as usize][guard_pos.0 as usize] = '+';
                }
                // We were not here yet
                else {
                    match guard_dir {
                        (0, 1) | (0, -1) => {
                            map[guard_pos.1 as usize][guard_pos.0 as usize] = '|';
                        }
                        (1, 0) | (-1, 0) => {
                            map[guard_pos.1 as usize][guard_pos.0 as usize] = '-';
                        }
                        _ => unreachable!(),
                    }
                }

                let pos_before_turn = guard_pos;
                let had_to_turn = step(&mut guard_dir, &mut guard_pos, &map);

                if had_to_turn {
                    map[pos_before_turn.1 as usize][pos_before_turn.0 as usize] = '+';
                }
            }
            None => break false,
        }
    }
}

fn get_possible_positions(
    mut guard_pos: (isize, isize),
    mut map: Vec<Vec<char>>,
) -> Vec<(isize, isize)> {
    let mut guard_dir = (0_isize, -1_isize);

    let mut possible_positions = Vec::new();

    loop {
        match get_pos(&guard_pos, &map) {
            Some(c) => {
                if c != 'X' {
                    possible_positions.push(guard_pos);
                    map[guard_pos.1 as usize][guard_pos.0 as usize] = 'X';
                }

                step(&mut guard_dir, &mut guard_pos, &map);
            }
            None => break,
        }
    }

    possible_positions
}

fn step(dir: &mut (isize, isize), pos: &mut (isize, isize), map: &[Vec<char>]) -> bool {
    let mut had_to_turn = false;

    loop {
        let new_pos = next_pos(dir, pos);

        if get_pos(&new_pos, map) == Some('#') {
            *dir = turn(dir);
            had_to_turn = true;
        } else {
            *pos = new_pos;
            break;
        }
    }

    had_to_turn
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
