#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

extern crate rand;

pub mod board;
mod zobrist;
mod fen;

fn main(){
    use board::chessboard;

    // init order is important
    zobrist::init();
    let mut main_board : chessboard = board::init();

    fen::parse("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", &mut main_board);
}
