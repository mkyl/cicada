use board;
use moves;
use movement;
use square;

use time;

const hash_map_size : usize = 0x100000;
const inf : i32 = 100000;

// TODO src https://chessprogramming.wikispaces.com/Simplified+evaluation+function

const black_pawn_piece_square : [i32; 120] =
   [0, 0,  0,  0,  0,  0,  0,  0,  0,  0,
    0,  0,  0,  0,  0,  0,  0,  0, 0,  0,
    0, 0,  0,  0,  0,  0,  0,  0,  0,  0,
    0, 50, 50, 50, 50, 50, 50, 50, 50, 0,
    0, 10, 10, 20, 30, 30, 20, 10, 10, 0,
    0,  5,  5, 10, 25, 25, 10,  5,  5, 0,
    0,  0,  0,  0, 20, 20,  0,  0,  0, 0,
    0, 5, -5,-10,  0,  0, -10, -5,  5, 0,
    0, 5, 10, 10,-20,-20, 10, 10,  5, 0,
    0,  0,  0,  0,  0,  0,  0,  0, 0, 0,
    0,  0,  0,  0,  0,  0,  0,  0, 0, 0,
    0,  0,  0,  0,  0,  0,  0,  0, 0, 0];

const white_pawn_piece_square : [i32; 120] =
   [0,  0,  0,  0,  0,  0,  0,  0, 0, 0,
    0,  0,  0,  0,  0,  0,  0,  0, 0, 0,
    0,  0,  0,  0,  0,  0,  0,  0, 0, 0,
    0, 5, 10, 10,-20,-20, 10, 10,  5, 0,
    0, 5, -5,-10,  0,  0, -10, -5,  5, 0,
    0,  0,  0,  0, 20, 20,  0,  0,  0, 0,
    0,  5,  5, 10, 25, 25, 10,  5,  5, 0,
    0, 10, 10, 20, 30, 30, 20, 10, 10, 0,
    0, 50, 50, 50, 50, 50, 50, 50, 50, 0,
    0, 0,  0,  0,  0,  0,  0,  0,  0,  0,
    0,  0,  0,  0,  0,  0,  0,  0, 0,  0,
    0, 0,  0,  0,  0,  0,  0,  0,  0,  0];

const black_knight_piece_square : [i32; 120] = 
   [0, 0,  0,  0,  0,  0,  0,  0,  0,  0,
    0,  0,  0,  0,  0,  0,  0,  0, 0,  0,
    0, -50,-40,-30,-30,-30,-30,-40,-50, 0,
    0, -40,-20,  0,  0,  0,  0,-20,-40, 0,
    0, -30,  0, 10, 15, 15, 10,  0,-30, 0,
    0, -30,  5, 15, 20, 20, 15,  5,-30, 0,
    0, -30,  0, 15, 20, 20, 15,  0,-30, 0,
    0, -30,  5, 10, 15, 15, 10,  5,-30, 0,
    0, -40,-20,  0,  5,  5,  0,-20,-40, 0,
    0, -50,-40,-30,-30,-30,-30,-40,-50, 0,
    0,  0,  0,  0,  0,  0,  0,  0, 0, 0,
    0,  0,  0,  0,  0,  0,  0,  0, 0, 0];

const white_knight_piece_square : [i32; 120] = 
    [0,  0,  0,  0,  0,  0,  0,  0, 0, 0,
    0,  0,  0,  0,  0,  0,  0,  0, 0, 0,
    0, -50,-40,-30,-30,-30,-30,-40,-50, 0,
    0, -40,-20,  0,  5,  5,  0,-20,-40, 0,
    0, -30,  5, 10, 15, 15, 10,  5,-30, 0,
    0, -30,  0, 15, 20, 20, 15,  0,-30, 0,
    0, -30,  5, 15, 20, 20, 15,  5,-30, 0,
    0, -30,  0, 10, 15, 15, 10,  0,-30, 0,
    0, -40,-20,  0,  0,  0,  0,-20,-40, 0,
    0, -50,-40,-30,-30,-30,-30,-40,-50, 0,
    0,  0,  0,  0,  0,  0,  0,  0, 0,  0,
    0, 0,  0,  0,  0,  0,  0,  0,  0,  0];

const black_bishop_piece_square : [i32; 120] =
   [0, 0,  0,  0,  0,  0,  0,  0,  0,  0,
    0,  0,  0,  0,  0,  0,  0,  0, 0,  0,
   0, -20,-10,-10,-10,-10,-10,-10,-20, 0,
   0, -10,  0,  0,  0,  0,  0,  0,-10, 0,
   0, -10,  0,  5, 10, 10,  5,  0,-10, 0,
   0, -10,  5,  5, 10, 10,  5,  5,-10, 0,
   0, -10,  0, 10, 10, 10, 10,  0,-10, 0,
   0, -10, 10, 10, 10, 10, 10, 10,-10, 0,
   0, -10,  5,  0,  0,  0,  0,  5,-10, 0,
   0, -20,-10,-10,-10,-10,-10,-10,-20, 0,
    0,  0,  0,  0,  0,  0,  0,  0, 0, 0,
    0,  0,  0,  0,  0,  0,  0,  0, 0, 0];

