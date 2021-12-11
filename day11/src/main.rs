use std::{
    fs::File,
    io::{prelude::*, BufReader},
    cmp::{min},
};

fn pt_to_index(x:usize, y:usize) -> usize {
    return (y * 12) + x;
}

fn part1() {
    let buf = BufReader::new(File::open("input.txt").unwrap());
    let mut flatgrid:Vec<u32> = vec!(0;12);
    for line in buf.lines() {
        let linestr = line.unwrap();
        let rowvec:Vec<u32> = linestr.chars().map(|c| c.to_digit(10).unwrap()).collect();
        flatgrid.push(0);
        flatgrid.extend(&rowvec);
        flatgrid.push(0);
    }
    flatgrid.extend(vec!(0;12));
    let mut flashes = 0;
    for step in 0..100 {
        for x in 1..11 {
            for y in 1..11 {
                flatgrid[pt_to_index(x,y)] += 1;
            }
        }

        let mut flash = true;
        while flash {
            flash = false;
            let mut deltas:Vec<(usize,usize,usize)> = Vec::new();
            for y in 1..11 {
                for x in 1..11 {
                    if flatgrid[pt_to_index(x,y)] > 0 && flatgrid[pt_to_index(x,y)] < 10 {
                        let mut delta = 0;
                        for dx in 0..3 {
                            for dy in 0..3 {
                                if flatgrid[pt_to_index(x+dx-1, y+dy-1)] == 10 {
                                    delta += 1;
                                }
                            }
                        }
                        deltas.push((x,y,delta));
                        if delta > 0 {
                            flash = true;
                        }
                    }
                }
            }
            flatgrid.iter_mut().for_each(|x| *x %= 10);
            for d in deltas {
                flatgrid[pt_to_index(d.0, d.1)] = min(10, flatgrid[pt_to_index(d.0,d.1)] + d.2 as u32);
            }
        }
        println!("\nstep {}", step+1);
        for row in 1..11 {
            for col in 1..11 {
                print!("{}", flatgrid[pt_to_index(col,row)]);
                if flatgrid[pt_to_index(col,row)] == 0 {
                    flashes += 1;
                }
            }
            println!();
        }
    }
    println!("Flashes = {}", flashes);
}

fn part2() {
    let buf = BufReader::new(File::open("input.txt").unwrap());
    let mut flatgrid:Vec<u32> = vec!(0;12);
    for line in buf.lines() {
        let linestr = line.unwrap();
        let rowvec:Vec<u32> = linestr.chars().map(|c| c.to_digit(10).unwrap()).collect();
        flatgrid.push(0);
        flatgrid.extend(&rowvec);
        flatgrid.push(0);
    }
    flatgrid.extend(vec!(0;12));
    let mut flash_count = 0;
    loop {
        flash_count += 1;
        for x in 1..11 {
            for y in 1..11 {
                flatgrid[pt_to_index(x,y)] += 1;
            }
        }

        let mut flash = true;
        while flash {
            flash = false;
            let mut deltas:Vec<(usize,usize,usize)> = Vec::new();
            for y in 1..11 {
                for x in 1..11 {
                    if flatgrid[pt_to_index(x,y)] > 0 && flatgrid[pt_to_index(x,y)] < 10 {
                        let mut delta = 0;
                        for dx in 0..3 {
                            for dy in 0..3 {
                                if flatgrid[pt_to_index(x+dx-1, y+dy-1)] == 10 {
                                    delta += 1;
                                }
                            }
                        }
                        deltas.push((x,y,delta));
                        if delta > 0 {
                            flash = true;
                        }
                    }
                }
            }
            flatgrid.iter_mut().for_each(|x| *x %= 10);
            for d in deltas {
                flatgrid[pt_to_index(d.0, d.1)] = min(10, flatgrid[pt_to_index(d.0,d.1)] + d.2 as u32);
            }
        }
        //println!("\nstep {}", step+1);

        let mut all_flashes = true;
        for row in 1..11 {
            for col in 1..11 {
                //print!("{}", flatgrid[pt_to_index(col,row)]);
                if flatgrid[pt_to_index(col,row)] != 0 {
                    all_flashes = false;
                    break;
                }
            }
           // println!();
        }
        if all_flashes {
            break;
        }
    }
    println!("count = {}", flash_count);
}

fn main() {
    part1();
    part2();
}
