mod game;

use std::io::{self, Read};

fn main() {
    let mut buf = String::new();
    io::stdin()
        .read_to_string(&mut buf)
        .expect("failed to read stdin");
    let mut board = game::Board::parse(buf).expect("failed to parse board");
    board.check().expect("the provided board is invalid");
    board.solve().expect("unable to solve board");
    println!("Solution: {:#?}", board);
}
