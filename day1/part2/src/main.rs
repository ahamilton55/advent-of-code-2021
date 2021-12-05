use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    //if let Ok(lines) = read_lines("./sample.txt") {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut fp = -1;
        let mut sp = -1;
        let mut prev_sum = -1;
        let mut count = 0;

        for line in lines {
            if let Ok(l) = line {
                let d = l.parse::<i32>().unwrap();

                let sum = sp + fp + d;

                println!("{} + {} + {} = {}", sp, fp, d, sum);
                if sum > prev_sum && prev_sum > 0 {
                    println!("{} > {}", sum, prev_sum);
                    count += 1;
                }

                if sp > 0 {
                    prev_sum = sum;
                }

                sp = fp;
                fp = d;
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
