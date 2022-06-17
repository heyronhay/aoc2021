use std::{
    fs::File,
    io::{prelude::*, BufReader},
    collections::HashMap,
};

fn part1() {
    let buf = BufReader::new(File::open("input.txt").unwrap());
    let mut stack:Vec<char> = Vec::new();
    let point_map:HashMap<char, usize> = HashMap::from([
        (')', 3),
        (']', 57),
        ('}', 1197),
        ('>', 25137),
    ]);
    let close_map:HashMap<char, char> = HashMap::from([
        (')', '('),
        (']', '['),
        ('}', '{'),
        ('>', '<'),
    ]);

    let mut error_total = 0;
    for line in buf.lines() {
        let linestr = line.unwrap();
        let rowvec:Vec<char> = linestr.chars().collect();
        for c in rowvec {
            match c {
                '{' | '(' | '[' | '<' => stack.push(c),
                '}' | ')' | ']' | '>' => {
                    match stack.pop() {
                        Some(a) => {
                            if a == close_map[&c] {
                                continue
                            }
                        },
                        None => ()
                    }
                    error_total += point_map[&c];
                    break;
                },
                _ => println!("What is this {}", c)
            }
        }
    }
    println!("error total = {}", error_total);
}

fn part2() {
    let buf = BufReader::new(File::open("input.txt").unwrap());
    let point_map:HashMap<char, usize> = HashMap::from([
        ('(', 1),
        ('[', 2),
        ('{', 3),
        ('<', 4),
    ]);
    let close_map:HashMap<char, char> = HashMap::from([
        (')', '('),
        (']', '['),
        ('}', '{'),
        ('>', '<'),
    ]);
    let mut scores:Vec<usize> = Vec::new();
    for line in buf.lines() {
        let linestr = line.unwrap();
        let rowvec:Vec<char> = linestr.chars().collect();
        let mut corrupt = false;
        let mut stack:Vec<char> = Vec::new();
        for c in rowvec {
            match c {
                '{' | '(' | '[' | '<' => stack.push(c),
                '}' | ')' | ']' | '>' => {
                    match stack.pop() {
                        Some(a) => {
                            if a == close_map[&c] {
                                continue
                            }
                        },
                        None => ()
                    }
                    corrupt = true;
                    break;
                },
                _ => println!("What is this {}", c)
            }
        }
        if !corrupt {
            print!("{:?}", stack);
            let mut score = 0;
            stack.reverse();
            for v in stack {
                score = score * 5 + point_map[&v];
            }
            println!(" : {}", score);
            scores.push(score);
        }
    }
    scores.sort();
    let total:usize = scores[scores.len() / 2];
    println!("close total = {}", total);
}

fn main() {
    part1();
    part2();
}
