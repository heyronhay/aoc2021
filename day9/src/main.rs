use std::{
    fs::File,
    io::{prelude::*, BufReader},
    collections::HashMap,
};

fn pt_to_index(x:usize, y:usize, size:usize) -> usize {
    return (y * size) + x;
}

fn part1() {
    let buf = BufReader::new(File::open("input.txt").unwrap());
    let mut flatgrid:Vec<u32> = Vec::new();
    let mut w = 0;
    let mut h = 0;
    for line in buf.lines() {
        let linestr = line.unwrap();
        let rowvec:Vec<u32> = linestr.chars().map(|c| c.to_digit(10).unwrap()).collect();
        w = rowvec.len();
        flatgrid.extend(&rowvec);
        h += 1;
    }

    let mut total = 0;
    for row in 0..h {
        for col in 0..w {
            let val = flatgrid[pt_to_index(col,row,w)];
            if row != 0 {
                if val >= flatgrid[pt_to_index(col,row-1,w)] {
                    continue;
                }
            }
            if row != (h - 1) {
                if val >= flatgrid[pt_to_index(col,row+1,w)] {
                    continue;
                }
            }
            if col != 0 {
                if val >= flatgrid[pt_to_index(col-1,row,w)] {
                    continue;
                }
            }
            if col != (w - 1) {
                if val >= flatgrid[pt_to_index(col+1,row,w)] {
                    continue;
                }
            }
            total += 1 + val;
        }
    }
    println!("total = {}", total);
}

fn part2() {
    let buf = BufReader::new(File::open("input.txt").unwrap());
    let mut flatgrid:Vec<u32> = Vec::new();
    let mut w = 0;
    let mut h = 0;
    for line in buf.lines() {
        let linestr = line.unwrap();
        let rowvec:Vec<u32> = linestr.chars().map(|c| c.to_digit(10).unwrap()).collect();
        w = rowvec.len();
        flatgrid.extend(&rowvec);
        h += 1;
    }

    let mut last_val = 9;
    let mut curr_basin = -1;
    let mut flat_basin_grid:Vec<i32> = vec![-1;flatgrid.len()];
    let mut basin_map:HashMap<i32, i32> = HashMap::new();
    for row in 0..h {
        for col in 0..w {
            let val = flatgrid[pt_to_index(col,row,w)];
            if val != 9 {
                if last_val == 9 {
                    curr_basin += 1;
                }
                flat_basin_grid[pt_to_index(col,row,w)] = curr_basin;
            }
            last_val = val;
            print!("{:5}", flat_basin_grid[pt_to_index(col,row,w)]);
        }
        println!();
        last_val = 9;
    }
    println!();
    for col in 0..w {
        print!("{:5}", flat_basin_grid[pt_to_index(col,0,w)]);
    }
    println!();
    for row in 1..h {
        for col in 0..w {
            let basin_val = flat_basin_grid[pt_to_index(col,row,w)];
            let mut basin_val_above = flat_basin_grid[pt_to_index(col,row-1,w)];
            if basin_val != -1 {
                if basin_val_above != -1 {
                    if basin_map.contains_key(&basin_val_above) {
                        basin_val_above = basin_map[&basin_val_above];
                    }
                    basin_map.insert(basin_val, basin_val_above);
                    flat_basin_grid[pt_to_index(col,row,w)] = basin_val_above;
                }
                if col != 0 {
                    let basin_curr_val = flat_basin_grid[pt_to_index(col,row,w)];
                    let basin_left = flat_basin_grid[pt_to_index(col-1,row,w)];
                    if basin_left != basin_curr_val && !basin_map.contains_key(&basin_left) && !basin_map.contains_key(&basin_curr_val) {
                        basin_map.insert(basin_left, basin_curr_val);
                    }
                }
            }
            print!("{:5}", flat_basin_grid[pt_to_index(col,row,w)]);
        }
        println!();
    }
    println!();

    let mut basin_vec_map:HashMap<i32,usize> = HashMap::new();
    for row in 0..h {
        for col in 0..w {
            let mut basin_val = flat_basin_grid[pt_to_index(col,row,w)];
            if basin_val != -1 {
                while basin_map.contains_key(&basin_val) {
                    println!("basin_val = {}", basin_val);
                    basin_val = basin_map[&basin_val];
                }
                flat_basin_grid[pt_to_index(col,row,w)] = basin_val;
                *basin_vec_map.entry(flat_basin_grid[pt_to_index(col,row,w)]).or_insert(0) += 1;
            }
            print!("{:5}", flat_basin_grid[pt_to_index(col,row,w)]);
        }
        println!();
    }
    let mut basin_lengths:Vec<usize> = basin_vec_map.values().cloned().collect();
    basin_lengths.sort();
    basin_lengths.reverse();
    println!("basins = {:?}", basin_map);
    println!("basin length total = {}", basin_lengths[0] * basin_lengths[1] * basin_lengths[2]);
}

fn main() {
    part1();
    part2();
}
