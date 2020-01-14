#![allow(unstable_name_collisions)]

use std::collections::HashSet;
use std::env;
use std::fs;

use ext::*;

mod ext;

#[derive(Debug, Clone, Copy)]
pub struct Pos {
    row: usize,
    col: usize,
}

#[derive(Debug, Clone)]
pub struct Cell {
    pos: Pos,
    options: Vec<u8>,
}

pub struct Puzzle {
    board: Vec<Cell>,
}

fn read_file(fp: &str) -> String {
    fs::read_to_string(fp)
        .expect("Could not read file")
        .trim()
        .to_owned()
}

fn parse_char(pos: Pos, c: char) -> Cell {
    match (c.to_digit(10), c) {
        (Some(i), _) => Cell::only(pos, i as u8),
        (None, 'x') => Cell::all(pos),
        (_, c) => panic!("Couldn't parse cell: {}", c),
    }
}

fn parse(s: &str) -> Puzzle {
    let board = s
        .lines()
        .enumerate()
        .flat_map(|(row, l)| {
            l.chars()
                .enumerate()
                .map(move |(col, c)| parse_char(Pos { row, col }, c))
        })
        .collect();
    Puzzle { board }
}

fn from_pos(pos: Pos) -> usize {
    pos.row * 9 + pos.col
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let file = &args[1];
    println!("file: {}", file);
    let s = read_file(file);
    let mut puzzle = parse(&s);
    print_puzzle(&puzzle);
    println!();
    let mut removed = 1;
    let items_with_name = &[
        (puzzle.cols(), "cols"),
        (puzzle.rows(), "rows"),
        (puzzle.groups(), "groups"),
    ];
    while removed >= 1 {
        removed = 0;
        for (items, name) in items_with_name {
            match remove_pairs(&mut puzzle, &items[..], name) {
                i => {
                    if i > 0 {
                        removed += i;
                        print_puzzle(&puzzle)
                    }
                }
            }
            verify_state(&puzzle);
            match remove_singles(&mut puzzle, &items[..], name) {
                i => {
                    if i > 0 {
                        removed += i;
                        print_puzzle(&puzzle)
                    }
                }
            }
            verify_state(&puzzle);
        }
    }
    print_puzzle(&puzzle);
}

fn verify_friends(puzzle: &Puzzle, ixs: std::ops::Range<usize>) {
    let cells = || ixs.clone().map(|i| &puzzle.board[i]);
    let options = cells().flat_map(|c| &c.options).collect::<HashSet<_>>();
    if options.len() != 9 {
        println!("{:?} ({})", options, options.len());
        for cell in cells() {
            println!("cell: {:?}", format_cell(&cell))
        }
        panic!()
    }
}

fn format_cell(cell: &Cell) -> String {
    format!(
        "row={}, col={}, options={:?}",
        cell.pos.row, cell.pos.col, cell.options
    )
}

fn verify_state(puzzle: &Puzzle) {
    // rows
    verify_friends(puzzle, 0..9);
    verify_friends(puzzle, 9..18);
    verify_friends(puzzle, 18..27);
    verify_friends(puzzle, 27..36);
    verify_friends(puzzle, 36..45);
    verify_friends(puzzle, 45..54);
    verify_friends(puzzle, 54..63);
    verify_friends(puzzle, 63..72);
    verify_friends(puzzle, 72..81);
}

fn remove_singles(puzzle: &mut Puzzle, x: &[Vec<usize>], header: &str) -> usize {
    println!("remove_singles {}", header);
    let mut removals = 0;
    for ixs in x {
        println!("{}", header);
        let cloned_board = puzzle.board.clone();
        let singles = || ixs.iter().filter_map(|&c| cloned_board[c].get_single());
        println!("singles {:?}", singles().collect::<Vec<_>>());
        for &i in ixs.iter() {
            for (pos, single) in singles() {
                removals += if puzzle.board[i].remove_single(pos, single) {
                    1
                } else {
                    0
                }
            }
        }
    }
    removals
}

fn remove_pairs(puzzle: &mut Puzzle, x: &[Vec<usize>], header: &str) -> usize {
    println!("remove_pairs {}", header);
    let mut removals = 0;
    for ixs in x {
        println!("{}", header);
        let cloned_board = puzzle.board.clone();
        for &i in ixs {
            let c = &cloned_board[i];
            if !c.is_single() {
                let same_len = ixs
                    .iter()
                    .filter(|&&j| cloned_board[j].has_exactly_options(&c.options))
                    .count();
                if same_len == cloned_board[i].options.len() {
                    for &j in ixs {
                        if !cloned_board[j].has_exactly_options(&c.options) {
                            removals += puzzle.board[j].remove_multiple(&c);
                        }
                    }
                }
            }
        }
    }
    removals
}

