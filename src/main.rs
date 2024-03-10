use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(args.clone());

    if args.len() == 2 {
        // Default Case (only file location)
        let file_path  = &args[1];
        let file = BufReader::new(File::open(file_path).expect("Should've been able to read the file?!"));
        let mut cnt = 0;
        for _ in file.lines() {
            cnt += 1;
        }
        println!("Total number of lines in file {} are: {}", file_path, cnt);
    } else if args.len() == 3 {
        // Flag and File case
        let flag = &args[1];
        let file_path  = &args[2];

        let mut bytes = [0u8; 12];
        let file = BufReader::new(File::open(file_path).expect("Should've been able to read the file?!"));
    } else {
        println!("Wrong usage of command line argument!\nPlease use either 1 or 2 args.\n");
    }
}
