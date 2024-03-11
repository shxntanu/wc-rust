# Word Counter

Word Counter implementation in Rust.

> [Challenge](https://codingchallenges.fyi/challenges/challenge-wc)

## Usage

```sh
cargo run -- <flag> <filename>
```

or 

```sh
cargo run <file>

```

## Example

```sh
> cargo run src/test.txt

$ Finished dev [unoptimized + debuginfo] target(s) in 0.22s
     Running `target/debug/ccwc src/test.txt`
Total number of lines in the file are: 17
```

```sh
> cargo run -- -l src/test.txt

$ Compiling ccwc v0.1.0 (/Users/shantanu/Desktop/Rust/ccwc)
    Finished dev [unoptimized + debuginfo] target(s) in 0.93s
     Running `target/debug/ccwc -l src/test.txt`
Total number of lines in the file are: 17
```
