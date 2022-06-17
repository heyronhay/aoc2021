use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

fn calc_num_fish(days:usize) {
    let buf = BufReader::new(File::open("input.txt").unwrap());
    let mut orig_fish:Vec<usize> = vec![0; 7];
    let mut new_fish:Vec<usize> = vec![0; 9];
    let mut orig_zero_day = 0;
    let mut new_zero_day = 0;
    let mut iter = buf.lines();

    let start_line = iter.next().unwrap().unwrap();
    let start_vals:Vec<usize> = start_line.split(",").map(|l| l.parse().unwrap()).collect();

    for val in start_vals {
        orig_fish[val] += 1;
    }

    let mut num_new_fish;
    let mut num_new_orig_fish;
    let mut old_new_zero_day;
    for _ in 0..days {
        num_new_fish = orig_fish[orig_zero_day] + new_fish[new_zero_day];
        num_new_orig_fish = new_fish[new_zero_day];
        orig_fish[orig_zero_day] += num_new_orig_fish;
        new_fish[new_zero_day] = 0;
        old_new_zero_day = new_zero_day;
        orig_zero_day = (orig_zero_day + 1) % 7;
        new_zero_day = (new_zero_day + 1) % 9;
        new_fish[old_new_zero_day] += num_new_fish;
    }

    

}
//fn part1() {
//    calc_num_fish(80);
//}

fn part2() {
    const LOOPS:u32 = 100000;
    use std::time::Instant;
    let now = Instant::now();
    for _ in 0..LOOPS {
        calc_num_fish(256);
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed / LOOPS);
}

fn main() {
    //part1();
    part2();
}
