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
            hash ^= zobrist[sboard.layout[board::chocolate[i] as usize] as usize][i];
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
            hash ^= EP[(sboard.en_passant % 10) as usize]; 
        }
    }
    
    return hash;
}
