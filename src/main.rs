#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

extern crate rand;

mod board;
mod zobrist;
mod fen;
mod square;
mod moves;
mod sanity;

fn main(){
    use board::chessboard;

    println!{"
                `-.  \\    .-'              ██████╗██╗ ██████╗ █████╗ ██████╗  █████╗  
        ,-`````\"\"-\\__ |  /                ██╔════╝██║██╔════╝██╔══██╗██╔══██╗██╔══██╗ 
         '-.._    _.-'` '-o,              ██║     ██║██║     ███████║██║  ██║███████║ 
             _>--:{{<   ) |)               ██║     ██║██║     ██╔══██║██║  ██║██╔══██║ 
         .-''      '-.__.-o`              ╚██████╗██║╚██████╗██║  ██║██████╔╝██║  ██║ 
        '-._____..-/`  |  \\                ╚═════╝╚═╝ ╚═════╝╚═╝  ╚═╝╚═════╝ ╚═╝  ╚═╝ 
                ,-'   /    `-.                                     \"Numbers Matter\""};


    // init order is important
    println!("\n Startup Sequence:");

    println!(" [i] Initializing Zobrist Hash Tables");
    zobrist::init();

    println!(" [i] Initializing Chessboard");
    let mut main_board : chessboard = board::init();

    fen::parse("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", &mut main_board);
    board::print(&main_board);

    let hello = moves::_move::new(56, 77, 0, board::piece::r as u8, true, false, true);
    println!("from: {} to: {} promoted:{} EP:{}", moves::from(&hello), moves::to(&hello), moves::promoted(&hello),
    moves::castling(&hello));

    for x in 0..5 {
        print!("{}", moves::to_AN(&hello)[x]);
    }
    println!("");

    debug_assert!(sanity::sane(&main_board));
}

