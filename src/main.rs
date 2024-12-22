mod tictactoe;

use tictactoe::{TicTacToe, Mark}; 
use colored::*; 
use crossterm::{execute, terminal, cursor};

macro_rules! print_flush {
    ($($arg:tt)*) => {
        {
            use std::io::{self, Write}; 
            print!($($arg)*); 
            io::stdout().flush().expect("Failed to flush stdout.");
        }
    };
}

fn error_msg(message: &str, position: (u16, u16)) {
    clear_line();
    println!("{}", message.red()); 
    move_cursor(position);
    clear_line(); 
}

fn move_cursor(position: (u16, u16)) {
    execute!(std::io::stdout(), cursor::MoveTo(position.0, position.1)).unwrap();
}

fn clear_line() {
    execute!(std::io::stdout(), terminal::Clear(terminal::ClearType::CurrentLine)).unwrap(); 
}

fn clear_screen() {
    execute!(std::io::stdout(), terminal::Clear(terminal::ClearType::All)).unwrap(); 
    move_cursor((0, 0));
}

fn display_rules() {
    println!("{}", "============================= Rules =============================".black().on_white()); 
    println!("The aim of the game is to form a straight line of '{}'s or '{}'s either horizontally, vertically, or diagonally.", Mark::X, Mark::O);
    println!("Player 1 puts down '{}'s on the board while Player 2 puts down '{}'s on the board.", Mark::X, Mark::O);
    println!("A player wins when they form a straight line with their respective mark, '{}' or '{}', first.", Mark::X, Mark::O);
    println!("{}", "========================= Instructions ==========================".black().on_white()); 
    println!("  1. Enter the {} in the grid to place your mark in that position.", "letter".bold()); 
    println!("  2. The first player to make a straight line with their mark wins the game.");
}

fn get_grid_size() -> usize {
    let position = cursor::position().unwrap(); 
    loop {
        let mut size_str = String::new(); 
        print_flush!("Enter the size of the grid (2 <= size <= 7): "); 
        std::io::stdin().read_line(&mut size_str).expect("Failed to read line."); 

        match size_str.trim().parse::<usize>() {
            Ok(size) if size >= 2 && size <= 7 => break size, 
            Ok(_) => error_msg("Grid size must be within the specified bounds.", position),
            Err(_) => error_msg("Please enter a valid number", position),
        }
    }
}

fn main() {
    use std::io; 
    clear_screen();
    println!("Welcome to Tic Tac Toe!");
    let start_position = cursor::position().unwrap(); 
    loop {
        print_flush!("Would you like to read the rules and instructions? (y/n): ");
        let mut rule_choice = String::new(); 
        io::stdin().read_line(&mut rule_choice).expect("Failed to read line.");
    
        match rule_choice.trim().to_lowercase().as_str() {
            "y" | "yes" => {
                clear_screen(); 
                display_rules(); 
    
                print_flush!("\nPress enter to continue."); 
                io::stdin().read_line(&mut String::new()).expect("Failed to read line.");
                break; 
            }, 
            "n" | "no" => break, 
            _ => {
                move_cursor(start_position);
                clear_line(); 
                continue
            },
        };
    }

    'main: loop {
        clear_screen();
        let size: usize = get_grid_size(); 
        let mut game = TicTacToe::new(size); 

        let mut turn: usize = 0; 
        let marks: [Mark; 2] = [Mark::X, Mark::O]; 

        loop { 
            clear_screen();
            game.display_board(); 
            println!("\nIt's Player {}'s turn ({}).", (turn % 2) + 1, marks[turn % 2]); 
            
            let position = cursor::position().unwrap(); 
            let cell = loop {
                print_flush!("Enter a cell: "); 
                let mut cell_str = String::new(); 
                io::stdin().read_line(&mut cell_str).expect("Failed to read line."); 

                if cell_str.trim().len() > 1 {
                    error_msg("Please enter a single character", position);
                } else if let Some(ch) = cell_str.trim().chars().next() {
                    if !game.check_valid_cell(ch) {
                        error_msg("Please enter a valid cell.", position); 
                    } else { break ch; }
                }
            };

            game.update_board(cell, marks[turn % 2].clone());
            turn += 1; 

            if game.check_win(marks[1 - (turn % 2)].clone()) {
                clear_screen(); 
                game.display_board(); 
                println!("\nPlayer {} wins!", (turn % 2)); 
                break; 
            } else if turn == size*size { // check tie
                clear_screen(); 
                game.display_board(); 
                println!("\nIt's a tie."); 
                break; 
            }
        }

        let position = cursor::position().unwrap(); 
        loop {
            print_flush!("Would you like to play again? (y/n): ");
            let mut again_choice = String::new(); 
            io::stdin().read_line(&mut again_choice).expect("Failed to read line."); 

            match again_choice.trim().to_lowercase().as_str() {
                "y" | "yes" => break, 
                "n" | "no" => {
                    println!("Thanks for playing."); 
                    break 'main; // breaks out of the main loop 
                }, 
                _ => {
                    move_cursor(position);
                    clear_line(); 
                    continue;
                },
            }
        }
    }
}