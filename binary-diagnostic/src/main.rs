use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    println!("dive!");
    let file = File::open("input.txt").expect("file not found");
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().map(|x| x.unwrap()).collect();

    let length = lines[0].len();

    let mut one_count: Vec<i32> = vec![0; length]; // vector to count how many ones a position has
    let mut zero_count: Vec<i32> = vec![0; length]; // vector to count how many ones a position has

    println!("lines:  {:?}", lines.len());

    let mut all_bits: Vec<Vec<u32>> = vec![];
    for line in lines {
        let bits: Vec<u32> = line.chars().map(|x| x.to_digit(10).unwrap()).collect();
        for i in 0 .. bits.len(){
            one_count[i] += bits[i] as i32;
            zero_count[i] += 1 - (bits[i] as i32);
        }
        all_bits.push(bits);
    }

    // part 1
    let gamma_b: Vec<u32> = one_count.iter()
        .zip(zero_count.iter())
        .map(|(o, z)|  match o>=z { true => 1, false => 0} )
        .collect();
    let epsilon_b: Vec<u32> = one_count.iter()
        .zip(zero_count.iter())
        .map(|(o, z)|  match o>=z { true => 0, false => 1} )
        .collect();

    
    let gamma = to_dec(&gamma_b);
    let epsilon = to_dec(&epsilon_b);

    println!("{:?}", gamma_b);
    println!("gamma: {}", gamma);
    println!("eps: {}", epsilon);
    println!("{:?}", epsilon*gamma);

    // part 2
    let mut oxy_filter = all_bits.clone();
    let mut co2_filter = all_bits.clone();
    let mut oxy_b = gamma_b.clone();
    let mut co2_b = epsilon_b.clone();
    let mut oxygen = vec![0; length];
    let mut co2 = vec![0; length];

    for i in 0..length{
        oxy_filter = oxy_filter.into_iter().filter(|x| x[i] == oxy_b[i]).collect();
        co2_filter = co2_filter.into_iter().filter(|x| x[i] == co2_b[i]).collect();

        if !oxy_filter.is_empty() {
            oxygen = oxy_filter[0].clone();
            one_count = vec![0; length];
            zero_count = vec![0; length];
            for bits in &oxy_filter {
                for i in 0 .. bits.len(){
                    one_count[i] += bits[i] as i32;
                    zero_count[i] += 1 - (bits[i] as i32);
                }
            }
            oxy_b = one_count.iter()
                .zip(zero_count.iter())
                .map(|(o, z)|  match o>=z { true => 1, false => 0} )
                .collect();
        }
        if co2_filter.len() > 0 {
            co2 = co2_filter[0].clone();
            one_count = vec![0; length];
            zero_count = vec![0; length];
            for bits in &co2_filter {
                for i in 0 .. bits.len(){
                    one_count[i] += bits[i] as i32;
                    zero_count[i] += 1 - (bits[i] as i32);
                }
            } 
            co2_b = one_count.iter()
                .zip(zero_count.iter())
                .map(|(o, z)|  match o>=z { true => 0, false => 1} )
                .collect();
        } 
    }

    println!("life sus rating: {}", to_dec(&oxygen)*to_dec(&co2));
}

fn to_dec(bits: &Vec<u32>) -> u32{
    let base:i32 = 2;
    let length = bits.len();
    let mut result = 0;
    for i in 0 .. length {
        let power = base.pow(((length-1)-i) as u32) as u32;
        result += bits[i] * power;
    }
    return result;
}