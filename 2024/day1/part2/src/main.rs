use std::{
    collections::HashMap,
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

    let mut r_map = HashMap::new();

    for r_num in r_nums {
        r_map
            .entry(r_num)
            .and_modify(|num| *num += 1)
            .or_insert(1_isize);
    }

    let sum: isize = l_nums
        .iter()
        .map(|l_num| l_num * r_map.get(l_num).unwrap_or(&0))
        .sum();

    println!("Sum: {sum}");
}
