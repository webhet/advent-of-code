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

    let line = reader
        .lines()
        .next()
        .expect("Expected one line")
        .expect("Error reading line");
    let strings = line.split(',');

    let sum: usize = strings.map(hash_string).sum();

    println!("Sum {sum}");
}

fn hash_string(string: &str) -> usize {
    let mut val = 0;

    for c in string.chars() {
        val += c as usize;
        val *= 17;
        val %= 256;
    }

    val
}
