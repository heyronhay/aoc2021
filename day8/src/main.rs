use std::{
    fs::File,
    io::{prelude::*, BufReader},
    collections::HashSet,
};

fn part1() {
    let buf = BufReader::new(File::open("input.txt").unwrap());
    let mut digit_count = 0;
    for line in buf.lines() {
        let linestr = line.unwrap();
        let parts:Vec<&str> = linestr.split(" | ").collect();
        let outputs:Vec<&str> = parts[1].split(" ").collect();
        for output in outputs {
            let outlen = output.len();
            match outlen {
                2 | 3 | 4 | 7 => digit_count += 1,
                _ => (),
            }
        }
    }

    println!("Total unique digit count = {}", digit_count);
}

fn part2() {
    let buf = BufReader::new(File::open("input.txt").unwrap());
    let mut total = 0;
    for line in buf.lines() {
        let linestr = line.unwrap();
        let parts:Vec<&str> = linestr.split(" | ").collect();
        let digits:Vec<&str> = parts[0].split(" ").collect();
        let mut one:HashSet<char> = HashSet::new();
        let mut four:HashSet<char> = HashSet::new();
        for digit in digits {
            let dlen = digit.len();
            if dlen == 2 {
                one = digit.chars().collect();
            } else if dlen == 4 {
                four = digit.chars().collect();
            }
        }
        let outputs:Vec<&str> = parts[1].split(" ").collect();
        let mut num = 0;
        for output in outputs {
            num *= 10;
            let outlen = output.len();
            let digitset:HashSet<char> = output.chars().collect();
            match outlen {
                2 => num += 1,
                3 => num += 7,
                4 => num += 4,
                7 => num += 8,
                5 => {
                    let oneinter:HashSet<_> = one.intersection(&digitset).collect();
                    let oneinter_len = oneinter.len();
                    let fourinter:HashSet<_> = four.intersection(&digitset).collect();
                    let fourinter_len = fourinter.len();
                    if oneinter_len == 1 && fourinter_len == 2 {
                        num += 2;
                    } else if oneinter_len == 2 && fourinter_len == 3 {
                        num += 3;
                    } else if oneinter_len == 1 && fourinter_len == 3 {
                        num += 5;
                    } else {
                        println!("Hmm, shouldn't happen!");
                    }
                },
                6 => {
                    let oneinter:HashSet<_> = one.intersection(&digitset).collect();
                    let oneinter_len = oneinter.len();
                    let fourinter:HashSet<_> = four.intersection(&digitset).collect();
                    let fourinter_len = fourinter.len();
                    if oneinter_len == 2 && fourinter_len == 3 {
                        num += 0;
                    } else if oneinter_len == 1 && fourinter_len == 3 {
                        num += 6;
                    } else if oneinter_len == 2 && fourinter_len == 4 {
                        num += 9;
                    } else {
                        println!("Hmm, shouldn't happen!");
                    }
                },
                _ => println!("weird digit!"),
            }
        }
        total += num;
    }
    println!("Total = {}", total);
}

fn main() {
    part1();
    part2();
}

/*

 0:6     1:2     2:5     3:5     4:4
 aaaa    ....    aaaa    aaaa    ....
b    c  .    c  .    c  .    c  b    c
b    c  .    c  .    c  .    c  b    c
 ....    ....    dddd    dddd    dddd
e    f  .    f  e    .  .    f  .    f
e    f  .    f  e    .  .    f  .    f
 gggg    ....    gggg    gggg    ....

 5:5     6:6     7:3     8:7     9:6
 aaaa    aaaa    aaaa    aaaa    aaaa
b    .  b    .  .    c  b    c  b    c
b    .  b    .  .    c  b    c  b    c
 dddd    dddd    ....    dddd    dddd
.    f  e    f  .    f  e    f  .    f
.    f  e    f  .    f  e    f  .    f
 gggg    gggg    ....    gggg    gggg


0 = a,b,c,e,f,g
1 = c,f
2 = a,c,d,e,g
3 = a,c,d,f,g
4 = b,c,d,f
5 = a,b,d,f,g
6 = a,b,d,e,f,g
7 = a,c,f
8 = a,b,c,d,e,f,g
9 = a,b,c,d,f,g

2 = a,c,d,e,g
3 = a,c,d,f,g
5 = a,b,d,f,g

5's + 4 = 2 unique
5's + 1 = 3 unique
5's + 7 = 3 unique

   1   4   7
2  1   2   2
3  2   3   3
5  1   3   2

   1   4   7
0  2   3   3
6  1   3   2
9  2   4   3

2-3-5 = e,f,b,c
2+3+5 = a,d,g

0-6-9 = c,d,e
0+6+9 = a,b,f,g

1+7 = a
1+2 = f,c
5+6 = e
2+4 = g (b from 0, d from 2)


*/
