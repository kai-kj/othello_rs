mod board;
pub mod cli;

fn main() {
    // let mut board = Board::start();
    // println!("{}", board);
    //
    // board.set(Position::parse("c4").unwrap());
    // println!("{}", board);
    //
    // board.set(Position::parse("c3").unwrap());
    // println!("{}", board);
    //
    // board.set(Position::parse("f5").unwrap());
    // println!("{}", board);

    let mut cli = cli::Cli::new();
    cli.run();
}
