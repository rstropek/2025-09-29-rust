mod logic;

use logic::{Playingfield, WinnerStatus, Player};
use std::io::{self, Write};

fn main() {
    let mut field = Playingfield::default();
    
    println!("Welcome to Tic-Tac-Toe!");
    println!("Enter coordinates like 'A1', 'B2', 'C3', etc.");
    println!("Columns: A, B, C");
    println!("Rows: 1, 2, 3\n");

    loop {
        // Display the current board
        println!("{field}");

        // Check for winner or draw
        match field.get_winner() {
            WinnerStatus::Winner(Player::X) => {
                println!("Player X wins! 🎉");
                break;
            }
            WinnerStatus::Winner(Player::O) => {
                println!("Player O wins! 🎉");
                break;
            }
            WinnerStatus::Draw => {
                println!("It's a draw! 🤝");
                break;
            }
            WinnerStatus::NoWinner => {
                // Game continues
            }
        }

        // Get user input
        print!("Enter your move: ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim().to_uppercase();

        // Parse coordinate
        match input.parse() {
            Ok(coordinate) => {
                // Try to make the move
                match field.make_move(coordinate) {
                    Ok(_) => {
                        // Move successful, continue to next iteration
                    }
                    Err(_) => {
                        println!("❌ That cell is already occupied! Try again.\n");
                    }
                }
            }
            Err(err) => {
                println!("❌ Invalid input: {:?}. Please use format like 'A1', 'B2', etc.\n", err);
            }
        }
    }

    println!("\nThanks for playing!");
}
