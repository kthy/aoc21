use std::fs::File;
use std::io::{self,BufRead};
use std::path::Path;

fn main() {
    let mut counter = 0;
    let mut last_depth = i32::MAX;

    if let Ok(lines) = read_lines("./input/depths.txt") {
        for line in lines {
            if let Ok(depth_as_str) = line {
                let depth = depth_as_str.parse::<i32>().unwrap();
                if depth > last_depth {
                    counter += 1;
                }
                last_depth = depth;
            }
        }
    }

    println!("{}", counter);
}

// Reading lines in separate function because if done directly in main(), this code results in
// error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option`.
// <https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html>
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
