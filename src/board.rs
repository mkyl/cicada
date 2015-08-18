
pub const full_board_size: usize = 120;
pub const void: u8 = 100; 
const max_game_length: usize = 1024;
const white: bool = false;
const black: bool = true;

enum piece {
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

enum castling_bits {
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


struct past {
    move_key: u16,
    castling: u8,
    en_passant: u8,
    fifty: u8
}

pub struct chessboard {
    // array containing board
    // see ../theory.md for explanation of layout
    pub layout: [u8; full_board_size],
    piece_list: [[u8; 13]; 10],
    
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

    history: [past; max_game_length],

    zobrist: u64
}

pub fn init() -> () {

}
