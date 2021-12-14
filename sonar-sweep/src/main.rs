use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    println!("sonar sweep!");
    let file = File::open("../input.txt").expect("file not found");
    let reader = BufReader::new(file);

    let lines: Vec<i64> = reader.lines()
        .map(|x| x.unwrap().parse().unwrap_or(-1))
        .collect();

    
    println!("{}", count_increases(&lines));

    let windows = lines.windows(3);
    let sliding_measurement: Vec<i64> = windows.map(|x| x.iter().sum()).collect();

    println!("{}", count_increases(&sliding_measurement));
}

fn count_increases(measurements: &Vec<i64>) -> i64{
    let mut last_depth = -1;
    let mut count = -1;     // correct for +1 count as last depth starts at -1

    for &depth in measurements {
        if depth > last_depth {
            count += 1;
        }
        last_depth = depth;
    }
    return count;
}