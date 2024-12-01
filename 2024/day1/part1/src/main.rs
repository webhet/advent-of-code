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

    let mut l_nums = Vec::new();
    let mut r_nums = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");

        let (l_num, r_num) = line.as_str().split_once("   ").expect("Failed to split");

        l_nums.push(l_num.parse::<isize>().expect("Failed to parse num"));
        r_nums.push(r_num.parse::<isize>().expect("Failed to parse num"));
    }

    l_nums.sort();
    r_nums.sort();

    let sum: usize = l_nums
        .into_iter()
        .zip(r_nums.into_iter())
        .map(|(l, r)| l.abs_diff(r))
        .sum();

    println!("Sum {sum}")
}
