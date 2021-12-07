use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

fn part1() {
    let buf = BufReader::new(File::open("input.txt").unwrap());
    let mut iter = buf.lines();

    let start_line = iter.next().unwrap().unwrap();
    let mut start_vals:Vec<i64> = start_line.split(",").map(|l| l.parse().unwrap()).collect();
    start_vals.sort();
    let start_len = start_vals.len();
    let mid = start_len / 2;
    let median:i64;
    if start_len % 2 == 0 {
        median = (start_vals[mid-1] + start_vals[mid]) / 2;
    } else {
        median = start_vals[mid];
    }
 
    let mut total = 0;
    for crab in start_vals {
        total += (crab - median).abs();
    }
    println!("Total = {}", total);
}

fn part2() {
    let buf = BufReader::new(File::open("input.txt").unwrap());
    let mut iter = buf.lines();

    let start_line = iter.next().unwrap().unwrap();
    let start_vals:Vec<i64> = start_line.split(",").map(|l| l.parse().unwrap()).collect();
    let start_len = start_vals.len() as i64;
    let sum:i64 = Iterator::sum(start_vals.iter());
    let mean_val:i64 = sum / start_len;
    println!("real mean = {:.4}",(sum as f64 / start_len as f64));
    let mut total = 0;
    let mut dist;
    for crab in start_vals {
        dist = (crab - mean_val).abs();
        total += (dist * (dist + 1)) / 2;
    }
    println!("Mean = {}, Total = {}", mean_val, total);
}

fn main() {
    part1();
    part2();
}
