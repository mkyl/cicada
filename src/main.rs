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

    /*

    // fen::parse("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", &mut main_board);
    // fen::parse("2rr3k/pp3pp1/1nnqbN1p/3pN3/2pP4/2P3Q1/PPB4P/R4RK1 w - - 0 1", &mut main_board);
    // fen::parse("3r1r2/1pp2p1k/p5pp/4P3/2nP3R/2P3QP/P1B1q1P1/5RK1 w - - 0 1", &mut main_board);
    // fen::parse("3R4/5K2/8/4k1P1/4p3/7B/4p3/8 b - - 1 0", &mut main_board);
    fen::parse("r1b1k2r/ppppnppp/2n2q2/2b5/3NP3/2P1B3/PP3PPP/RN1QKB1R w KQkq - 0 1", &mut main_board);
    // fen::parse("qrbnkbnr/p1p1p3/3p3p/1p1p4/2P1Pp2/8/PP1P1PpP/QRBNKB1R b KQkq e3 0 1", &mut main_board);
    // fen::parse("5k2/1n6/4n3/6N1/8/3N4/8/5K2 w - - 0 1", &mut main_board);
    // fen::parse("6k1/8/5r2/8/1nR5/5N2/8/6K1 w - - 0 1", &mut main_board);
    // fen::parse("6k1/8/4nq2/8/1nQ5/5N2/1N6/6K1 b - - 0 1", &mut main_board);
    // fen::parse("6k1/1b6/4n3/8/1n4B1/1B3N2/1N6/2b3K1 b - - 0 1", &mut main_board);
    // fen::parse("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1", &mut main_board);
    // fen::parse("3rk2r/8/8/8/8/8/6p1/R3K2R b KQk - 0 1", &mut main_board);
    // fen::parse("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1", &mut main_board);
    // fen::parse("r3k2r/8/3Q4/8/8/5q2/8/R3K2R b KQkq - 0 1", &mut main_board);
    // fen::parse("r3k2r/1b4bq/8/8/8/8/7B/R3K2R w KQkq - 0 1", &mut main_board);
    // fen::parse("n1n5/PPPk4/8/8/8/8/4Kppp/5N1N b - - 0 1", &mut main_board);
    // fen::parse("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1", &mut main_board);
    // fen::parse("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10", &mut main_board);

    let look = 5;
    think::start(&mut main_board, look, 0);

    println!("best move:");
    for x in 0..look {
        let movec = moves::_move{container: think::find_transposition(&main_board), score:0};
        print!("{}{}{}{} ", board::to_AN(moves::from(&movec))[0], board::to_AN(moves::from(&movec))[1], board::to_AN(moves::to(&movec))[0], board::to_AN(moves::to(&movec))[1]); 
        movement::make(&movec, &mut main_board);
    }
    print!("\n");
*/
    /*
    let move1 = moves::_move::new(22, 43, 0, 0, false, false, false);
    movement::make(&move1, &mut main_board);
    let move4 = moves::_move::new(84, 74, 0, 0, false, false, false);
    movement::make(&move4, &mut main_board);
    */

    /*
    println!("materials: {} {}", main_board.score[0], main_board.score[1]);

    let move1 = moves::_move::new(22, 43, 0, 0, false, false, false);
    think::store_transposition(move1.container, &mut main_board);
    movement::make(&move1, &mut main_board);
    let move2 = moves::_move::new(97, 78, 0, 0, false, false, false);
    think::store_transposition(move2.container, &mut main_board);
    movement::make(&move2, &mut main_board);
    let move3 = moves::_move::new(35, 45, 0, 0, false, false, false);
    think::store_transposition(move3.container, &mut main_board);
    movement::make(&move3, &mut main_board);
    let move4 = moves::_move::new(82, 72, 0, 0, false, false, false);
    think::store_transposition(move4.container, &mut main_board);
    movement::make(&move4, &mut main_board);

    for x in 0..4 {
        movement::undo(&mut main_board);
        let movec = moves::_move{container: think::find_transposition(&main_board), score:0};
        if movec.container != 1 {
           println!("from: {}, to: {}", moves::from(&movec), moves::to(&movec)) ;
        } else {
        }
    }
    */

    /*
    println!("from: {} to: {} promoted:{} EP:{}", moves::from(&hello), moves::to(&hello), moves::promoted(&hello),
    moves::castling(&hello));

    for x in 0..5 {
        print!("{}", moves::to_AN(&hello)[x]);
    }
    println!("");

    //main_board.side = board::black;
    moves::generator(&mut move_list, &main_board);

    for x in 0..move_list.count as usize {
        for i in 0..5 {
            print!("{}", moves::to_AN(&move_list.all[x])[i]);
        }
        if moves::capture(&move_list.all[x]) != 0 {
            print!(" capture");
        }
        if moves::castling(&move_list.all[x]) {
            print!(" castling");
        }
        if moves::en_passant(&move_list.all[x]) {
            print!(" en passant");
        }
        if moves::pawn_double(&move_list.all[x]) {
            print!(" double");
        }
        println!("");
    }

    println!("move count: {}", move_list.count);
    */
}

