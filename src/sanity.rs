use board;
use fen;
use zobrist;

#[test]
fn layout() {
    zobrist::init();
    let mut main_board : board::chessboard = board::init();

    fen::parse("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", &mut main_board);
    verify_squares(&main_board);

    fen::parse("8/4r1p1/5pBp/2k2P2/3p4/1Pn2KP1/3R1P2/8 w - - 0 1", &mut main_board);
    verify_squares(&main_board);

    fen::parse("4k3/8/8/8/8/8/4P3/4K3 w - - 5 39", &mut main_board);
    verify_squares(&main_board);

    fen::parse("r1bq1rk1/pp3ppp/3n4/2p1N3/2B5/7P/PPP2PP1/R1BQR1K1 w - h2 0 33", &mut main_board);
    verify_squares(&main_board);
}

fn verify_squares (cboard: &board::chessboard) {
    for x in 0..board::full_board_size {
        if x > 20 && x < 99 && x % 10 != 0 && x % 10 != 9 {
            assert!(cboard.layout[x] >= board::piece::Empty as u8);
            assert!(cboard.layout[x] <= board::piece::p as u8);
        } else {
            assert_eq!(cboard.layout[x], board::void_square);
        }
    }
}

#[test]
fn hashing() {
    zobrist::init();
    let mut main_board : board::chessboard = board::init();

    fen::parse("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", &mut main_board);
    let first_hash = main_board.zobrist;
    fen::parse("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 40 32", &mut main_board);
    let second_hash = main_board.zobrist;
    
    assert!(first_hash == second_hash);
    
    fen::parse("8/p7/Pp1p1rk1/1Pp2N2/4P1K1/3P3P/8/8 b - - 0 53", &mut main_board);
    let third_hash = main_board.zobrist;

    assert!(first_hash != third_hash);
}

#[test]
fn piece_list() {
    zobrist::init();
    let mut main_board : board::chessboard = board::init();

    fen::parse("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", &mut main_board);
    verify_pl(&main_board);

    fen::parse("8/p7/Pp1p1rk1/1Pp2N2/4P1K1/3P3P/8/8 b - - 0 53", &mut main_board);
    verify_pl(&main_board);

    fen::parse("r1bq1rk1/pp3ppp/3n4/2p1N3/2B5/7P/PPP2PP1/R1BQR1K1 w - h2 0 33", &mut main_board);
    verify_pl(&main_board);
}

fn verify_pl(cboard: &board::chessboard) {
    assert_eq!(cboard.piece_count[board::piece::k as usize], 1);
    assert_eq!(cboard.piece_count[board::piece::K as usize], 1);

    for i in 1..board::piece::p as usize{
        for x in 0..cboard.piece_count[i]{
            assert_eq!(cboard.layout[cboard.piece_list[i as usize][x as usize] as usize], i as u8);
        }
        assert_eq!(cboard.piece_list[i][cboard.piece_count[i] as usize+1], 0);
        if cboard.piece_count[i] != 0 {
            assert!(cboard.piece_list[i][cboard.piece_count[i] as usize - 1] != 0);
        }
    }
}

#[test]
fn castling() {
    zobrist::init();
    let mut main_board : board::chessboard = board::init();

    fen::parse("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", &mut main_board);
    verify_castle(&main_board);

    fen::parse("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", &mut main_board);
    verify_castle(&main_board);
}

fn verify_castle(cboard: &board::chessboard) {
    assert!(cboard.castling <= 15);
}
