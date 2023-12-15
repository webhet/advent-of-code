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

    let line = reader
        .lines()
        .next()
        .expect("Expected one line")
        .expect("Error reading line");
    let strings = line.split(',');

    let mut boxes = std::iter::repeat(Vec::new()).take(256).collect::<Vec<_>>();

    strings.for_each(|s| add_string_to_boxes(s, &mut boxes));

    let sum = get_lens_sum(&boxes);

    println!("Sum {sum}");
}

fn get_lens_sum(boxes: &[Vec<(String, usize)>]) -> usize {
    let mut sum = 0;

    for (box_idx, b) in boxes.iter().enumerate() {
        for (slot_idx, (_, focal_length)) in b.iter().enumerate() {
            sum += (box_idx + 1) * (slot_idx + 1) * focal_length;
        }
    }

    sum
}

fn add_string_to_boxes(string: &str, boxes: &mut [Vec<(String, usize)>]) {
    let is_remove_operation = string.contains('-');

    let (label, focal_length) = if is_remove_operation {
        (&string[0..(string.len() - 1)], 0_usize)
    } else {
        let (label, num_part) = string.split_once('=').expect("= split failed");
        (label, num_part.parse().expect("NUm parse failed"))
    };

    let box_num = hash_string(label);

    if is_remove_operation {
        boxes[box_num].retain(|(l, _)| l != label);
    } else {
        let exisiting_lens = boxes[box_num].iter_mut().find(|(l, _)| l == label);

        if let Some(exisiting_lens) = exisiting_lens {
            exisiting_lens.1 = focal_length;
        } else {
            boxes[box_num].push((label.to_owned(), focal_length));
        }
    }
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
