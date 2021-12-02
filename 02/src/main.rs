use std::fmt::{Display, Formatter, Result};
use std::fs::File;
use std::io::{self,BufRead};
use std::path::Path;

struct Sub {
    aim: i32,
    depth: i32,
    horizontal_position: i32,
}

impl Sub {
    pub fn helm(&mut self, cmd: &str, units: i32) -> () {
        match cmd {
            "down" => self.down(units),
            "forward" => self.fwd(units),
            "up" => self.up(units),
            _ => (),
        }
    }

    fn down(&mut self, units: i32) -> () {
        self.aim += units;
    }

    fn fwd(&mut self, units: i32) -> () {
        self.horizontal_position += units;
        self.depth += self.aim * units;
    }

    fn up(&mut self, units: i32) -> () {
        self.aim -= units;
    }
}

impl Display for Sub {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut Formatter) -> Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", self.depth * self.horizontal_position)
    }
}

fn main() {
    let mut sub = Sub {aim: 0, depth: 0, horizontal_position: 0};
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(control_input) = line {
                let mut iter_split = control_input.split_whitespace();
                let cmd = iter_split.next().unwrap();
                let units = iter_split.next().unwrap().parse::<i32>().unwrap();
                sub.helm(cmd, units);
            }
        }
    }

    println!("{}", sub);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
