use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    //if let Ok(lines) = read_lines("./sample.txt") {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut i = -1;
        let mut count = 0;
        for line in lines {
            if let Ok(l) = line {
                let d = l.parse::<i32>().unwrap();

                println!("{}, {}", d, i);
                if d > i && i > 0 {
                    count += 1;
                }

                i = d;
            }
        }
        println!("found: {}", count)
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