const white_bishop_piece_square : [i32; 120] =
    [0,  0,  0,  0,  0,  0,  0,  0, 0, 0,
    0,  0,  0,  0,  0,  0,  0,  0, 0, 0,
   0, -20,-10,-10,-10,-10,-10,-10,-20, 0,
   0, -10,  5,  0,  0,  0,  0,  5,-10, 0,
   0, -10, 10, 10, 10, 10, 10, 10,-10, 0,
   0, -10,  0, 10, 10, 10, 10,  0,-10, 0,
   0, -10,  5,  5, 10, 10,  5,  5,-10, 0,
   0, -10,  0,  5, 10, 10,  5,  0,-10, 0,
   0, -10,  0,  0,  0,  0,  0,  0,-10, 0,
   0, -20,-10,-10,-10,-10,-10,-10,-20, 0,
    0,  0,  0,  0,  0,  0,  0,  0, 0,  0,
    0, 0,  0,  0,  0,  0,  0,  0,  0,  0];

const black_rook_piece_square : [i32; 120] =
   [0, 0,  0,  0,  0,  0,  0,  0,  0,  0,
    0,  0,  0,  0,  0,  0,  0,  0, 0,  0,
    0,  0,  0,  0,  0,  0,  0,  0,  0, 0,
    0,  5, 10, 10, 10, 10, 10, 10,  5, 0,
    0, -5,  0,  0,  0,  0,  0,  0, -5, 0,
    0, -5,  0,  0,  0,  0,  0,  0, -5, 0,
    0, -5,  0,  0,  0,  0,  0,  0, -5, 0,
    0, -5,  0,  0,  0,  0,  0,  0, -5, 0,
    0, -5,  0,  0,  0,  0,  0,  0, -5, 0,
    0,  0,  0,  0,  5,  5,  0,  0,  0, 0,
    0,  0,  0,  0,  0,  0,  0,  0, 0, 0,
    0,  0,  0,  0,  0,  0,  0,  0, 0, 0];

const white_rook_piece_square : [i32; 120] =
    [0,  0,  0,  0,  0,  0,  0,  0, 0, 0,
    0,  0,  0,  0,  0,  0,  0,  0, 0, 0,
    0,  0,  0,  0,  5,  5,  0,  0,  0, 0,
    0, -5,  0,  0,  0,  0,  0,  0, -5, 0,
    0, -5,  0,  0,  0,  0,  0,  0, -5, 0,
    0, -5,  0,  0,  0,  0,  0,  0, -5, 0,
    0, -5,  0,  0,  0,  0,  0,  0, -5, 0,
    0, -5,  0,  0,  0,  0,  0,  0, -5, 0,
    0,  5, 10, 10, 10, 10, 10, 10,  5, 0,
    0,  0,  0,  0,  0,  0,  0,  0,  0, 0,
    0,  0,  0,  0,  0,  0,  0,  0, 0,  0,
    0, 0,  0,  0,  0,  0,  0,  0,  0,  0];

pub struct transposition {
    pub hash: u64,
    pub move_: u32
}

impl transposition {
    fn empty() -> transposition {
        transposition {
            hash: 0,
            move_: 0
        }
    }
}

pub struct transposition_table {
    pub entries: Vec<transposition> 
}

impl transposition_table {
    pub fn new() -> transposition_table {
        let mut vector = Vec::new();
        for x in 0..hash_map_size {
            vector.push(transposition::empty());
        }
        transposition_table{
            entries: vector
        }
    }
}

pub fn store_transposition(move_ : u32, cboard : &mut board::chessboard) {
    let x = cboard.zobrist % hash_map_size as u64;
    let i = x as usize;
    cboard.transposition_table.entries[i].hash = cboard.zobrist;
    cboard.transposition_table.entries[i].move_ = move_;
}

pub fn find_transposition(cboard: &board::chessboard) -> u32 {
    let x = cboard.zobrist % hash_map_size as u64;
    let i = x as usize;

    // TODO collisions here should be astronomically rare
    // but is this really the case?
    if cboard.zobrist == cboard.transposition_table.entries[i].hash {
        return cboard.transposition_table.entries[i].move_
    } else {
        println!(" [!] hash miss");
        return 0
    }
}

pub struct search {
    depth_target: u8,
    depth: u8,
    leaves: f64,

    think_time: u16,
    begin: time::PreciseTime,
    end: time::PreciseTime
}

impl search {
    pub fn new(time : u16, ply: u8) -> search {

        search{
            depth_target: ply,
            depth: 0,
            leaves: 0f64,

            think_time: time,
            begin: time::PreciseTime::now(),
            // TODO really unefficient
            end: time::PreciseTime::now()
        }
    }
}

