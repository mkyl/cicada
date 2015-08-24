#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

extern crate rand;

mod board;
mod zobrist;
mod fen;
mod square;
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
}

