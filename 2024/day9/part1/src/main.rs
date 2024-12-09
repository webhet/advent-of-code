use std::{env, fs};

fn main() {
    let Some(filepath) = env::args().nth(1) else {
        println!("Usage: part1 <filepath>");
        return;
    };

    let input = fs::read_to_string(filepath).expect("Should have been able to read the file");

    let mut disk = Vec::new();

    let mut file_idx: usize = 0;

    for (idx, c) in input.chars().enumerate() {
        let len = c.to_digit(10).expect("Failed to parse num") as usize;

        if idx % 2 == 0 {
            // is file
            for _ in 0..len {
                disk.push(file_idx as isize);
            }

            file_idx += 1;
        } else {
            // free space
            for _ in 0..len {
                disk.push(-1);
            }
        }
    }

    let mut l_idx = 0_usize;
    let mut r_idx = disk.len() - 1;

    loop {
        if r_idx <= l_idx {
            break;
        }

        if disk[l_idx] < 0 {
            // free space
            // move something here

            loop {
                if disk[r_idx] >= 0 {
                    disk[l_idx] = disk[r_idx];
                    disk[r_idx] = -1;

                    r_idx -= 1;
                    break;
                }

                r_idx -= 1;
            }
        }

        l_idx += 1;
    }

    let checksum: usize = disk
        .iter()
        .enumerate()
        .map(|(idx, file_idx)| {
            if *file_idx < 0 {
                0
            } else {
                idx * *file_idx as usize
            }
        })
        .sum();

    println!("Checksum: {checksum}")
}
