use std::{
    collections::HashSet,
    env,
    fs::File,
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

    let (start, map) = parse_map(reader);

    let count = count_garden_plots_after_steps(64, start, &map);

    println!("Count {count}");
}

fn count_garden_plots_after_steps(steps: usize, start: (usize, usize), map: &[Vec<bool>]) -> usize {
    let max_x = map.first().map(|l| l.len() - 1).unwrap_or(0);
    let max_y = map.len() - 1;

    let mut current_position_set = HashSet::new();
    current_position_set.insert(start);

    for _ in 0..steps {
        let current_positions = std::mem::take(&mut current_position_set);

        for pos in current_positions {
            let neighbours = find_allowed_neighbours(pos.0, pos.1, max_x, max_y, map);

            for neighbour in neighbours {
                current_position_set.insert(neighbour);
            }
        }
    }

    current_position_set.len()
}

fn find_allowed_neighbours(
    x: usize,
    y: usize,
    max_x: usize,
    max_y: usize,
    map: &[Vec<bool>],
) -> Vec<(usize, usize)> {
    let mut allowed = Vec::with_capacity(4);

    // Up
    if y > 0 && !map[y - 1][x] {
        allowed.push((x, y - 1));
    }

    // Down
    if y < max_y && !map[y + 1][x] {
        allowed.push((x, y + 1));
    }

    // Left
    if x > 0 && !map[y][x - 1] {
        allowed.push((x - 1, y));
    }

    // Right
    if x < max_x && !map[y][x + 1] {
        allowed.push((x + 1, y));
    }

    allowed
}

fn parse_map(reader: BufReader<File>) -> ((usize, usize), Vec<Vec<bool>>) {
    let mut start = (0, 0);

    let map = reader
        .lines()
        .enumerate()
        .map(|(y, line)| {
            let line = line.expect("Failed to read line");

            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == 'S' {
                        start = (x, y)
                    }

                    c == '#'
                })
                .collect()
        })
        .collect();

    (start, map)
}
