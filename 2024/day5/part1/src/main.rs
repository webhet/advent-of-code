use std::{
    collections::HashSet,
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

    let mut rules = HashSet::new();
    let mut pages = Vec::new();
    let mut reading_rules = true;

    for line in reader.lines() {
        let line = line.expect("Failed to read line");

        if line.is_empty() {
            reading_rules = false;
            continue;
        }

        if reading_rules {
            let (num1, num2) = line.split_once("|").expect("Failed to split");

            rules.insert((
                num1.parse::<isize>().expect("Failed to parse num"),
                num2.parse::<isize>().expect("Failed to parse num"),
            ));
        } else {
            let page = line
                .split(",")
                .into_iter()
                .map(|num_str| num_str.parse::<isize>().expect("Failed to parse num"))
                .collect::<Vec<_>>();

            pages.push(page);
        }
    }

    let mut sum: isize = 0;

    for page in pages {
        let mut safe = true;

        'outer: for (l_idx, l) in page[..page.len() - 1].iter().enumerate() {
            for r in page[l_idx + 1..].iter() {
                if rules.contains(&(*r, *l)) {
                    safe = false;
                    break 'outer;
                }
            }
        }

        if safe {
            sum += page[page.len() / 2];
        }
    }

    println!("Sum: {sum}");
}
