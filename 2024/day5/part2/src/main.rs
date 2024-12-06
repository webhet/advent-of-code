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

    let incorrect_pages = pages
        .into_iter()
        .filter(|page| !is_page_safe(&page, &rules))
        .collect::<Vec<_>>();

    let mut sum: isize = 0;

    for mut page in incorrect_pages {
        reorder_slice(&mut page, &rules);

        sum += page[page.len() / 2];
    }

    println!("Sum: {sum}");
}

fn is_page_safe(page: &[isize], rules: &HashSet<(isize, isize)>) -> bool {
    let mut safe = true;

    'outer: for (l_idx, l) in page[..page.len() - 1].iter().enumerate() {
        for r in page[l_idx + 1..].iter() {
            if rules.contains(&(*r, *l)) {
                safe = false;
                break 'outer;
            }
        }
    }

    safe
}

fn reorder_slice(page_slice: &mut [isize], rules: &HashSet<(isize, isize)>) {
    for l_idx in 0..page_slice.len() - 1 {
        for r_idx in l_idx + 1..page_slice.len() {
            if rules.contains(&(page_slice[r_idx], page_slice[l_idx])) {
                // swap
                let tmp = page_slice[l_idx];
                page_slice[l_idx] = page_slice[r_idx];
                page_slice[r_idx] = tmp;
            }
        }
    }
}
