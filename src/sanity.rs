use board;
use moves;
use movement;

#[test]
fn layout() {
    use fen;
    use zobrist;
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
    use fen;
    use zobrist;
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
fn move_make_hash() {
    use fen;
    use zobrist;
    zobrist::init();
    let mut main_board : board::chessboard = board::init();

    fen::parse("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", &mut main_board);

    let move1 = moves::_move::new(22, 43, 0, 0, false, false, false, 0);
    movement::make(&move1, &mut main_board);
    assert_eq!(zobrist::hash(&main_board), main_board.zobrist);

    let move2 = moves::_move::new(97, 78, 0, 0, false, false, false, 0);
    movement::make(&move2, &mut main_board);
    assert_eq!(zobrist::hash(&main_board), main_board.zobrist);

    let move3 = moves::_move::new(35, 45, 0, 0, false, false, false, 0);
    movement::make(&move3, &mut main_board);
    assert_eq!(zobrist::hash(&main_board), main_board.zobrist);

    let move4 = moves::_move::new(82, 72, 0, 0, false, false, false, 0);
    movement::make(&move4, &mut main_board);
    assert_eq!(zobrist::hash(&main_board), main_board.zobrist);
}

#[test]
fn move_undo_hash() {
    use fen;
    use zobrist;
    zobrist::init();
    let mut main_board : board::chessboard = board::init();

    fen::parse("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", &mut main_board);

    let move1 = moves::_move::new(22, 43, 0, 0, false, false, false, 0);
    movement::make(&move1, &mut main_board);
    let move2 = moves::_move::new(97, 78, 0, 0, false, false, false, 0);
    movement::make(&move2, &mut main_board);
    let move3 = moves::_move::new(35, 45, 0, 0, false, false, false, 0);
    movement::make(&move3, &mut main_board);
    let move4 = moves::_move::new(82, 72, 0, 0, false, false, false, 0);
    movement::make(&move4, &mut main_board);

    for x in 0..4 {
        movement::undo(&mut main_board);
        assert_eq!(zobrist::hash(&main_board), main_board.zobrist);
    }
}

fn verify_hash(main_board : &board::chessboard) {
    use zobrist;
    assert_eq!(zobrist::hash(&main_board), main_board.zobrist);
}

#[test]
fn piece_list() {
    use fen;
    use zobrist;
    zobrist::init();
    let mut main_board : board::chessboard = board::init();

    fen::parse("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", &mut main_board);
    verify_pl(&main_board);

    fen::parse("8/p7/Pp1p1rk1/1Pp2N2/4P1K1/3P3P/8/8 b - - 0 53", &mut main_board);
    verify_pl(&main_board);

    fen::parse("r1bq1rk1/pp3ppp/3n4/2p1N3/2B5/7P/PPP2PP1/R1BQR1K1 w - h2 0 33", &mut main_board);
    verify_pl(&main_board);

    fen::parse("rnbqkb1r/pp1p1pPp/8/2p1pP2/1P1P4/3P3P/P1P1P3/RNBQKBNR w KQkq - 0 1", &mut main_board);
    verify_pl(&main_board);
}

fn verify_pl(cboard: &board::chessboard) {
    assert_eq!(cboard.piece_count[board::piece::k as usize], 1);
    assert_eq!(cboard.piece_count[board::piece::K as usize], 1);

    for i in 1..board::piece::p as usize{
        for x in 0..cboard.piece_count[i]{
            assert_eq!(cboard.layout[cboard.piece_list[i as usize][x as usize] as usize], i as u8);
        }
        if cboard.piece_count[i] != 0 {
            assert!(cboard.piece_list[i][cboard.piece_count[i] as usize - 1] != 0);
        }
    }
}

#[test]
fn castling() {
    use fen;
    use zobrist;
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

#[test]
fn repetition() {
    use fen;
    use zobrist;
    use think;
    zobrist::init();
    let mut main_board : board::chessboard = board::init();

    fen::parse("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", &mut main_board);

    movement::make(&moves::_move::new(22, 43, 0, 0, false, false, false,0), &mut main_board);
    assert_eq!(think::repetition(&main_board), false);
    movement::make(&moves::_move::new(92, 73, 0, 0, false, false, false, 0), &mut main_board);
    assert_eq!(think::repetition(&main_board), false);
    movement::make(&moves::_move::new(43, 22, 0, 0, false, false, false, 0), &mut main_board);
    assert_eq!(think::repetition(&main_board), false);
    movement::make(&moves::_move::new(73, 92, 0, 0, false, false, false, 0), &mut main_board);
    assert!(think::repetition(&main_board));
    movement::make(&moves::_move::new(92, 73, 0, 0, false, false, false, 0), &mut main_board);
    assert!(think::repetition(&main_board));
    movement::make(&moves::_move::new(73, 65, 0, 0, false, false, false, 0), &mut main_board);
    assert_eq!(think::repetition(&main_board), false);
}

pub fn sane(cboard: &board::chessboard) -> bool {
    let result : bool = true;

     verify_squares(cboard);
     verify_pl(cboard);
     verify_castle(cboard);
     verify_hash(cboard);

    result
}

static mut leafnodes : f64 = 0f64;

pub fn perft(depth : i32, cboard : &mut board::chessboard) {
    if depth == 0 {
        unsafe {
            leafnodes += 1f64;
            return;
        }
    }

    let mut move_list : moves::movelist =  moves::movelist::new();
    moves::generator(&mut move_list, cboard);

    for x in 0..move_list.count as usize {
        if !movement::make(&move_list.all[x], cboard) {
            continue
        }
        perft(depth - 1, cboard);
        movement::undo(cboard);
    }
}

pub fn perft_test(depth: i32, cboard : &mut board::chessboard) -> f64 {
    unsafe {
        println!("Perft test to depth: {}", depth);
        leafnodes = 0f64;

        let mut move_list : moves::movelist =  moves::movelist::new();
        moves::generator(&mut move_list, cboard);

        for x in 0..move_list.count as usize {
            if !movement::make(&move_list.all[x], cboard) {
                continue
            }
            perft(depth - 1, cboard);
            movement::undo(cboard);
        }

        println!("Test Complete: {} nodes", leafnodes);
        leafnodes
    }
}

#[test]
pub fn perft_suite() {
    use fen;
    use zobrist;
    use think;
    zobrist::init();
    let mut main_board : board::chessboard = board::init();

    fen::parse("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1", &mut main_board);
    assert_eq!(perft_test(3,  &mut main_board), 97862f64);

    fen::parse("n1n5/PPPk4/8/8/8/8/4Kppp/5N1N b - - 0 1", &mut main_board);
    assert_eq!(perft_test(4,  &mut main_board), 182838f64);

    fen::parse("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", &mut main_board);
    assert_eq!(perft_test(4,  &mut main_board), 197281f64);
}