fn print_cell_option(cell: &Cell, digit: u8) {
    if cell.has_option(digit) {
        print!("{}", digit)
    } else {
        print!(" ")
    }
}

fn print_puzzle(puzzle: &Puzzle) {
    for row in puzzle.rows() {
        println!("+---+---+---+---+---+---+---+---+---+");
        for i in &row {
            let cell = &puzzle.board[*i];
            print!("|");
            print_cell_option(&cell, 1);
            print_cell_option(&cell, 2);
            print_cell_option(&cell, 3)
        }
        println!("|");
        for i in &row {
            let cell = &puzzle.board[*i];
            print!("|");
            print_cell_option(&cell, 4);
            print_cell_option(&cell, 5);
            print_cell_option(&cell, 6)
        }
        println!("|");
        for i in &row {
            let cell = &puzzle.board[*i];
            print!("|");
            print_cell_option(&cell, 7);
            print_cell_option(&cell, 8);
            print_cell_option(&cell, 9)
        }
        println!("|");
    }
    println!("+---+---+---+---+---+---+---+---+---+");
}

impl Cell {
    fn all(pos: Pos) -> Cell {
        Cell {
            pos,
            options: (1..10).collect(),
        }
    }

    fn only(pos: Pos, d: u8) -> Cell {
        Cell {
            pos,
            options: vec![d],
        }
    }

    fn is_single(&self) -> bool {
        self.options.len() == 1
    }

    fn get_single(&self) -> Option<(Pos, u8)> {
        if self.is_single() {
            self.options.first().map(|&x| (self.pos, x))
        } else {
            None
        }
    }

    fn has_option(&self, d: u8) -> bool {
        self.options.contains(&d)
    }

    fn has_exactly_options(&self, vs: &[u8]) -> bool {
        self.options[..] == *vs
    }

    fn remove_single(&mut self, pos: Pos, single: u8) -> bool {
        if self.is_single() {
            return false;
        }
        match self.options.remove_item(&&single) {
            Some(i) => {
                println!(
                    "{:?}: Removed {}, because of {:?}, {}",
                    self.pos, i, pos, single
                );
                true
            }
            None => false,
        }
    }

    fn remove_multiple(&mut self, multiple: &Cell) -> usize {
        let mut removed = 0;
        if self.is_single() {
            removed
        } else {
            for m in &multiple.options {
                removed += if self.options.remove_item(&m).is_some() {
                    1
                } else {
                    0
                }
            }
            removed
        }
    }
}

impl Puzzle {
    fn rows(&self) -> Vec<Vec<usize>> {
        let mut rows = vec![];
        for row_i in 0..9 {
            let mut col = vec![];
            for col_i in 0..9 {
                col.push(from_pos(Pos {
                    row: row_i,
                    col: col_i,
                }))
            }
            assert!(col.len() == 9, format!("{}", col.len()));
            rows.push(col)
        }
        rows
    }

    fn cols(&self) -> Vec<Vec<usize>> {
        let mut cols = vec![];
        for col_i in 0..9 {
            let mut row = vec![];
            for row_i in 0..9 {
                row.push(from_pos(Pos {
                    row: row_i,
                    col: col_i,
                }))
            }
            assert!(row.len() == 9, format!("{}", row.len()));
            cols.push(row)
        }
        cols
    }

    fn groups(&self) -> Vec<Vec<usize>> {
        let mut groups = vec![];
        for i in 0..3 {
            for j in 0..3 {
                let mut group = vec![];
                for x in 0..3 {
                    for y in 0..3 {
                        group.push(from_pos(Pos {
                            row: i * 3 + x,
                            col: j * 3 + y,
                        }));
                    }
                }
                groups.push(group)
            }
        }
        groups
    }

    // Alternative:
    // fn groups2(&self) -> [[usize; 9]; 9] {
    //     let mut groups: [[usize; 9]; 9] = [[0; 9]; 9];
    //     for i in 0..3 {
    //         for j in 0..3 {
    //             let group = &mut groups[i * 3 + j];
    //             for x in 0..3 {
    //                 for y in 0..3 {
    //                     group[x * 3 + y] = from_pos(Pos {
    //                         row: i * 3 + x,
    //                         col: j * 3 + y,
    //                     });
    //                 }
    //             }
    //         }
    //     }
    //     groups
    // }
}
