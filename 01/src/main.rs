use std::fs::File;
use std::io::{self,BufRead};
use std::path::Path;

struct SlidingWindow {
    old: i32,
    mid: i32,
    new: i32,
}

impl SlidingWindow {
    fn slide(&mut self, new_new: i32) -> () {
        self.old = self.mid;
        self.mid = self.new;
        self.new = new_new;
    }

    fn sum(&self) -> i32 {
        // Surely there must be a better way of checked_adding multiple numbers???
        let sum: i32 = match self.old.checked_add(self.mid) {
            Some(num) => {
                match num.checked_add(self.new) {
                    Some(num) => num,
                    None => i32::MAX,
                }
            },
            None => i32::MAX,
        };
        return sum;
    }
}

fn main() {
    let mut counter = 0;
    let mut sliding_counter = 0;
    let mut last_depth = i32::MAX;
    let mut sliding_window = SlidingWindow {
        old: i32::MAX, mid: i32::MAX, new: i32::MAX
    };
    let mut last_sliding_depth = i32::MAX;

    if let Ok(lines) = read_lines("./input/depths.txt") {
        for line in lines {
            if let Ok(depth_as_str) = line {
                let depth = depth_as_str.parse::<i32>().unwrap();
                if depth > last_depth {
                    counter += 1;
                }
                sliding_window.slide(depth);
                let sliding_depth = sliding_window.sum();
                if sliding_depth > last_sliding_depth {
                    sliding_counter += 1;
                }
                last_depth = depth;
                last_sliding_depth = sliding_depth;
            }
        }
    }

    println!("{} measurements are larger than the previous measurement.", counter);
    println!("{} three-measurement sliding window sums are larger than the previous sum.", sliding_counter);
}

// Reading lines in separate function because if done directly in main(), this code results in
// error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option`.
// <https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html>
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
