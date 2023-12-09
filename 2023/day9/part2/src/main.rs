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

    let input_sequences = parse_input_sequences(reader);

    let sum: i64 = input_sequences.into_iter().map(|is| is.find_prev()).sum();

    println!("Sum: {sum}");
}

struct InputSequence(Vec<i64>);

impl InputSequence {
    pub fn find_prev(&self) -> i64 {
        let mut difference_sequences = Vec::new();

        loop {
            let seq = difference_sequences.last().unwrap_or(&self.0);

            let diff: Vec<i64> = seq
                .iter()
                .enumerate()
                .skip(1)
                .map(|(idx, val)| {
                    let prev = seq[idx - 1];

                    let diff = (val - prev).abs();

                    if *val < prev {
                        -diff
                    } else {
                        diff
                    }
                })
                .collect();

            if diff.iter().all(|v| *v == 0) {
                break;
            }

            difference_sequences.push(diff);
        }

        let last_prev = difference_sequences
            .into_iter()
            .rev()
            .fold(0, |acc, seq| seq.first().unwrap() - acc);

        self.0.first().unwrap() - last_prev
    }
}

fn parse_input_sequences(reader: BufReader<File>) -> Vec<InputSequence> {
    let mut sequences = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");

        sequences.push(InputSequence(
            line.split_ascii_whitespace()
                .map(|str| str.parse().expect("Num parse failed"))
                .collect(),
        ));
    }

    sequences
}
