use std::{
    collections::{HashMap, HashSet},
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

    let mut antennas: HashMap<char, Vec<(isize, isize)>> = HashMap::new();

    let mut x_max = 0;
    let mut y_max = 0;

    for (y, line) in reader.lines().enumerate() {
        let line = line.expect("Failed to read line");

        y_max = y as isize;

        for (x, c) in line.chars().enumerate() {
            x_max = x as isize;

            if c != '.' {
                antennas
                    .entry(c)
                    .and_modify(|v| v.push((x as isize, y as isize)))
                    .or_insert(vec![(x as isize, y as isize)]);
            }
        }
    }

    let mut antinode_positions = HashSet::new();

    for positions in antennas.into_values() {
        let combinations = get_antenna_combinations(&positions);

        for (a, b) in combinations {
            let delta_x = b.0 - a.0;
            let delta_y = b.1 - a.1;

            antinode_positions.insert((a.0 - delta_x, a.1 - delta_y));
            antinode_positions.insert((b.0 + delta_x, b.1 + delta_y));
        }
    }

    /*
    for y in 0..=y_max {
        for x in 0..x_max {
            if antinode_positions.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }

        println!();
    }
    */

    let unique_antinodes = antinode_positions
        .iter()
        .filter(|pos| pos.0 >= 0 && pos.0 <= x_max && pos.1 >= 0 && pos.1 <= y_max)
        .count();

    println!("Unique antinodes: {}", unique_antinodes);
}

fn get_antenna_combinations(positions: &[(isize, isize)]) -> Vec<((isize, isize), (isize, isize))> {
    let mut combinations = Vec::new();

    for (idx, a) in positions.iter().enumerate() {
        for b in &positions[(idx + 1)..] {
            combinations.push((*a, *b));
        }
    }

    combinations
}
