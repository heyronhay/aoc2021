use std::{
    fs::File,
    io::{prelude::*, BufReader},
    collections::{HashMap,HashSet},
};
use hashbag::HashBag;

fn add_path(curr_path:&Vec<String>, cave_map:&HashMap<String,HashSet<String>>, tried_exits:&HashBag<String>, paths:& mut Vec<Vec<String>>) {
   //println!("curr_path = {:?}", curr_path);
    let exits = cave_map.get(&curr_path[curr_path.len()-1]).unwrap();
    for exit in exits {
        if exit == "start" {
            continue;
        }
        if exit == "end" {
            paths.push(curr_path.clone());
            continue;
        }
        let mut twice = false;
        for (_, copies) in tried_exits.set_iter() {
            if copies == 2 {
                twice = true;
                break;
            }
        }
        if tried_exits.contains(exit) == 0 || !twice {
            let mut new_path = curr_path.clone();
            new_path.push(exit.clone());
            let mut new_tried_exits = tried_exits.clone();
            if exit.chars().all(|c| c.is_lowercase()) {
                new_tried_exits.insert(exit.clone());
            }
            add_path(&new_path, cave_map, &new_tried_exits, paths);
        }
    }
}

fn part1() {

}
fn part2() {
    let buf = BufReader::new(File::open("input.txt").unwrap());
    let mut cave_map:HashMap<String,HashSet<String>> = HashMap::new();
    for line in buf.lines() {
        let linestr = line.unwrap();
        let parts:Vec<String> = linestr.split("-").map(String::from).collect();
        cave_map.entry(parts[0].clone()).or_insert(HashSet::new()).insert(parts[1].clone());
        cave_map.entry(parts[1].clone()).or_insert(HashSet::new()).insert(parts[0].clone());
    }

    let start_path:Vec<String> = vec![String::from("start")];
    let tried_exits:HashBag<String> = HashBag::new();
    let mut paths:Vec<Vec<String>> = Vec::new();
    add_path(&start_path, &cave_map, &tried_exits, &mut paths);

    println!("len paths = {}", paths.len());
 
}


fn main() {
    part1();
    part2();
}
