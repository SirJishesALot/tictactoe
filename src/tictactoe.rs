use colored::*; 
use std::collections::HashMap; 

#[derive(Clone)]
#[derive(PartialEq)]
pub enum Mark { Unmarked, X, O }

impl std::fmt::Display for Mark {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mark = match self {
            Mark::Unmarked => "-".red(), 
            Mark::X => "X".blue(), 
            Mark::O => "O".green(),
        };
        write!(f, "{}", mark)
    }
}

pub struct TicTacToe {
    letters: Vec<char>,
    size: usize,
    line: String, 
    board: Vec<Vec<Mark>>, 
    map: HashMap<char, (usize, usize)>, // map from num to index. eg: 1 -> (0, 0)
}

impl TicTacToe {
    pub fn new(_size: usize) -> Self {
        let mut _letters: Vec<char> = ('a'..='z').collect();
        let uppercase: Vec<char> = ('A'..='Z').collect();
        _letters.extend(uppercase);

        let mut _line = String::new();
        for _ in 0.._size-1 {
            _line += "----"; 
        } _line += "---"; 

        TicTacToe {
            letters: _letters.clone(),
            size: _size, 
            line: _line, 
            board: Self::initialize_board(_size), 
            map: Self::initialize_map(&_letters, _size),
        }
    }

    fn initialize_board(_size: usize) -> Vec<Vec<Mark>> {
        let mut _board: Vec<Vec<Mark>> = vec![];
        for i in 0.._size {
            _board.push(Vec::with_capacity(_size)); 
            _board[i].extend(std::iter::repeat(Mark::Unmarked).take(_size));
        }
        _board
    }

    fn initialize_map(_letters: &Vec<char>, _size: usize) -> HashMap<char, (usize, usize)> {
        let mut _map: HashMap<char, (usize, usize)> = HashMap::new(); 
        let mut row: usize = 0; 
        for i in 0.._size {
            _map.insert(_letters[i], (row, i));
        }
        for i in _size..(_size*_size) {
            if i % _size == 0 { row += 1; }
            _map.insert(_letters[i], (row, i % _size));
        }
        _map
    }

    pub fn display_board(&self) {
        use std::io::{self, Write}; 

        let mut index: usize = 0; 
        for i in 0..self.size {
            for j in 0..self.size {
                match self.board[i][j] {
                    Mark::Unmarked => print!(" {} ", self.letters[index]),
                    Mark::X => print!(" {} ", Mark::X),
                    Mark::O => print!(" {} ", Mark::O), 
                }
                if j < self.size - 1 { print!("|"); }
                index += 1; 
            }
            io::stdout().flush().expect("Failed to flush stdout.");
            if i < self.size - 1 { println!("\n{}", self.line); }
        }
        println!("");
    }

    fn check_diagonal(&self, mark: Mark) -> bool {
        for i in 0..self.size {
            if self.board[i][i] != mark {
                return false;
            }
        } true 
    }
    fn check_antidiagonal(&self, mark: Mark) -> bool {
        for i in 0..self.size {
            let n = self.size - 1 - i; 
            if self.board[i][n] != mark {
                return false;
            }
        } true
    }

    pub fn check_win(&self, mark: Mark) -> bool {
        for row in 0..self.size { // check all rows
            if self.board[row].iter().all(|x| *x == mark) {
                return true;
            }
        }

        for col in 0..self.size { // check all cols
            if (0..self.size).map(|row| self.board[row][col].clone()).all(|x| x == mark) {
                return true;
            }
        }
        
        Self::check_diagonal(self, mark.clone()) || Self::check_antidiagonal(self, mark.clone())
    }

    pub fn check_valid_cell(&self, cell: char) -> bool {
        match self.map.get(&cell) {
            Some((row, col)) => self.board[*row][*col] == Mark::Unmarked,
            None => false,
        }
    }

    pub fn update_board(&mut self, cell: char, mark: Mark) {
        match self.map.get(&cell) {
            Some((row, col)) => self.board[*row][*col] = mark, 
            None => println!("Invalid cell."), // this line should never run
        }
    }
}