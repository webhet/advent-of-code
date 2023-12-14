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

    let columns = parse_columns(reader);

    let sum: usize = columns.into_iter().map(north_weight_of_column).sum();

    println!("Sum {sum}");
}

#[derive(Debug)]
enum Object {
    BlockRock,
    RoundRock,
    Empty,
}

fn north_weight_of_column(column: Vec<Object>) -> usize {
    let col_len = column.len() as isize;
    let mut sum = 0;
    let mut last_block_idx = -1;
    let mut round_rock_count = 0;

    for (col_idx, o) in column.into_iter().enumerate() {
        match o {
            Object::BlockRock => {
                let first_rock_weight = col_len - last_block_idx - 1;
                for i in 0..round_rock_count {
                    sum += (first_rock_weight - i) as usize;
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

    let first_rock_weight = col_len - last_block_idx - 1;
    for i in 0..round_rock_count {
        sum += (first_rock_weight - i) as usize;
    }

    sum
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
