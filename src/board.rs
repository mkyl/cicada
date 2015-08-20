pub static mut chocolate : [u8; 64] = [0; 64];

pub const full_board_size: usize = 120;
pub const playable_size: usize = 64;
pub const void_square: u8 = 100; 
const max_game_length: usize = 1024;
const white: bool = false;
const black: bool = true;

pub enum piece {
    Empty,
    P,
    N,
    B,
    R,
    Q,
    K,
    p,
    n,
    b,
    r,
    q,
    k
}

enum file {
    file_a,
    file_b,
    file_c,
    file_d,
    file_e,
    file_f,
    file_g,
    file_h,
    invalid_file
}

enum rank {
    rank_1,
    rank_2,
    rank_3,
    rank_4,
    rank_5,
    rank_6,
    rank_7,
    rank_8,
    invalid_rank
}

pub enum castling_bits {
    K_cp = 1 << 0,
    Q_cp = 1 << 1,
    k_cp = 1 << 2,
    q_cp = 1 << 3
}
    

pub enum square {
    A1 = 21, B1, C1, D1, E1, F1, G1, H1,
    A2 = 31, B2, C2, D2, E2, F2, G2, H2,
    A3 = 41, B3, C3, D3, E3, F3, G3, H3,
    A4 = 51, B4, C4, D4, E4, F4, G4, H4,
    A5 = 61, B5, C5, D5, E5, F5, G5, H5,
    A6 = 71, B6, C6, D6, E6, F6, G6, H6,
    A7 = 81, B7, C7, D7, E7, F7, G7, H7,
    A8 = 91, B8, C8, D8, E8, F8, G8, H8
}


struct snapshot {
    move_key: u16,

    castling: u8,
    en_passant: u8,
    fifty: u8,

    zobrist: u64
}

pub struct chessboard {
    // array containing board
    // see ../theory.md for explanation of layout
    pub layout: [u8; full_board_size],
    piece_list: [[u8; 10]; 13],
    
    // number of half moves
    // engine has looked ahead
    ply: u8,

    // how many half moves have been played so far
    depth: u16,
    // used for FIDE rule 9.3
    fifty: u8,
    // stores index of EP square, if any
    pub en_passant: u8,
    pub castling: u8,
    pub side: bool,

    past: [snapshot; max_game_length],

    zobrist: u64
}


pub fn init() -> (chessboard) {
    use std::mem;

    let mut counter = 0;
    for x in 0..full_board_size {
        if x > 20 && x < 99 && x % 10 != 0 && x % 10 != 9 {
            unsafe {
                chocolate[counter] = x as u8;
            }
            counter += 1;
        }
    }

    let mut new_board : chessboard = chessboard{
        layout: [0; full_board_size],
        piece_list: [[0; 10]; 13],
        ply: 0,
        depth: 0,
        fifty: 0,
        en_passant: void_square,
        castling: 0,
        side: white,
        past: unsafe { mem::zeroed() } ,
        zobrist: 0
    }; 

    reset(&mut new_board);

    return new_board;
    /*
    for x in 0..12 {
        println!(" {} {} {} {} {} {} {} {} {} {} ", 
                 new_board.layout[0 + 10 * x],
                 new_board.layout[1 + 10 * x],
                 new_board.layout[2 + 10 *x],
                 new_board.layout[3 + 10 *x],
                 new_board.layout[4 + 10 *x],
                 new_board.layout[5 + 10 *x],
                 new_board.layout[6 + 10 *x],
                 new_board.layout[7 + 10 *x],
                 new_board.layout[8 + 10 *x],
                 new_board.layout[9 + 10 *x]);
    }

    for x in 0..64 {
        println!(" {} ", unsafe{chocolate[x]});
    }

    println!("zobrist hash stored: {}", new_board.zobrist);
    println!("zobrist hash recalc: {}", zobrist::hash(&new_board));
    */
}

fn reset (board: &mut chessboard) {
    use std::mem;
    use zobrist;

    for i in 0..full_board_size {
        board.layout[i] = void_square;
    }

    for i in 0..playable_size {
        unsafe {
            board.layout[chocolate[i] as usize] = piece::Empty as u8;
        }
    }

    for i in 0..piece::k as usize {
        for x in 0..10 {
            board.piece_list[i][x] = 0;
        }
    }

    board.castling = castling_bits::K_cp as u8 | castling_bits::Q_cp as u8 
        | castling_bits::k_cp as u8 | castling_bits::q_cp as u8;

    board.en_passant = void_square;

    board.side = white;

    board.past = { unsafe { mem::zeroed() } };

    board.zobrist = zobrist::hash(board);
}
