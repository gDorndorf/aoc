use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    i: i32,
    j: i32,
    risk: i64,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.risk.cmp(&self.risk)
        .then_with(|| self.i.cmp(&other.i)
        .then_with(|| self.j.cmp(&other.j)))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    println!("chiton!");
    let file = File::open("input.txt").expect("file not found");
    let reader = BufReader::new(file);

    let mut cave: Vec<Vec<i64>> = reader.lines()
        .map(|x| x.unwrap().chars()
                .map(|y| y.to_digit(10).unwrap() as i64).collect())
        .collect();

    
    // part 2 
    for i in 1..5 {
        for line in 0..cave[0].len() {
            cave.push(
                cave[line].iter().map(|x| (*x-1 + i) % 9 + 1)
                    .collect(),
            );
        }
    }
    for line in &mut cave {
        let l = line.len();
        for i in 1..5 {
            for j in 0..l {
                line.push((line[j] - 1 + i) % 9 +1);
            }
        }
    }
    // eo p2

    let d = cave.len();
    println!("cave len: {}", d);

    let mut risks: Vec<Vec<i64>> = vec![vec![i64::MAX; d]; d];
    let mut heap = BinaryHeap::new();
    let directions = [(1,0), (0,1), (-1i32,0), (0,-1i32)];

    risks[0][0] = 0;    // distances
    heap.push(Node {i:0, j:0, risk:0});  // start at node 0

    while let Some(Node {i, j, risk}) = heap.pop() {
        if risk > risks[i as usize][j as usize] {continue;}
        if i as usize == d && j as usize == d {break;}
        for p in directions {
            if (    p.0 != -1 || i != 0) 
                && (p.1 != -1 || j != 0) 
                && i+p.0 < d as i32  
                && j+p.1 < d as i32{
                let neighbor = Node {i: i + p.0, j: j + p.1 , risk: risk + cave[(i + p.0) as usize][(j+p.1) as usize]};
            
                if neighbor.risk < risks[neighbor.i as usize][neighbor.j as usize] {
                    heap.push(neighbor);
                    risks[neighbor.i as usize][neighbor.j as usize] = neighbor.risk;
                }
            }
        }
    }

    println!("minimum risk path: {}", risks[d-1][d-1]);
}

