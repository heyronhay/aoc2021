use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

fn part1() {
    let buf = BufReader::new(File::open("input.txt").unwrap());
    let mut horizontal:i32 = 0;
    let mut depth:i32 = 0;
    let mut parts:Vec<String>;
    let mut line_string:String;
    for line in buf.lines() {
        line_string = line.expect("unable to read line");
        parts = line_string.split(" ").map(String::from).collect();
        let first_char = parts[0].chars().nth(0).expect("no first char");
        let num = parts[1].parse::<i32>().unwrap();
        match first_char {
            'f' => horizontal += num,
            'u' => depth -= num,
            'd' => depth += num,
            _ => println!("Unknown command!")
        }
    }
    println!("Total = {}", horizontal * depth);
}

fn part2() {
    let buf = BufReader::new(File::open("input.txt").unwrap());
    let mut horizontal:i32 = 0;
    let mut aim:i32 = 0;
    let mut depth:i32 = 0;
    let mut parts:Vec<String>;
    let mut line_string:String;
    for line in buf.lines() {
        line_string = line.expect("unable to read line");
        parts = line_string.split(" ").map(String::from).collect();
        let first_char = parts[0].chars().nth(0).expect("no first char");
        let num = parts[1].parse::<i32>().unwrap();
        match first_char {
            'f' => {
                    horizontal += num;
                    depth += aim * num;
                },
            'u' => aim -= num,
            'd' => aim += num,
            _ => println!("Unknown command!")
        }
    }
    println!("Total = {}", horizontal * depth);
}

fn main() {
    part1();
    part2();
}
