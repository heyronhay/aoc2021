use std::{
    fs::File,
    io::{prelude::*, BufReader},
    collections::{HashMap},
};

#[derive(Clone)]
struct BoardCell {
    value: usize,
    picked: bool,
    row: usize,
    col: usize,
}

#[derive(Clone)]
struct Board {
    id: usize,
    won: bool,
    row_counts: [usize;5],
    col_counts: [usize;5],
    cell_map: HashMap<usize, BoardCell>,
}

fn part1() {
    let buf = BufReader::new(File::open("input.txt").unwrap());
    let mut iter = buf.lines();
    let mut boards:Vec<Board> = Vec::new();

    // Read random number line
    let random_line = iter.next().unwrap().unwrap();
    let rand_nums:Vec<usize> = random_line.split(",").map(|v| v.parse().unwrap()).collect();
    println!("rand = {:?}", rand_nums);

    let mut board_count = 0;
    loop {
        if iter.next().is_none() {
            break;
        };
        let mut board:Board = Board {
            id: board_count,
            won: false,
            row_counts: [0;5],
            col_counts: [0;5],
            cell_map: HashMap::new(),
        };
        for row in 0..5 {
            let board_line = iter.next().unwrap().unwrap();
            let cols:Vec<_> = board_line.split_whitespace().map(|v| v.parse().unwrap()).collect();
            let mut col = 0;
            for val in cols {
                let cell:BoardCell = BoardCell {
                    value: val,
                    picked: false,
                    row: row,
                    col: col,
                };
                board.cell_map.insert(val, cell);
                col += 1;
            }
        }
        boards.push(board);
        board_count += 1;
    }

    let mut total = 0;
    for rand_num in rand_nums {
        let mut found = false;
        for board in boards.iter_mut(){
            if board.cell_map.contains_key(&rand_num) {
                let cell = board.cell_map.get_mut(&rand_num).unwrap();
                board.row_counts[cell.row] += 1;
                board.col_counts[cell.col] += 1;
                cell.picked = true;
                if (board.row_counts[cell.row] == 5) || (board.col_counts[cell.col] == 5) {
                    println!("bingo! {}", rand_num);
                    board.won = true;
                    found = true;
                    let mut unset = 0;
                    for (_, cell) in board.cell_map.iter_mut() {
                        if !cell.picked {
                            unset += cell.value;
                        }
                    }
                    total = unset * rand_num;
                    break;
                }
            }
        }
        if found {
            break;
        }
    }
    println!("board = {}", total);
}

fn part2() {
    let buf = BufReader::new(File::open("input.txt").unwrap());
    let mut iter = buf.lines();
    let mut boards:Vec<Board> = Vec::new();

    // Read random number line
    let random_line = iter.next().unwrap().unwrap();
    let rand_nums:Vec<usize> = random_line.split(",").map(|v| v.parse().unwrap()).collect();
    println!("rand = {:?}", rand_nums);

    let mut board_count = 0;
    loop {
        if iter.next().is_none() {
            break;
        };
        let mut board:Board = Board {
            id: board_count,
            won: false,
            row_counts: [0;5],
            col_counts: [0;5],
            cell_map: HashMap::new(),
        };
        for row in 0..5 {
            let board_line = iter.next().unwrap().unwrap();
            let cols:Vec<_> = board_line.split_whitespace().map(|v| v.parse().unwrap()).collect();
            let mut col = 0;
            for val in cols {
                let cell:BoardCell = BoardCell {
                    value: val,
                    picked: false,
                    row: row,
                    col: col,
                };
                board.cell_map.insert(val, cell);
                col += 1;
            }
        }
        boards.push(board);
        board_count += 1;
    }

    let mut total = 0;
    let mut board_count = boards.len();
    for rand_num in rand_nums {
        let mut found = false;
        for board in boards.iter_mut(){
            if !board.won && board.cell_map.contains_key(&rand_num) {
                let cell = board.cell_map.get_mut(&rand_num).unwrap();
                board.row_counts[cell.row] += 1;
                board.col_counts[cell.col] += 1;
                cell.picked = true;
                if (board.row_counts[cell.row] == 5) || (board.col_counts[cell.col] == 5) {
                    println!("bingo! {}", rand_num);
                    board.won = true;
                    board_count -= 1;
                    if board_count == 0 {
                        found = true;
                        let mut unset = 0;
                        for (_, cell) in board.cell_map.iter_mut() {
                            if !cell.picked {
                                unset += cell.value;
                            }
                        }
                        total = unset * rand_num;
                        break;
                    }
                }
            }
        }
        if found {
            break;
        }
    }
    println!("board = {}", total);
}

fn main() {
    part1();
    part2();
}
