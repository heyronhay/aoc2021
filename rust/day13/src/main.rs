use std::{
    fs::File,
    io::{prelude::*, BufReader},
    collections::{HashSet},
    cmp::max,
};


fn part1() {
    let buf = BufReader::new(File::open("input.txt").unwrap());
    let mut read_points = true;
    let mut points:HashSet<(usize,usize)> = HashSet::new();
    let mut folds:Vec<(char,usize)> = Vec::new();

    let mut w = 0;
    let mut h = 0;
    for line in buf.lines() {
        let linestr = line.unwrap();
        if linestr.len() == 0 {
            read_points = false;
        } else if read_points {
            let xy:Vec<usize> = linestr.split(",").map(|v| v.parse().unwrap()).collect();
            points.insert((xy[0],xy[1]));
            w = max(w,xy[0]);
            h = max(h,xy[1]);
        } else {
            let parts:Vec<_> = linestr["fold along ".len()..].split("=").collect();
            folds.push((parts[0].chars().nth(0).unwrap(),parts[1].parse().unwrap()));
        }
    }

    let mut next_points = points.clone();
    for fold in folds {
        let curr_points = next_points.clone();
        next_points.clear();
        for point in curr_points {
            if fold.0 == 'x' {
                if point.0 > fold.1 {
                    next_points.insert((2*fold.1 - point.0, point.1));
                } else {
                    next_points.insert((point.0, point.1));
                }
            } else {
                if point.1 > fold.1 {
                    next_points.insert((point.0, 2*fold.1 - point.1));
                } else {
                    next_points.insert((point.0, point.1));
                }
            }
        }
    }
    for y in 0..h {
        for x in 0..w {
            if next_points.contains(&(x,y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn part2() {

}

fn main() {
    part1();
    part2();
}
