use std::{
    env,
    fs::File,
    io::{BufRead, BufReader, Lines},
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

    let mut lines_iter = reader.lines();

    let races = parse_races(&mut lines_iter);

    let num_ways_to_win: u64 = races.iter().map(|race| race.num_ways_to_win()).product();

    println!("Num ways we can win: {num_ways_to_win}");
}

struct Race {
    time_ms: u64,
    record_mm: u64,
}

impl Race {
    pub fn num_ways_to_win(&self) -> u64 {
        let middle = self.time_ms / 2;

        let mut possible_ways = 0;

        for i in 1..=middle {
            if (self.time_ms - i) * i > self.record_mm {
                possible_ways += 1;
            }
        }

        let possible_ways = possible_ways * 2;

        if self.time_ms % 2 == 0 {
            possible_ways - 1
        } else {
            possible_ways
        }
    }
}

fn parse_races(lines_iter: &mut Lines<BufReader<File>>) -> Vec<Race> {
    let time_line = lines_iter
        .next()
        .expect("Unexpected EOF")
        .expect("Line read fail");
    let time_strs = time_line.split_ascii_whitespace();
    let record_distance_line = lines_iter
        .next()
        .expect("Unexpected EOF")
        .expect("Line read fail");
    let mut record_distance_strs = record_distance_line.split_ascii_whitespace();

    record_distance_strs.next().expect("Unexpected EOL"); // Skip 1

    time_strs
        .into_iter()
        .skip(1)
        .map(|time_str| {
            let record_distance_str = record_distance_strs
                .next()
                .expect("Got different length for distance");

            let time_ms = time_str.parse().expect("NUm parse failed");
            let record_mm = record_distance_str.parse().expect("NUm parse failed");

            Race { time_ms, record_mm }
        })
        .collect()
}
