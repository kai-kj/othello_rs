mod bitboard;
mod board;
mod lut;
mod prelude;

use crate::prelude::*;

fn main() {
    let mut board = Board::new();
    println!("{}", board);

    board.set(Position::parse("c4").unwrap());
    println!("{}", board);

    board.set(Position::parse("c3").unwrap());
    println!("{}", board);
}
