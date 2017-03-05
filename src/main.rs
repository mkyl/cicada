#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

extern crate rand;
extern crate time;

mod board;
mod zobrist;
mod fen;
mod square;
mod moves;
mod movement;
mod sanity;
mod think;
mod uci;

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

    println!(" [i] Initializing Zobrist Hashes");
    zobrist::init();

    println!(" [i] Initializing Chessboard");
    println!(" [i] Initializing Transposition Tables");
    let mut cboard : chessboard = board::init();

    println!(" [✓] Startup Successful\n");

    uci::looping(&mut cboard);
}

