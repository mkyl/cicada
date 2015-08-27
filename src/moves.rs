use board;

const max_moves : usize = 256;
const around : [i8; 2] = [1, -1];

pub struct _move {
    /* 0000 0000 0000 0000 0000 0000 0000
     * 000C DEpp ppcc ccff ffff fttt tttt */
    pub container : u32,
    pub score: u8 
}

impl _move {
    pub fn new(from : u8, to : u8, captured : u8, promoted : u8, en_passant : bool, pawn_double : bool, castling : bool) -> _move {
        let mut combined : u32 = 0;
        combined |= from as u32;
        combined |= (to as u32) << 7;
        combined |= (captured as u32) << 14;
        combined |= (promoted as u32) << 18; 
        combined |= (en_passant as u32) << 22;
        combined |= (pawn_double as u32) << 23;
        combined |= (castling as u32) << 24;

        _move {
           container : combined,
           score: 0
        }
    }
}

pub struct movelist {
    pub all : [_move; max_moves],
    pub count: u8
}

impl movelist {
    pub fn new() -> movelist {
        use std::mem;
        movelist {
            all: unsafe{ mem::zeroed() },
            count: 0
        }
    }
}

pub fn add_move(m : _move, list : &mut movelist) {
    list.all[list.count as usize] = m;
    list.count += 1;
}

pub fn from(m : &_move) -> u8 {
    (m.container & 0x7f) as u8
}

pub fn to(m : &_move) -> u8 {
    (m.container >> 7 & 0x7f) as u8
}

pub fn captured(m : &_move) -> u8 {
    (m.container >> 14 & 0xf) as u8
}

pub fn promoted(m : &_move) -> u8 {
    (m.container >> 18 & 0xf) as u8
}

pub fn en_passant(m : &_move) -> bool {
    (m.container >> 22 & 1) == 1
}

pub fn pawn_double(m : &_move) -> bool {
    (m.container >> 23 & 1) == 1
}

pub fn castling(m : &_move) -> bool {
    (m.container >> 24 & 1) == 1
}

pub fn to_AN(m : &_move) -> [char; 5]{
    let mut result = [' '; 5];

    result[0] = board::to_AN(from(m))[0];
    result[1] = board::to_AN(from(m))[1];
    result[2] = board::to_AN(to(m))[0];
    result[3] = board::to_AN(to(m))[1];

    if promoted(m) != 0 {
        result[4] = board::AN_pieces[promoted(m) as usize];
    }

    result
}

pub fn generator(list : &mut movelist, cboard : &board::chessboard) {
    let mut baseline : usize = 0;
    if cboard.side == board::black {
        baseline = 6;
    }

    for x in (1 + baseline)..(board::piece::k as usize + baseline) {
        for i in 0..cboard.piece_count[x] as usize{
            let piece = cboard.piece_list[x][i];
            // TODO more elegant way
            match x - baseline {
                1 => { // king
                },

                2 => { // queen
                },

                3 => { // rook
                },

                4 => { // bishop
                },

                5 => { // knight
                },

                6 => { // pawn
                    let mut dir : i8 = 10;
                    if baseline != 0 { dir = -10 };
                    
                    // quiet moves
                    if cboard.layout[piece.wrapping_add(dir as u8) as usize]
                            == board::piece::Empty as u8 {
                        if baseline == 0 {
                            if piece / 10 == 8 {
                                add_move(_move::new(piece, piece.wrapping_add(dir as u8), 0,
                                    board::piece::Q as u8, false, false, false), list);
                                add_move(_move::new(piece, piece.wrapping_add(dir as u8), 0,
                                    board::piece::R as u8, false, false, false), list);
                                add_move(_move::new(piece, piece.wrapping_add(dir as u8), 0,
                                    board::piece::B as u8, false, false, false), list);
                                add_move(_move::new(piece, piece.wrapping_add(dir as u8), 0,
                                    board::piece::N as u8, false, false, false), list);
                            } else {
                                if piece / 10 == 3 && cboard.layout[piece.wrapping_add((dir * 2) as u8)
                                                                as usize]== board::piece::Empty as u8 {
                                    add_move(_move::new(piece, piece.wrapping_add((dir * 2) as u8), 0, 0, false, true, false), list);
                                }
                                add_move(_move::new(piece, piece.wrapping_add(dir as u8), 0, 0, false, false, false), list);
                            }
                        } else {
                            if piece / 10 == 3 {
                                add_move(_move::new(piece, piece.wrapping_add(dir as u8), 0,
                                    board::piece::q as u8, false, false, false), list);
                                add_move(_move::new(piece, piece.wrapping_add(dir as u8), 0,
                                    board::piece::r as u8, false, false, false), list);
                                add_move(_move::new(piece, piece.wrapping_add(dir as u8), 0, 
                                    board::piece::b as u8, false, false, false), list);
                                add_move(_move::new(piece, piece.wrapping_add(dir as u8), 0,
                                    board::piece::n as u8, false, false, false), list);
                            } else {
                                if piece / 10 == 8 && cboard.layout[piece.wrapping_add((dir * 2) as u8)
                                                                as usize]== board::piece::Empty as u8 {
                                    add_move(_move::new(piece, piece.wrapping_add((dir * 2) as u8), 0, 0, false, true, false), list);
                                }
                                add_move(_move::new(piece, piece.wrapping_add(dir as u8), 0, 0, false, false, false), list);
                            }
                        }
                    }

                    // captures
                    for y in 0..2 {
                        let offset = around[y] + dir;
                        let target = cboard.layout[piece.wrapping_add(offset as u8) as usize];
                        // TODO this condition may be too computation-heavy
                        if target != board::void_square &&
                            ((target != board::piece::Empty as u8 &&
                            (target < baseline as u8 + 1 || target > baseline as u8 + 6)) ||
                            (piece.wrapping_add(offset as u8) == cboard.en_passant)) {
                            if (baseline == 0) && (piece / 10 == 8) {
                                add_move(_move::new(piece, piece.wrapping_add(offset as u8), target,
                                    board::piece::Q as u8, false, false, false), list);
                                add_move(_move::new(piece, piece.wrapping_add(offset as u8), target,
                                    board::piece::R as u8, false, false, false), list);
                                add_move(_move::new(piece, piece.wrapping_add(offset as u8), target,
                                    board::piece::B as u8, false, false, false), list);
                                add_move(_move::new(piece, piece.wrapping_add(offset as u8), target,
                                    board::piece::N as u8, false, false, false), list);
                            } else if (baseline != 0) && (piece / 10 == 3) {
                                add_move(_move::new(piece, piece.wrapping_add(offset as u8), target,
                                    board::piece::q as u8, false, false, false), list);
                                add_move(_move::new(piece, piece.wrapping_add(offset as u8), target,
                                    board::piece::r as u8, false, false, false), list);
                                add_move(_move::new(piece, piece.wrapping_add(offset as u8), target, 
                                    board::piece::b as u8, false, false, false), list);
                                add_move(_move::new(piece, piece.wrapping_add(offset as u8), target,
                                    board::piece::n as u8, false, false, false), list);
                            } else {
                                add_move(_move::new(piece, piece.wrapping_add(offset as u8), target,
                                     0, false, false, false), list);
                            }
                        }
                    }
                },

                _ => {panic!("invalid piece");}
            }
        }
    }
}
