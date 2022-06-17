use std::{
    fs::File,
    io::{prelude::*, BufReader},
    collections::HashMap,
    cmp::{min,max},
};
use hashbag::HashBag;

fn part1() {
    let buf = BufReader::new(File::open("input.txt").unwrap());
    let mut template:Vec<char> = Vec::new();
    let mut mappings:HashMap<(char,char), char> = HashMap::new();
    let mut first_lines = true;
    for line in buf.lines() {
        let linestr = line.unwrap();
        if linestr.len() == 0 {
            first_lines = false;
        } else if first_lines {
            template = linestr.chars().collect();
        } else {
            let chars:Vec<char> = linestr.chars().collect();
            mappings.insert((chars[0], chars[1]), chars[6]);

        }
    }

    // println!("Mappings = {:?}", mappings);
    // println!("template = {:?}", template);


    for step in 0..40 {
        let mut new_template:Vec<char> = Vec::new();
        for pair in template.windows(2) {
            new_template.push(pair[0]);
            let mapping = mappings.get(&(pair[0],pair[1]));
            match mapping {
                Some(new_char) => new_template.push(*new_char),
                None => ()
            }
        }
        new_template.push(template[template.len()-1]);
        template.clear();
        template.extend(new_template);
        println!("step {}", step+1);
    }

    let template_bag:HashBag<char> = HashBag::from_iter(template);

    let mut max_count = 0;
    let mut min_count = usize::MAX;

    for (_,c) in template_bag {
        max_count = max(c,max_count);
        min_count = min(c,min_count);
    }
    println!("{} - {} = {}", max_count, min_count, max_count - min_count);
}

fn part2() {

}

fn main() {
    part1();
    part2();
}
