#![feature(vec_remove_item)]
use std::collections::HashSet;
use std::env;
use std::fs;

fn read_file(fp: &str) -> String {
    fs::read_to_string(fp)
        .expect("Could not read file")
        .trim()
        .to_string()
}

fn parse_char(pos: Pos, c: char) -> Cell {
    match (c.to_digit(10), c) {
        (Some(i), _) => Cell::only(pos, i as u8),
        (None, 'x') => Cell::all(pos),
        (_, c) => panic!("Couldn't parse cell: {}", c)
    }
}

fn parse(s: &String) -> Puzzle {
    let board = s.lines().enumerate().map(|(row, l)| {
        l.chars().enumerate().map(|(col, c)| parse_char(Pos { row, col }, c)).collect::<Vec<_>>()
    }).flatten().collect();
    Puzzle {
        board
    }
}

#[derive(Debug, Clone)]
pub struct Pos {
    row: usize,
    col: usize,
}

#[derive(Debug, Clone)]
pub struct Cell {
    pos: Pos,
    options: Vec<u8>,
}

impl Cell {
    fn all(pos: Pos) -> Cell {
        Cell { pos, options: (1..10).collect() }
    }
    fn only(pos: Pos, d: u8) -> Cell {
        Cell { pos, options: vec![d] }
    }
    fn is_single(&self) -> bool {
        self.options.len() == 1
    }
    fn get_single(&self) -> Option<&Cell> {
        if self.is_single() {
            Some(self)
        } else {
            None
        }
    }
    fn get_single_u8(&self) -> Option<&u8> {
        if self.is_single() {
            self.options.first()
        } else {
            None
        }
    }
    fn has_option(&self, d: u8) -> bool {
        self.options.contains(&d)
    }
    fn has_exactly_options(&self, vs: &Vec<u8>) -> bool {
        self.options == *vs
    }
    fn remove_single(&mut self, single: &Cell) -> bool {
        if self.is_single() {
            false
        } else {
          assert!(single.is_single());
          if let Some(s) = single.get_single_u8() {
              match self.options.remove_item(s) {
                  Some(i) => {
                      println!("{:?}: Removed {}, because of {:?}", self.pos, i, single);
                      true
                  },
                  None => false,
              }
          } else {
              assert!(false, "impossible");
              false
          }
        }
    }
    fn remove_multiple(&mut self, multiple: &Cell) -> usize {
        let mut removed = 0;
        if self.is_single() {
            removed
        } else {
            for m in &multiple.options {
                removed += if self.options.remove_item(&m).is_some() { 1 } else { 0 }
            };
            removed
        }
    }
}

fn from_pos(pos: Pos) -> usize {
    pos.row * 9 + pos.col
}
fn to_pos(u: usize) -> Pos {
    Pos {
        row: u / 9,
        col: u % 9
    }
}

pub struct Puzzle {
    board: Vec<Cell>
}
impl Puzzle {
    fn rows(&self) -> Vec<Vec<usize>> {
        let mut rows = vec![];
        for row_i in 0..9 {
            let mut col = vec![];
            for col_i in 0..9 {
                col.push(from_pos(Pos { row: row_i, col: col_i }))
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
                row.push(from_pos(Pos { row: row_i, col: col_i }))
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
                group.push(from_pos(Pos { row: i*3    , col: j*3    }));
                group.push(from_pos(Pos { row: i*3    , col: j*3 + 1}));
                group.push(from_pos(Pos { row: i*3    , col: j*3 + 2}));
                group.push(from_pos(Pos { row: i*3 + 1, col: j*3    }));
                group.push(from_pos(Pos { row: i*3 + 1, col: j*3 + 1}));
                group.push(from_pos(Pos { row: i*3 + 1, col: j*3 + 2}));
                group.push(from_pos(Pos { row: i*3 + 2, col: j*3    }));
                group.push(from_pos(Pos { row: i*3 + 2, col: j*3 + 1}));
                group.push(from_pos(Pos { row: i*3 + 2, col: j*3 + 2}));
                groups.push(group)
            }
        };
        groups
    }
}



