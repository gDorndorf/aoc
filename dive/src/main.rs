use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    println!("dive!");
    let file = File::open("input.txt").expect("file not found");
    let reader = BufReader::new(file);

    let lines = reader.lines()
        .map(|x| x.unwrap());
    
    let mut hor_p = 0;
    let mut depth = 0;
    
    for line in lines {
        let command: Vec<String> = line.split(' ').
            map(|x| x.to_string())
            .collect();
        let amount: i64 = command[1].parse().unwrap();
        match command[0].as_str() {
            "forward" => hor_p += amount,
            "down" => depth += amount,
            "up" => depth -= amount,
            _ => panic!("Command {} is not understood.", command[0].as_str()),
        };
    }
    println!("{}", depth * hor_p);
}
