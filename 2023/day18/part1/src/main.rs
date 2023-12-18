use std::{
    collections::VecDeque,
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

    let directions = parse_directions(reader);

    let outline = map_outline(&directions);

    print_outline(&outline);

    let fill = fill_outline(&outline);

    println!();

    print_outline(&fill);

    let squares: usize = fill.iter().flatten().map(|b| if *b { 1 } else { 0 }).sum();

    println!("Squares {squares}");
}

#[derive(Debug)]
enum Direction {
    Up(isize),
    Down(isize),
    Right(isize),
    Left(isize),
}

fn fill_outline(outline: &VecDeque<VecDeque<bool>>) -> VecDeque<VecDeque<bool>> {
    let mut outline_with_fill = outline.clone();

    let mut last_below_above = 0;

    for y in 0..outline.len() {
        let mut inside = false;

        for x in 0..(outline[0].len()) {
            if outline[y][x] {
                let above = y > 0 && outline[y - 1][x];
                let below = y < outline.len() - 1 && outline[y + 1][x];

                if above && below {
                    inside = !inside;
                    last_below_above = 0;
                    continue;
                }

                if below && (last_below_above != -1) {
                    inside = !inside;
                    last_below_above = 1;
                    continue;
                }

                if above && (last_below_above != 1) {
                    inside = !inside;
                    last_below_above = -1;
                    continue;
                }
            }

            if inside || outline[y][x] {
                outline_with_fill[y][x] = true;
            }
        }
    }

    outline_with_fill
}

fn print_outline(outline: &VecDeque<VecDeque<bool>>) {
    for r in outline {
        for p in r {
            if *p {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn map_outline(directions: &[Direction]) -> VecDeque<VecDeque<bool>> {
    let mut map: VecDeque<VecDeque<bool>> = VecDeque::from([VecDeque::from([true])]);
    let mut curr_x: isize = 0;
    let mut curr_y: isize = 0;
    let mut offset_x: isize = 0;
    let mut offset_y: isize = 0;

    for direction in directions {
        //print_outline(&map);
        //println!();
        //println!("{:?}", direction);

        match direction {
            Direction::Up(len) => {
                for i in ((curr_y - len)..=curr_y).rev() {
                    let y_idx = i + offset_y;

                    if y_idx >= 0 {
                        map[y_idx as usize][(curr_x + offset_x) as usize] = true;
                    } else {
                        let new_row_len = map.front().unwrap().len();

                        map.push_front(VecDeque::from_iter(
                            std::iter::repeat(false).take(new_row_len),
                        ));
                        offset_y += 1;
                        map.front_mut().unwrap()[(curr_x + offset_x) as usize] = true;
                    }
                }

                curr_y -= len;
            }
            Direction::Down(len) => {
                for i in curr_y..=(curr_y + len) {
                    let y_idx = i + offset_y;

                    if y_idx < map.len() as isize {
                        map[y_idx as usize][(curr_x + offset_x) as usize] = true;
                    } else {
                        let new_row_len = map.front().unwrap().len();

                        map.push_back(VecDeque::from_iter(
                            std::iter::repeat(false).take(new_row_len),
                        ));
                        map.back_mut().unwrap()[(curr_x + offset_x) as usize] = true;
                    }
                }

                curr_y += len;
            }
            Direction::Right(len) => {
                for i in curr_x..=(curr_x + len) {
                    let x_idx = i + offset_x;

                    if x_idx < map[(curr_y + offset_y) as usize].len() as isize {
                        map[(curr_y + offset_y) as usize][x_idx as usize] = true;
                    } else {
                        for (idx, row) in map.iter_mut().enumerate() {
                            row.push_back(idx == (curr_y + offset_y) as usize);
                        }
                    }
                }

                curr_x += len;
            }
            Direction::Left(len) => {
                for i in ((curr_x - len)..=curr_x).rev() {
                    let x_idx = i + offset_x;

                    if x_idx >= 0 {
                        map[(curr_y + offset_y) as usize][x_idx as usize] = true;
                    } else {
                        for (idx, row) in map.iter_mut().enumerate() {
                            row.push_front(idx == (curr_y + offset_y) as usize);
                        }
                        offset_x += 1;
                    }
                }

                curr_x -= len;
            }
        }
    }

    map
}

fn parse_directions(reader: BufReader<File>) -> Vec<Direction> {
    reader
        .lines()
        .map(|line| {
            let line = line.expect("Faield to read line");

            let line_split = line.split(' ').collect::<Vec<_>>();

            let num = line_split[1].parse().expect("Num parse failed");

            match line_split[0] {
                "U" => Direction::Up(num),
                "D" => Direction::Down(num),
                "L" => Direction::Left(num),
                "R" => Direction::Right(num),
                _ => panic!("Unexpected direction"),
            }
        })
        .collect()
}
