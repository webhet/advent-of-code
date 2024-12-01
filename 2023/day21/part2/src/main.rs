use std::{
    collections::{HashMap, HashSet},
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

    let count = count_garden_plots_after_steps(1000, start, &map);

    println!("Count {count}");
}

fn count_garden_plots_after_steps(steps: usize, start: (usize, usize), map: &[Vec<bool>]) -> usize {
    let max_x = map.first().map(|l| l.len() - 1).unwrap_or(0);
    let max_y = map.len() - 1;

    let mut current_position_set = HashSet::new();
    current_position_set.insert((start.0 as isize, start.1 as isize));

    //let mut cache = HashMap::new();

    let mut plot_count = 0;

    for _ in 0..steps {
        let current_positions = std::mem::take(&mut current_position_set);
        let current_len = current_positions.len();

        for pos in current_positions {
            for (offset_x, offset_y) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let x = pos.0 + offset_x;
                let y = pos.1 + offset_y;

                let idx_x = coord_to_idx(x, max_x);
                let idx_y = coord_to_idx(y, max_y);

                if !map[idx_y][idx_x] {
                    current_position_set.insert((x, y));
                }
            }
        }
    }

    //println!("{}", cache.len()); RL 15 UD 21

    plot_count
}

#[inline]
fn coord_to_idx(coord: isize, max: usize) -> usize {
    let idx = coord % (max as isize + 1);

    if idx < 0 {
        (idx + max as isize + 1) as usize
    } else {
        idx as usize
    }
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
