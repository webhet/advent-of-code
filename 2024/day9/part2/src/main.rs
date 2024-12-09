use std::{env, fs};

#[derive(Debug)]
enum FilesystemLocation {
    Free { len: usize },
    File { idx: usize, len: usize },
}

fn main() {
    let Some(filepath) = env::args().nth(1) else {
        println!("Usage: part1 <filepath>");
        return;
    };

    let input = fs::read_to_string(filepath).expect("Should have been able to read the file");

    let mut disk = Vec::new();

    let mut file_index: usize = 0;

    for (idx, c) in input.chars().enumerate() {
        let len = c.to_digit(10).expect("Failed to parse num") as usize;

        if idx % 2 == 0 {
            // is file
            disk.push(FilesystemLocation::File {
                idx: file_index,
                len,
            });

            file_index += 1;
        } else {
            // free space
            disk.push(FilesystemLocation::Free { len });
        }
    }

    let mut move_pos = 0;

    for r_idx in (0..disk.len()).rev() {
        match disk[r_idx] {
            FilesystemLocation::File {
                idx: file_idx,
                len: file_len,
            } => {
                // find next appropriate file

                let mut free_len = 0;

                for l_idx in 0..r_idx {
                    match disk[l_idx] {
                        FilesystemLocation::File { .. } => {}
                        FilesystemLocation::Free { len } => {
                            if len >= file_len {
                                move_pos = l_idx;
                                free_len = len;

                                break;
                            }
                        }
                    }
                }

                if free_len > 0 {
                    disk[move_pos] = FilesystemLocation::File {
                        idx: file_idx,
                        len: file_len,
                    };
                    disk[r_idx] = FilesystemLocation::Free { len: file_len };

                    if file_len < free_len {
                        match disk[move_pos + 1] {
                            FilesystemLocation::Free { len } => {
                                disk[move_pos + 1] = FilesystemLocation::Free {
                                    len: (free_len - file_len) + len,
                                };
                            }
                            FilesystemLocation::File { .. } => {
                                disk.insert(
                                    move_pos + 1,
                                    FilesystemLocation::Free {
                                        len: free_len - file_len,
                                    },
                                );
                            }
                        }
                    }
                }
            }
            FilesystemLocation::Free { .. } => {}
        }
    }

    let mut pos = 0;
    let mut checksum: usize = 0;

    for loc in disk {
        match loc {
            FilesystemLocation::Free { len } => {
                pos += len;
            }
            FilesystemLocation::File { idx, len } => {
                for i in pos..(pos + len) {
                    checksum += idx * i;
                }

                pos += len;
            }
        }
    }

    println!("Checksum: {checksum}")
}
