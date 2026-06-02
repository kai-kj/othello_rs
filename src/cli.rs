use crate::board::prelude::*;
use std::io::Write;

pub struct Cli {
    board: Board,
    prev_valid: bool,
    exit: bool,
}

impl Cli {
    pub fn new() -> Cli {
        Cli {
            board: Board::start(),
            prev_valid: true,
            exit: false,
        }
    }

    pub fn run(&mut self) {
        while !self.exit {
            self.next_turn();
        }

        match (
            self.board.count_pieces(Player::Black),
            self.board.count_pieces(Player::White),
        ) {
            (b, w) if b > w => {
                println!("  {} won!", Player::Black);
            }
            (b, w) if b < w => {
                println!("  {} won!", Player::White);
            }
            _ => {
                println!("  draw!");
            }
        };

        println!("  game complete, bye");
    }

    fn next_turn(&mut self) {
        if self.prev_valid {
            println!("{}", self.board.pretty("  ").unwrap());
            println!("  {} to play", self.board.to_play);
        }

        let input = self.get_input("> ").trim().to_lowercase();

        if input == "exit" {
            self.exit = true;
            return;
        }

        let position = match Position::parse(&input) {
            Some(position) => position,
            None => {
                println!("  invalid input, try again (expected c4, f3, ...)");
                self.prev_valid = false;
                return;
            }
        };

        if self.board.legal().get(position) == false {
            println!("  illegal move, try again");
            self.prev_valid = false;
            return;
        }

        self.board.set(position);
        self.prev_valid = true;

        if self.board.legal() == 0 {
            self.exit = true;
        }
    }

    fn get_input(&mut self, prompt: &str) -> String {
        print!("{}", prompt);
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }
}
