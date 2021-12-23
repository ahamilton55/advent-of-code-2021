use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    //if let Ok(lines) = read_lines("./sample.txt") {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut xs: [i32; 12]= [0; 12];
        let mut count = 0;

        for line in lines {
            if let Ok(l) = line {
                println!("{}", l);
                count += 1;
                let mut i = 0;
                for digit in  l.chars() {
                    if digit == '1' {
                        xs[i] += 1;
                    }
                    i += 1;
                }
                
            }
        }

        let mut gamma = 0;
        let mut epsilon = 0;
        let mut i:u32 = (xs.len() as u32) - 1;
        let base:i32 = 2;
        println!("count: {}", count);
        for n in xs {
            println!("n[{}]: {}", i, n);
            if n >= count/2 {
                println!("gamma");
                gamma += base.pow(i);
            } else {
                println!("epsilon");
                epsilon += base.pow(i);
            }
            if i > 0 {
                i -= 1;
            }
        }

        println!("{}", gamma);
        println!("{}", epsilon);
        println!("{}", gamma*epsilon);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
