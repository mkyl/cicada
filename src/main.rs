#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

extern crate rand;

mod zobrist;
pub mod board;

fn main(){
    use board::chessboard;

    // init order is important
    zobrist::init();
    let mut main_board : chessboard = board::init();
}
