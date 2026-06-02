mod bitboard;
mod board;
mod player;
mod position;
mod prelude;

use crate::prelude::*;

fn main() {
    let mut board = Board::start();
    println!("{}", board);

    board.set(Position::parse("c4").unwrap());
    println!("{}", board);

    board.set(Position::parse("c3").unwrap());
    println!("{}", board);

    board.set(Position::parse("f5").unwrap());
    println!("{}", board);
}