fn handle(puzzle: &Puzzle, removed: usize) -> usize {
    if removed > 0 {
        print_puzzle(&puzzle)
    }
    verify_state(&puzzle);
    removed
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
    while removed >= 1 {
        removed = 0;
        let rows = puzzle.rows();
        match remove_singles(&mut puzzle, rows, "rows") {
            i =>
                if i > 0 {
                    removed += i;
                    print_puzzle(&puzzle)
                }
        }
        verify_state(&puzzle);

        let cols = puzzle.cols();
        match remove_singles(&mut puzzle, cols, "cols") {
            i =>
                if i > 0 {
                    removed += i;
                    print_puzzle(&puzzle)
                }
        }
        verify_state(&puzzle);

        let groups = puzzle.groups();
        match remove_singles(&mut puzzle, groups, "groups") {
            i =>
                if i > 0 {
                    removed += i;
                    print_puzzle(&puzzle)
                }
        }
        verify_state(&puzzle);

        let rows = puzzle.rows();
        match remove_pairs(&mut puzzle, rows, "rows") {
            i =>
                if i > 0 {
                    removed += i;
                    print_puzzle(&puzzle)
                }
        }
        verify_state(&puzzle);

        let cols = puzzle.cols();
        match remove_pairs(&mut puzzle, cols, "cols") {
            i =>
                if i > 0 {
                    removed += i;
                    print_puzzle(&puzzle)
                }
        }
        verify_state(&puzzle);

        let groups = puzzle.groups();
        match remove_pairs(&mut puzzle, groups, "groups") {
            i =>
                if i > 0 {
                    removed += i;
                    print_puzzle(&puzzle)
                }
        }
        verify_state(&puzzle);

    }
    print_puzzle(&puzzle);
}

fn verify_friends(puzzle: &Puzzle, ixs: &Vec<usize>) {
    let cells: Vec<_> = ixs.iter().map(|i| puzzle.board[*i].clone()).collect();
    let options = cells.iter().flat_map(|c| c.options.clone()).collect::<HashSet<_>>();
    if options.len() != 9 {
        println!("{:?} ({})", options, options.len());
        for cell in cells {
            println!("cell: {:?}", format_cell(&cell))
        }
        panic!()
    }
}

fn format_cell(cell: &Cell) -> String {
    format!("row={}, col={}, options={:?}", cell.pos.row, cell.pos.col, cell.options)
}

fn verify_state(puzzle: &Puzzle) {
    // rows
    verify_friends(puzzle, &(0..9).collect::<Vec<_>>());
    verify_friends(puzzle, &(9..18).collect::<Vec<_>>());
    verify_friends(puzzle, &(18..27).collect::<Vec<_>>());
    verify_friends(puzzle, &(27..36).collect::<Vec<_>>());
    verify_friends(puzzle, &(36..45).collect::<Vec<_>>());
    verify_friends(puzzle, &(45..54).collect::<Vec<_>>());
    verify_friends(puzzle, &(54..63).collect::<Vec<_>>());
    verify_friends(puzzle, &(63..72).collect::<Vec<_>>());
    verify_friends(puzzle, &(72..81).collect::<Vec<_>>())
}

fn remove_singles(puzzle: &mut Puzzle, x: Vec<Vec<usize>>, header: &str) -> usize {
    println!("remove_singles {}", header);
    let mut removals = 0;
    for ixs in x {
        println!("{}", header);
        let clone = ixs.clone();
        let cloned_board = puzzle.board.clone();
        let singles: Vec<&Cell> = clone.iter().filter_map(|c| cloned_board[*c].get_single()).collect::<Vec<_>>();
        let s: Vec<Vec<u8>> = singles.iter().map(|i| i.options.clone()).collect::<Vec<_>>();
        println!("singles {:?}", s);
        for i in ixs {
            for single in singles.iter() {
                let c = &mut puzzle.board[i];
                removals += if c.remove_single(single) { 1 } else { 0 }
            }
        }
    };
    removals
}

fn remove_pairs(puzzle: &mut Puzzle, x: Vec<Vec<usize>>, header: &str) -> usize {
    println!("remove_pairs {}", header);
    let mut removals = 0;
    for ixs in x.iter() {
        println!("{}", header);
        let cloned_board = puzzle.board.clone();
        for i in ixs {
            let c = &cloned_board[*i];
            if !c.is_single() {
                let same = ixs.iter().filter(|j| cloned_board[**j].has_exactly_options(&c.options)).collect::<Vec<_>>();
                if same.len() == cloned_board[*i].options.len() {
                    for j in ixs {
                        if !cloned_board[*j].has_exactly_options(&c.options) {
                            removals += puzzle.board[*j].remove_multiple(&c);
                        }
                    }
                }
            }
        }
    };
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
