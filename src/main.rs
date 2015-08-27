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

    let mut move_list : moves::movelist =  moves::movelist::new();

    // fen::parse("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", &mut main_board);
    //  fen::parse("rnbqkb1r/pp1p1pPp/8/2p1pP2/1P1P4/3P3P/P1P1P3/RNBQKBNR w KQkq e6 0 1", &mut main_board);
      fen::parse("qrbnkbnr/p1p1p3/3p3p/1p1p4/2P1Pp2/8/PP1P1PpP/QRBNKB1R b KQkq e3 0 1", &mut main_board);
    /*
    board::print(&main_board);

    let hello = moves::_move::new(56, 77, 0, board::piece::r as u8, true, false, true);
    println!("from: {} to: {} promoted:{} EP:{}", moves::from(&hello), moves::to(&hello), moves::promoted(&hello),
    moves::castling(&hello));

    for x in 0..5 {
        print!("{}", moves::to_AN(&hello)[x]);
    }
    */
    println!("");

    //main_board.side = board::black;
    moves::generator(&mut move_list, &main_board);

    for x in 0..move_list.count as usize {
        for i in 0..5 {
            print!("{}", moves::to_AN(&move_list.all[x])[i]);
        }
        println!("");
    }

    println!("move count: {}", move_list.count);

    debug_assert!(sanity::sane(&main_board));
}

