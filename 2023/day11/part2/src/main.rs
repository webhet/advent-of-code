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

    let galaxy_positions = parse_map(reader);

    let galaxy_pairs = get_galaxy_pairs(galaxy_positions);

    let steps: usize = galaxy_pairs.into_iter().map(|gp| gp.step_distance()).sum();

    println!("Steps {steps}");
}

#[derive(Debug)]
struct GalaxyPair {
    a: (usize, usize),
    b: (usize, usize),
}

impl GalaxyPair {
    pub fn step_distance(&self) -> usize {
        self.a.0.abs_diff(self.b.0) + self.a.1.abs_diff(self.b.1)
    }
}

fn get_galaxy_pairs(galaxy_positions: Vec<(usize, usize)>) -> Vec<GalaxyPair> {
    let mut pairs = Vec::new();

    for (idx, pos1) in galaxy_positions.iter().enumerate() {
        for pos2 in galaxy_positions.iter().skip(idx + 1) {
            pairs.push(GalaxyPair {
                a: (pos1.0, pos1.1),
                b: (pos2.0, pos2.1),
            })
        }
    }

    pairs
}

fn parse_map(reader: BufReader<File>) -> Vec<(usize, usize)> {
    let mut positions = Vec::new();
    let mut x_coords = HashSet::new();
    let mut y_coords = HashSet::new();

    let mut max_y = 0;
    let mut max_x = 0;

    for (y, line) in reader.lines().enumerate() {
        let line = line.expect("Failed to read line");

        max_y = y;

        for (x, c) in line.chars().enumerate() {
            max_x = x;

            if c == '#' {
                positions.push((x, y));
                x_coords.insert(x);
                y_coords.insert(y);
            }
        }
    }

    let all_ys: HashSet<usize> = (0..=max_y).collect();
    let all_xs: HashSet<usize> = (0..=max_x).collect();

    let free_ys: HashSet<usize> = all_ys.difference(&y_coords).copied().collect();
    let free_xs: HashSet<usize> = all_xs.difference(&x_coords).copied().collect();

    positions
        .into_iter()
        .map(|(x, y)| {
            let x_add = free_xs.iter().filter(|fx| x > **fx).count();
            let y_add = free_ys.iter().filter(|fy| y > **fy).count();

            (x + (x_add * (1_000_000 - 1)), y + (y_add * (1_000_000 - 1)))
        })
        .collect()
}
