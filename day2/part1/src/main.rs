use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    //if let Ok(lines) = read_lines("./sample.txt") {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut pos = 0;
        let mut depth = 0;

        for line in lines {
            if let Ok(l) = line {
                let v: Vec<&str> = l.split(' ').collect();
                let d = v[1].parse::<i32>().unwrap();

                println!("{} {}", v[0], v[1]);
                if v[0] == "forward" {
                    pos += d;
                } else if v[0] == "up" {
                    depth -= d;
                } else if v[0] == "down" {
                    depth += d;
                }
            }
        }
        println!("depth: {}", depth);
        println!("pos: {}", pos);
        println!("depth * pos: {}", depth * pos);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
