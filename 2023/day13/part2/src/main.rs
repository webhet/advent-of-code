use std::{
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

    let patterns = parse_patterns(reader);

    let sum: usize = patterns
        .into_iter()
        .map(|p| {
            let (orientation, axis_idx) = p.get_mirror_axis();

            match orientation {
                AxisOrientation::Horizontal => axis_idx * 100,
                AxisOrientation::Vertical => axis_idx,
            }
        })
        .sum();

    println!("Sum {sum}");
}

fn is_power_of_two(x: u64) -> bool {
    (x & (x - 1)) == 0
}

#[derive(Debug)]
enum AxisOrientation {
    Horizontal,
    Vertical,
}

struct Pattern {
    rows: Vec<u64>,
    columns: Vec<u64>,
}

impl Pattern {
    pub fn get_mirror_axis(&self) -> (AxisOrientation, usize) {
        for axis_idx in 1..self.rows.len() {
            let range_len = std::cmp::min(axis_idx, self.rows.len() - axis_idx);

            let range_before = &self.rows[(axis_idx - range_len)..axis_idx];
            let range_after = &self.rows[axis_idx..(axis_idx + range_len)];

            if compare_ranges(range_len, range_before, range_after) {
                return (AxisOrientation::Horizontal, axis_idx);
            }
        }

        for axis_idx in 1..self.columns.len() {
            let range_len = std::cmp::min(axis_idx, self.columns.len() - axis_idx);

            let range_before = &self.columns[(axis_idx - range_len)..axis_idx];
            let range_after = &self.columns[axis_idx..(axis_idx + range_len)];

            if compare_ranges(range_len, range_before, range_after) {
                return (AxisOrientation::Vertical, axis_idx);
            }
        }

        unreachable!()
    }
}

fn compare_ranges(range_len: usize, range_a: &[u64], range_b: &[u64]) -> bool {
    let mut had_smudge = false;

    let ranges_match = range_a.iter().enumerate().all(|(idx, a)| {
        let a = *a;
        let b = range_b[range_len - idx - 1];

        if a == b {
            return true;
        }

        if had_smudge {
            false
        } else {
            let res = a ^ b;

            if res > 0 && is_power_of_two(res) {
                had_smudge = true;
                true
            } else {
                false
            }
        }
    });

    ranges_match && had_smudge
}

fn parse_patterns(reader: BufReader<File>) -> Vec<Pattern> {
    let mut patterns = Vec::new();

    let mut rows = Vec::new();

    let mut row_len = 0;

    for line in reader.lines() {
        let line = line.expect("Failed to read line");

        if !line.is_empty() {
            row_len = line.chars().count();

            let mut num = 0;
            for (idx, c) in line.chars().enumerate() {
                if c == '#' {
                    num |= 1 << idx;
                }
            }
            rows.push(num);
        } else {
            let mut columns = Vec::new();

            for col_idx in 0..row_len {
                let mut num = 0;

                let current_pos = 1 << col_idx;

                for (row_idx, row) in rows.iter().enumerate() {
                    if row & current_pos > 0 {
                        num |= 1 << row_idx;
                    }
                }

                columns.push(num);
            }

            patterns.push(Pattern {
                rows: std::mem::take(&mut rows),
                columns,
            });
        }
    }

    let mut columns = Vec::new();

    for col_idx in 0..row_len {
        let mut num = 0;

        let current_pos = 1 << col_idx;

        for (row_idx, row) in rows.iter().enumerate() {
            if row & current_pos > 0 {
                num |= 1 << row_idx;
            }
        }

        columns.push(num);
    }

    patterns.push(Pattern {
        rows: std::mem::take(&mut rows),
        columns,
    });

    patterns
}
