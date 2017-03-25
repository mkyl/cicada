use think;

pub static mut chocolate : [u8; 64] = [0; 64];
pub const full_board_size: usize = 120;
pub const playable_size: usize = 64;
pub const void_square: u8 = 100;
pub const max_game_length: usize = 1024;
pub const white: bool = false;
pub const black: bool = true;
pub const piece_value: [i32; 13] = [0, 31337, 900, 500, 330, 320, 100, 31337, 900, 500, 330, 320, 100];

// standard FEN notation:
// capitals = white
// small = black

pub enum piece {
    Empty,
    K,
    Q,
    R,
    B,
    N,
    P,
    k,
    q,
    r,
    b,
    n,
    p
}

pub enum file {
    file_a = 0,
    file_h = 7,
}

pub enum rank {
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


pub struct snapshot {
    pub move_key: u32,

    pub castling: u8,
    pub en_passant: u8,
    pub fifty: u8,

    pub zobrist: u64
}

pub struct chessboard {
    // array containing board
    // see ../theory.md for explanation of layout
    pub layout: [u8; full_board_size],
    pub piece_count: [u8; 13],
    pub piece_list: [[u8; 10]; 13],
    pub score: [i32; 2],

    // number of half moves
    // engine has looked ahead
    pub ply: u8,

    // how many half moves have been played so far
    pub depth: u16,
    // used for FIDE rule 9.3
    pub fifty: u8,
    // stores index of EP square, if any
    pub en_passant: u8,
    pub castling: u8,
    pub side: bool,

    pub past: [snapshot; max_game_length],
    pub transposition_table: think::transposition_table,

    pub zobrist: u64
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
        piece_count: [0; 13],
        score: [0; 2],
        ply: 0,
        depth: 0,
        fifty: 0,
        en_passant: void_square,
        castling: 0,
        side: white,
        past: unsafe { mem::zeroed() },
        transposition_table: think::transposition_table::new(),
        zobrist: 0
    };

    reset(&mut new_board);

    new_board
}

pub fn reset (board: &mut chessboard) {
    use std::mem;
    use zobrist;

    for mut l in &mut board.layout[0..full_board_size] {
        *l = void_square;
    }

    unsafe {
        for c in &chocolate[..playable_size] {
            board.layout[*c as usize] = piece::Empty as u8;
        }
    }

    for i in 0..piece::p as usize {
        for x in 0..10 {
            board.piece_list[i][x] = 0;
        }
    }

    board.piece_count = [0; 13];

    board.score = [0; 2];

    board.castling = castling_bits::K_cp as u8 | castling_bits::Q_cp as u8
        | castling_bits::k_cp as u8 | castling_bits::q_cp as u8;

    board.en_passant = void_square;

    board.side = white;

    board.past = { unsafe { mem::zeroed() } };

    board.zobrist = zobrist::hash(board);
}

pub fn AN_to_chocolate (file : char, rank : u8) -> (u8) {
    let rank_index = rank - b'1';
    let file_index = file as u8 - b'a';

    (rank_index + 2) * 10 + file_index + 1
}

pub fn AN_to_board (file : u8, rank : u8) -> (u8) {
    (rank + 2) * 10 + file + 1
}

pub fn to_AN(square : u8) -> [char; 2] {
    let mut answer : [char; 2] = ['0'; 2];
    answer[0] = (b'a' + (square % 10 - 1)) as char;
    answer[1] = (b'1' + (square / 10 - 2)) as char;
    answer
}

pub fn update_pieces (cboard: &mut chessboard) {
    unsafe {
        for index in &chocolate[..playable_size] {
            let piece = cboard.layout[*index as usize];

            if piece != piece::Empty as u8 {
                cboard.piece_list[piece as usize][cboard.piece_count[piece as usize] as usize] = *index;
                cboard.piece_count[piece as usize] += 1;
                if piece < 7 {
                    // white
                    cboard.score[0] += piece_value[piece as usize];
                } else {
                    cboard.score[1] += piece_value[piece as usize];
                }
            }
        }
    }
}

pub fn print (cboard: &chessboard) {
    for x in (0..rank::rank_8 as u8 + 1).rev() {
        print!("{}  ", x as u8 + 1);
        for i in 0..file::file_h as u8 + 1{
            print!( "{:2} ", cboard.layout[AN_to_board(i,x) as usize]);
        }
        println!("");
    }
    println!("\n    A  B  C  D  E  F  G  H");

    println!("Castling: {}, Side: {}, En Passant: {}", cboard.castling, cboard.side, cboard.en_passant);
    println!("Moves: {}, Fifty: {}", cboard.depth, cboard.fifty);
    for i in 0..piece::p as usize + 1 {
        print!("{}: {} pieces | ", i,  cboard.piece_count[i]);
    }
    println!("white king: {}, black king: {}", cboard.piece_list[piece::K as usize][0], cboard.piece_list[piece::k as usize][0]);
    println!("1 white rook: {}, 2 white rook: {}", cboard.piece_list[piece::R as usize][0], cboard.piece_list[piece::R as usize][1]);
    println!("\nHash: {}", cboard.zobrist);
    println!();
}