pub fn repetition (cboard : &board::chessboard) -> bool{
    for x in (cboard.depth - cboard.fifty as u16) as usize..cboard.depth as usize - 1 {
        if cboard.zobrist == cboard.past[x].zobrist {
            return true
        }
    }
    false
}

fn evaluate (cboard: &board::chessboard) -> i32 {
    //  raw score        = white           - black
    let material_balance = cboard.score[0] - cboard.score[1];
    let mut score = material_balance;

    for x in 0..cboard.piece_count[board::piece::P as usize] as usize{
        let loc = cboard.piece_list[board::piece::P as usize][x];
        score += white_pawn_piece_square[loc as usize];
    }
   
    for x in 0..cboard.piece_count[board::piece::p as usize] as usize{
        let loc = cboard.piece_list[board::piece::p as usize][x];
        score -= black_pawn_piece_square[loc as usize];
    }

    for x in 0..cboard.piece_count[board::piece::N as usize] as usize {
        let loc = cboard.piece_list[board::piece::N as usize][x];
        score += white_knight_piece_square[loc as usize];
    }

    for x in 0..cboard.piece_count[board::piece::n as usize] as usize {
        let loc = cboard.piece_list[board::piece::n as usize][x];
        score -= black_knight_piece_square[loc as usize];
    }

    for x in 0..cboard.piece_count[board::piece::B as usize] as usize {
        let loc = cboard.piece_list[board::piece::B as usize][x];
        score += white_bishop_piece_square[loc as usize];
    }

    for x in 0..cboard.piece_count[board::piece::b as usize] as usize {
        let loc = cboard.piece_list[board::piece::b as usize][x];
        score -= black_bishop_piece_square[loc as usize];
    }

    for x in 0..cboard.piece_count[board::piece::R as usize] as usize {
        let loc = cboard.piece_list[board::piece::R as usize][x];
        score += white_rook_piece_square[loc as usize];
    }

    for x in 0..cboard.piece_count[board::piece::r as usize] as usize {
        let loc = cboard.piece_list[board::piece::r as usize][x];
        score -= black_rook_piece_square[loc as usize];
    }

    if cboard.side == board::white {
        return score
    } else {
        return -score
    }
}

pub fn start(cboard: &mut board::chessboard, depth_target: u8, think_time:u16) {
    let mut best : u32 = 0;
    let mut score = 0;
    cboard.ply = 0;

    for depth in 1..depth_target+1 {
        let mut node = 0f64;
        score = alpha_beta(-inf, inf, depth, cboard, &mut node);
        println!("info depth {} cp {} nodes {}", depth, score, node);
    }
}

fn alpha_beta(alpha: i32, beta: i32, depth: u8, cboard: &mut board::chessboard, node : &mut f64) -> i32 {
    if depth == 0 {
        return evaluate(cboard)
    }

    let mut new_alpha = alpha;
    let stale_alpha = alpha;
    let mut illegal = true;
    let mut move_ = 0;
    let mut score = -inf;

    if (repetition(cboard) || cboard.fifty > 99) && cboard.ply > 0 {
        return 0
    }

    if cboard.depth == board::max_game_length as u16 - 1 {
        return evaluate(cboard)
    }

    let mut move_list : moves::movelist =  moves::movelist::new();
    moves::generator(&mut move_list, cboard);


    for index in 0..move_list.count as usize {
        optimize_mvv_lva(index, &mut move_list);

        if !movement::make(&move_list.all[index], cboard) {
            continue
        }


        *node += 1f64;

        if illegal == true {
            illegal = false;
        }

        score = -alpha_beta(-beta, -new_alpha, depth - 1, cboard, node);
        movement::undo(cboard);

        if score > new_alpha {
            if score >= beta {
                return beta
            }
            new_alpha = score;
            move_ = move_list.all[index].container;
        }
    }

    if illegal {
        // checkmate
        if cboard.side == board::white {
            if square::attacked(cboard.piece_list[board::piece::K as usize][0], board::black, cboard) {
                return -board::piece_value[1] as i32 + cboard.ply as i32;
            } else {
                // stalemate
                return 0
            }
        } else {
            if square::attacked(cboard.piece_list[board::piece::k as usize][0], board::white, cboard) {
                return -board::piece_value[1] as i32 + cboard.ply as i32;
            } else {
                // stalemate
                return 0
            }
        }
    }

    if stale_alpha != new_alpha {
        store_transposition(move_, cboard);
    }
    
    return new_alpha
}

fn optimize_mvv_lva(index : usize, list : &mut moves::movelist) {
    let mut max_score = list.all[index].score;
    let mut max_index = index;
    let old_move = moves::_move{
        container: list.all[index].container,
        score: list.all[index].score};

    for x in index .. list.count as usize {
        if list.all[x].score > max_score {
            max_score = list.all[x].score;
            max_index = x;
        }
    }

    list.all[index].score = list.all[max_index].score;
    list.all[index].container = list.all[max_index].container;
    list.all[max_index] = old_move;
}
