use std::{
    fs::File,
    io::{prelude::*, BufReader},
    collections::{HashMap},
    cmp::{min,max},
};

fn part1() {
    let buf = BufReader::new(File::open("input.txt").unwrap());
    let mut cells = HashMap::new();

    for line in buf.lines() {
        let linestr = line.unwrap();
        let parts:Vec<&str> = linestr.split(" -> ").collect();
        let mut split_start = parts[0].split(",");
        let start:(usize,usize) = (split_start.next().unwrap().parse().unwrap(), split_start.next().unwrap().parse().unwrap());
        let mut split_end = parts[1].split(",");
        let end:(usize,usize) = (split_end.next().unwrap().parse().unwrap(), split_end.next().unwrap().parse().unwrap());
        if start.0 == end.0 {
            // vertical
            for y in min(start.1, end.1)..=max(start.1, end.1) {
                *cells.entry((start.0, y)).or_insert(0) += 1;
            }
        } else if start.1 == end.1 {
            // horizontal
            for x in min(start.0, end.0)..=max(start.0, end.0) {
                *cells.entry((x, start.1)).or_insert(0) += 1;
            }
        }
    }
    let mut dangerous = 0;
    for (_,v) in cells {
        if v > 1 {
            dangerous += 1;
        }
    }
    println!("dangerous count = {}", dangerous);
}

fn part2() {
    let buf = BufReader::new(File::open("input.txt").unwrap());
    let mut cells = HashMap::new();

    for line in buf.lines() {
        let linestr = line.unwrap();
        let parts:Vec<&str> = linestr.split(" -> ").collect();
        let mut split_start = parts[0].split(",");
        let start:(isize,isize) = (split_start.next().unwrap().parse().unwrap(), split_start.next().unwrap().parse().unwrap());
        let mut split_end = parts[1].split(",");
        let end:(isize,isize) = (split_end.next().unwrap().parse().unwrap(), split_end.next().unwrap().parse().unwrap());
        if start.0 == end.0 {
            // vertical
            for y in min(start.1, end.1)..=max(start.1, end.1) {
                *cells.entry((start.0, y)).or_insert(0) += 1;
            }
        } else if start.1 == end.1 {
            // horizontal
            for x in min(start.0, end.0)..=max(start.0, end.0) {
                *cells.entry((x, start.1)).or_insert(0) += 1;
            }
        } else {
            // diagonal
            let delta = (end.0 - start.0).abs();
            let slope = ((end.0 - start.0)/delta, (end.1 - start.1)/delta);
            for s in 0..=delta {
                let x = start.0 + (s * slope.0);
                let y = start.1 + (s * slope.1);
                *cells.entry((x, y)).or_insert(0) += 1;
            }       
        }
    }
    let mut dangerous = 0;
    for (_,v) in cells {
        if v > 1 {
            dangerous += 1;
        }
    }
    println!("dangerous count = {}", dangerous); 
}

fn main() {
    part1();
    part2();
}
