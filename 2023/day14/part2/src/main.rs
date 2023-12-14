use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    env,
    fs::File,
    hash::{Hash, Hasher},
    io::{BufRead, BufReader},
};

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

    let mut columns = parse_columns(reader);

    do_cycles(&mut columns);

    let sum: usize = columns.iter().map(|c| weight_of_column(c)).sum();

    println!("Sum {sum}");
}

#[derive(Debug, Hash)]
enum Object {
    BlockRock,
    RoundRock,
    Empty,
}

fn do_cycles(columns: &mut [Vec<Object>]) {
    const CYCLES: usize = 1000000000;

    let mut set = HashMap::new();

    let mut done_steps = 0;
    let mut cycle_step = 0;

    for i in 0..CYCLES {
        cycle(columns);

        let mut hasher = DefaultHasher::new();
        columns.hash(&mut hasher);
        let hash = hasher.finish();

        if let Some(cycle_idx) = set.get(&hash) {
            done_steps = i + 1;
            cycle_step = i - cycle_idx;
            break;
        }

        set.insert(hash, i);
    }

    let remaining = (CYCLES - done_steps) % cycle_step;

    for _ in 0..remaining {
        cycle(columns);
    }
}

fn cycle(columns: &mut [Vec<Object>]) {
    move_north_south(columns, true);
    //print_cols(columns);

    move_east_west(columns, false);
    //print_cols(columns);

    move_north_south(columns, false);
    //print_cols(columns);

    move_east_west(columns, true);
    //print_cols(columns);
}

fn move_north_south(columns: &mut [Vec<Object>], north: bool) {
    if north {
        columns.iter_mut().for_each(move_column_up);
    } else {
        columns.iter_mut().for_each(move_column_down);
    }
}

fn move_column_up(column: &mut Vec<Object>) {
    let col_len = column.len() as isize;
    let mut last_block_idx = -1;
    let mut round_rock_count = 0;

    for col_idx in 0..col_len as usize {
        let o = &column[col_idx];
        match o {
            Object::BlockRock => {
                let start = (last_block_idx + 1) as usize;

                for item in column.iter_mut().skip(start).take(round_rock_count) {
                    *item = Object::RoundRock;
                }
                for item in column
                    .iter_mut()
                    .take(col_idx)
                    .skip(start + round_rock_count)
                {
                    *item = Object::Empty;
                }

                round_rock_count = 0;
                last_block_idx = col_idx as isize;
            }
            Object::RoundRock => {
                round_rock_count += 1;
            }
            Object::Empty => {}
        }
    }

    let start = (last_block_idx + 1) as usize;

    for item in column.iter_mut().skip(start).take(round_rock_count) {
        *item = Object::RoundRock;
    }
    for item in column.iter_mut().skip(start + round_rock_count) {
        *item = Object::Empty;
    }
}

fn move_column_down(column: &mut Vec<Object>) {
    let col_len = column.len() as isize;
    let mut last_block_idx = column.len();
    let mut round_rock_count = 0;

    for col_idx in (0..col_len as usize).rev() {
        let o = &column[col_idx];
        match o {
            Object::BlockRock => {
                for i in (last_block_idx - round_rock_count)..last_block_idx {
                    column[i] = Object::RoundRock;
                }
                for i in (col_idx + 1)..(last_block_idx - round_rock_count) {
                    column[i] = Object::Empty;
                }

                round_rock_count = 0;
                last_block_idx = col_idx;
            }
            Object::RoundRock => {
                round_rock_count += 1;
            }
            Object::Empty => {}
        }
    }

    for i in (last_block_idx - round_rock_count)..last_block_idx {
        column[i] = Object::RoundRock;
    }
    for i in 0..(last_block_idx - round_rock_count) {
        column[i] = Object::Empty;
    }
}

fn move_east_west(columns: &mut [Vec<Object>], east: bool) {
    let row_len = columns.first().map(|r| r.len()).unwrap_or(0);

    if east {
        for row_idx in 0..row_len {
            move_column_right(columns, row_idx, row_len);
        }
    } else {
        for row_idx in 0..row_len {
            move_column_left(columns, row_idx);
        }
    }
}

fn move_column_left(columns: &mut [Vec<Object>], row_idx: usize) {
    let mut last_block_idx = -1;
    let mut round_rock_count = 0;

    for col_idx in 0..columns.len() {
        let o = &columns[col_idx][row_idx];

        match o {
            Object::BlockRock => {
                let start = (last_block_idx + 1) as usize;

                for i in start..(start + round_rock_count) {
                    columns[i][row_idx] = Object::RoundRock;
                }
                for i in (start + round_rock_count)..col_idx {
                    columns[i][row_idx] = Object::Empty;
                }

                round_rock_count = 0;
                last_block_idx = col_idx as isize;
            }
            Object::RoundRock => {
                round_rock_count += 1;
            }
            Object::Empty => {}
        }
    }

    let start = (last_block_idx + 1) as usize;

    for i in start..(start + round_rock_count) {
        columns[i][row_idx] = Object::RoundRock;
    }
    for i in (start + round_rock_count)..columns.len() {
        columns[i][row_idx] = Object::Empty;
    }
}

fn move_column_right(columns: &mut [Vec<Object>], row_idx: usize, row_len: usize) {
    let mut last_block_idx = row_len;
    let mut round_rock_count = 0;

    for col_idx in (0..columns.len()).rev() {
        let o = &columns[col_idx][row_idx];

        match o {
            Object::BlockRock => {
                for i in (last_block_idx - round_rock_count)..last_block_idx {
                    columns[i][row_idx] = Object::RoundRock;
                }
                for i in (col_idx + 1)..(last_block_idx - round_rock_count) {
                    columns[i][row_idx] = Object::Empty;
                }

                round_rock_count = 0;
                last_block_idx = col_idx;
            }
            Object::RoundRock => {
                round_rock_count += 1;
            }
            Object::Empty => {}
        }
    }

    for i in (last_block_idx - round_rock_count)..last_block_idx {
        columns[i][row_idx] = Object::RoundRock;
    }
    for i in 0..(last_block_idx - round_rock_count) {
        columns[i][row_idx] = Object::Empty;
    }
}

fn weight_of_column(column: &[Object]) -> usize {
    column.iter().enumerate().fold(0, |acc, (idx, obj)| {
        if matches!(obj, Object::RoundRock) {
            acc + (column.len() - idx)
        } else {
            acc
        }
    })
}

fn parse_columns(reader: BufReader<File>) -> Vec<Vec<Object>> {
    let mut columns = Vec::new();

    for (idx, line) in reader.lines().enumerate() {
        let line = line.expect("Failed to read line");

        if idx == 0 {
            for _ in 0..line.chars().count() {
                columns.push(Vec::new());
            }
        }

        for (c_idx, c) in line.chars().enumerate() {
            let block = match c {
                '#' => Object::BlockRock,
                'O' => Object::RoundRock,
                '.' => Object::Empty,
                _ => panic!("Unexpected char"),
            };

            columns[c_idx].push(block);
        }
    }

    columns
}
