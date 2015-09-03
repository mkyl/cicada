use board;
use board::chessboard;

static mut zobrist: [[u64; board::full_board_size]; 13] = [[0; board::full_board_size]; 13];
static mut castling: [u64; 16] = [0; 16];
static mut side: u64 = 0;
static mut EP: [u64; 8] = [0; 8];

pub fn init() {
    use rand::{thread_rng, Rng};
    let mut rng = thread_rng();

    for x in 0..13 {
        for i in 0..board::full_board_size {
            unsafe {
                zobrist[x][i] = rng.gen();
            }
        }
    }

    for x in 0..16 {
        unsafe {
            castling[x] = rng.gen();
        }
    }

    for x in 0..8 {
        unsafe {
            EP[x] = rng.gen();
        }
    }

    unsafe {
        side = rng.gen();
    }
}

pub fn hash(sboard: &board::chessboard) -> u64 {
    let mut hash : u64 = 0;

    for i in 0..board::playable_size {
        unsafe {
            if sboard.layout[board::chocolate[i] as usize] != board::piece::Empty as u8 {
                hash ^= zobrist[sboard.layout[board::chocolate[i] as usize] as usize][board::chocolate[i] as usize];
            }
        }
    }

    unsafe {
        hash ^= castling[sboard.castling as usize];
    }

    if sboard.side {
        unsafe {
            hash ^= side;
        }
    }

    if sboard.en_passant != board::void_square {
        unsafe {
            // mod 10 to find file
            hash ^= EP[(sboard.en_passant % 10) as usize - 1]; 
        }
    }
    
    return hash;
}

pub fn hash_square(target : u8, cboard: &mut board::chessboard) {
    unsafe {
        if cboard.layout[target as usize] != board::piece::Empty as u8 {
            cboard.zobrist ^= zobrist[cboard.layout[target as usize] as usize][target as usize];
        }
    }
}

pub fn castle(cboard: &mut board::chessboard) {
    unsafe {
        cboard.zobrist ^= castling[cboard.castling as usize];
    }
}

pub fn en_passant(cboard: &mut board::chessboard) {
    if cboard.en_passant != board::void_square {
        unsafe {
            // mod 10 to find file
            cboard.zobrist ^= EP[(cboard.en_passant % 10) as usize - 1]; 
        }
    }
}

pub fn sides(cboard: &mut board::chessboard) {
    if cboard.side {
        unsafe {
            cboard.zobrist ^= side;
        }
    }
}
