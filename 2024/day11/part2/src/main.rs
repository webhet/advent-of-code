use std::{collections::HashMap, env, fs};

fn main() {
    let Some(filepath) = env::args().nth(1) else {
        println!("Usage: part1 <filepath>");
        return;
    };

    let input = fs::read_to_string(filepath).expect("Should have been able to read the file");

    let stones = input
        .split(" ")
        .into_iter()
        .map(|s| s.parse::<isize>().expect("Failed to parse num"))
        .collect::<Vec<_>>();

    let mut sum = 0;

    let mut cache = HashMap::new();

    for stone in stones {
        let val = count_blink(stone, 1, 0, &mut cache);

        cache.insert((stone, 0), val + 1);

        sum += val + 1;
    }

    println!("Stones: {sum}");
}

fn count_blink(
    stone: isize,
    acc: usize,
    depth: usize,
    cache: &mut HashMap<(isize, usize), usize>,
) -> usize {
    if depth == 75 {
        return 0;
    }

    if let Some(cached_value) = cache.get(&(stone, depth)) {
        return *cached_value;
    }

    if stone == 0 {
        return count_blink(1, acc, depth + 1, cache);
    }

    let str = stone.to_string();

    if str.len() % 2 == 0 {
        let n1 = str[0..(str.len() / 2)]
            .parse::<isize>()
            .expect("Failed to parse num");

        let n2 = str[(str.len() / 2)..]
            .parse::<isize>()
            .expect("Failed to parse num");

        let b1 = count_blink(n1, acc, depth + 1, cache);
        cache.insert((n1, depth + 1), b1);

        let b2 = count_blink(n2, acc, depth + 1, cache);
        cache.insert((n2, depth + 1), b2);

        cache.insert((stone, depth), b1 + b2 + 1);

        return b1 + b2 + 1;
    }

    let val = count_blink(stone * 2024, acc, depth + 1, cache);

    cache.insert((stone * 2024, depth + 1), val);

    val
}
