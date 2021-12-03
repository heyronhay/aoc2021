use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

fn part1() {
    let buf = BufReader::new(File::open("input.txt").unwrap());
    let mut gamma:usize = 0;
    let mut epsilon:usize = 0;
    let mut one_bits_count:[isize;32] = [0;32];
    let mut bit_pos:usize = 0;
    for line in buf.lines() {
        bit_pos = 0;
        for bit in line.unwrap().chars() {
            if bit == '1' {
                one_bits_count[bit_pos] += 1;
            } else {
                one_bits_count[bit_pos] -= 1;
            }
            bit_pos += 1;
        }
    }

    for index in 0..bit_pos {
        gamma <<= 1;
        epsilon <<= 1;
        if one_bits_count[index] > 0 {
            gamma += 1;
        } else {
            epsilon += 1;
        }
    }

    println!("gamma = {}, epsilon = {}, power = {}", gamma, epsilon, gamma * epsilon);
}

fn part2() {
    let buf = BufReader::new(File::open("input.txt").unwrap());
    const NUM_BITS:usize = 12;
    const NUM_BUCKETS:usize = (1 << (NUM_BITS + 1)) - 1;
    let mut flat_tree:[isize;NUM_BUCKETS] = [0;NUM_BUCKETS];
    let mut bucket_pos:usize;
    for line in buf.lines() {
        bucket_pos = 1;
        for bit in line.unwrap().chars() {
            if bit == '0' {
                flat_tree[bucket_pos] += 1;
                bucket_pos = bucket_pos*2 + 1;
            } else {
                bucket_pos += 1;
                flat_tree[bucket_pos] += 1;
                bucket_pos = bucket_pos*2 + 1;
            }
        }
    }
    println!("buckets = {:?}", flat_tree);

    let mut o2 = 0;
    let mut o2_pos = 1;
    let mut co2 = 0;
    let mut co2_pos = 1;
    for _ in 0..NUM_BITS {
        o2 <<= 1;
        if flat_tree[o2_pos+1] >= flat_tree[o2_pos] {
            o2 += 1;
            o2_pos += 1;
        }
        o2_pos = o2_pos*2 + 1;

        co2 <<= 1;
        println!("0 = {}, 1 = {}", flat_tree[co2_pos], flat_tree[co2_pos+1]);
        if flat_tree[co2_pos] == 0 || (flat_tree[co2_pos+1] != 0 && flat_tree[co2_pos+1] < flat_tree[co2_pos]) {
            co2 += 1;
            co2_pos += 1;
        }
        co2_pos = co2_pos*2 + 1;
    }
    println!("o2 = {} - {:05b}, co2 = {} - {:05b}, life support = {}", o2, o2, co2, co2, o2*co2);
}

fn main() {
    part1();
    part2();
}
