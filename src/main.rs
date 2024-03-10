use std::env;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_file_size(path: &str) -> Result<u64, std::io::Error> {
    let metadata = fs::metadata(path)?;
    Ok(metadata.len())
}

fn count_words_in_file(path: &str) -> Result<u64, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut word_count = 0;
    for line in reader.lines() {
        let line = line?; // Handle potential errors from reading lines
        word_count += line.split_whitespace().count() as u64;
    }

    Ok(word_count)
}

fn count_lines_in_file(path: &str) -> Result<u64, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut line_count: u64 = 0;
    for _ in reader.lines() {
        line_count += 1;
    }

    Ok(line_count)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args.clone());

    if args.len() == 2 {
        // Default Case (only file location)
        println!(
            "Total number of lines in the file are: {}",
            count_lines_in_file(&args[1]).unwrap_or_else(|err| {
                eprintln!("Error reading file: {}", err);
                0
            })
        );
    } else if args.len() == 3 {
        // Flag and File case
        let flag = &args[1];
        let file_path = &args[2];

        if flag.len() != 2 || &flag[0..1] != "-" {
            println!("Wrong flag given!");
        } else {
            match &flag[1..2] {
                "l" => {
                    println!(
                        "Total number of lines in the file are: {}",
                        count_lines_in_file(file_path).unwrap_or_else(|err| {
                            eprintln!("Error reading file: {}", err);
                            0
                        })
                    );
                }
                "c" => {
                    println!(
                        "The number of bytes in the file are: {}",
                        get_file_size(file_path).unwrap_or_else(|err| {
                            eprintln!("Error reading file size: {}", err);
                            0
                        })
                    );
                }
                "w" => {
                    println!(
                        "The number of words in the file are: {}",
                        count_words_in_file(file_path).unwrap_or_else(|err| {
                            eprintln!("Error reading file: {}", err);
                            0
                        })
                    )
                }
                _ => {
                    // Handle any other possible values of flag
                }
            }
        }
    } else {
        println!("Wrong usage of command line argument!\nPlease use either 1 or 2 args.\n");
    }
}
