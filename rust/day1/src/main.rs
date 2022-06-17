use std::{
    fs::File,
    io::{prelude::*, BufReader, Error, ErrorKind},
    path::Path,
};

fn part1(readings : &std::vec::Vec<i64>) {
    let mut last:&i64 = &readings[0];
    let mut count:i64 = 0;
    for reading in &readings[1..] {
        if reading > last {
            count += 1;
        }
        last = reading;
    }
    println!("total increase = {}", count);
}

fn part2(readings : &std::vec::Vec<i64>) {
    let mut last_window:i64 = readings[0] + readings[1] + readings[2];
    let mut count:i64 = 0;
    for index in 1..readings.len()-2 {
        let curr_window = last_window - readings[index-1] + readings[index+2];
        if curr_window > last_window {
            count += 1;
        }
        last_window = curr_window;
    }
    println!("total increase = {}", count);
}

fn main() {
    let filename = Path::new("input.txt");
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    let lines:Result<Vec<i64>, Error> = buf.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .into_iter()
        .collect();

    let readings = lines.unwrap();

    part1(&readings);
    part2(&readings);
}
