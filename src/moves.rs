use board;
use square;

const maxMoves : usize = 256;
const around : [i8; 2] = [1, -1];
const MVV : [u16; 13] = [0, 0, 5000, 4000, 3000, 2000, 1000, 0, 5000, 4000, 3000, 2000, 1000];
const AN_pieces: [char; 13] = ['x', 'k', 'q', 'r', 'b', 'n', 'p', 'k', 'q', 'r', 'b', 'n', 'p'];

pub struct Move {
    /* 0000 0000 0000 0000 0000 0000 0000
     * 000C DEpp ppcc ccff ffff fttt tttt */
    pub container : u32,
    pub score: u16
}

impl Move {
    pub fn new(from : u8, to : u8, captured : u8, promoted : u8, en_passant : bool, pawn_double : bool, castling : bool, scoring : u16) -> Move {
        // TODO double check on EP in move generator
        let mut combined : u32 = 0;
        combined |= from as u32;
        combined |= (to as u32) << 7;
        combined |= (captured as u32) << 14;
        combined |= (promoted as u32) << 18;
        combined |= (en_passant as u32) << 22;
        combined |= (pawn_double as u32) << 23;
        combined |= (castling as u32) << 24;

        Move {
           container : combined,
           score: scoring
        }
    }
}

pub struct movelist {
    pub all : [Move; maxMoves],
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

pub fn addMove(m : Move, list : &mut movelist) {
    list.all[list.count as usize] = m;
    list.count += 1;
}

pub fn from(m : &Move) -> u8 {
    (m.container & 0x7f) as u8
}

pub fn to(m : &Move) -> u8 {
    (m.container >> 7 & 0x7f) as u8
}

pub fn capture(m : &Move) -> u8 {
    (m.container >> 14 & 0xf) as u8
}

pub fn promoted(m : &Move) -> u8 {
    (m.container >> 18 & 0xf) as u8
}

pub fn en_passant(m : &Move) -> bool {
    (m.container >> 22 & 1) == 1
}

pub fn pawn_double(m : &Move) -> bool {
    (m.container >> 23 & 1) == 1
}

pub fn castling(m : &Move) -> bool {
    (m.container >> 24 & 1) == 1
}

pub fn to_AN(m : &Move) -> [char; 5]{
    let mut result = [' '; 5];

    result[0] = board::to_AN(from(m))[0];
    result[1] = board::to_AN(from(m))[1];
    result[2] = board::to_AN(to(m))[0];
    result[3] = board::to_AN(to(m))[1];

    if promoted(m) != 0 {
        result[4] = AN_pieces[promoted(m) as usize];
    }

    result
}

pub fn from_AN(move_str : &[u8], cboard : &board::chessboard) -> Move{
    let mut move_list : movelist =  movelist::new();
    generator(&mut move_list, cboard);

    let from_sq = board::AN_to_board(move_str[0] - b'a', move_str[1] - b'1');
    let to_sq   = board::AN_to_board(move_str[2] - b'a', move_str[3] - b'1');
    let mut prom : u8 = 0;

    if move_str.len() == 5 {
        if cboard.side == board::WHITE {
            match move_str[4] as char{
                'q' => prom = board::piece::Q as u8,
                'r' => prom = board::piece::R as u8,
                'b' => prom = board::piece::B as u8,
                'n' => prom = board::piece::N as u8,
                _   => unreachable!()
            }
        } else {
            match move_str[4] as char{
                'q' => prom = board::piece::q as u8,
                'r' => prom = board::piece::r as u8,
                'b' => prom = board::piece::b as u8,
                'n' => prom = board::piece::n as u8,
                _   => unreachable!()
            }
        }
    }

    for x in 0..move_list.count as usize {
        let result = &move_list.all[x];
        if to(result) == to_sq && from(result) == from_sq && promoted(result) == prom {
            return Move{container: result.container, score : 0}
        }
    }
    panic!("Illegal Move");
}

pub fn generator(list : &mut movelist, cboard : &board::chessboard) {
    let baseline = if cboard.side == board::BLACK { 6 } else { 0 };

    for x in (1 + baseline)..(board::piece::k as usize + baseline) {
        for i in 0..cboard.piece_count[x] as usize{
            let piece = cboard.piece_list[x][i];
            // TODO more elegant way
            match x - baseline {
                1 => { // king
                    for y in 0..8 {
                        let target = cboard.layout[(piece.wrapping_add(square::king[y] as u8)) as usize];
                        if target != board::VOID_SQUARE && (target < baseline as u8 + 1 || target > baseline as u8 + 6) {
                            if target == board::piece::Empty as u8 {
                                addMove(Move::new(piece, piece.wrapping_add(square::king[y] as u8), 0, 0,
                                false, false, false, 0), list);
                            } else {
                                addMove(Move::new(piece, piece.wrapping_add(square::king[y] as u8), target, 0,
                                false, false, false, MVV[target as usize] + 600), list);
                            }
                        }
                    }
                    // castling
                    if baseline == 0 {
                        if cboard.castling as u8 & board::CastlingBits::K_cp as u8 != 0 {
                            if cboard.layout[board::square::F1 as usize] == board::piece::Empty as u8 {
                                if cboard.layout[board::square::G1 as usize] == board::piece::Empty as u8 {
                                    if !square::attacked(board::square::E1 as u8, board::BLACK, cboard) &&
                                        !square::attacked(board::square::F1 as u8, board::BLACK, cboard) {
                                        addMove(Move::new(piece, board::square::G1 as u8, 0, 0,
                                            false, false, true, 0), list);
                                    }
                                }
                            }
                        }
                        if cboard.castling as u8 & board::CastlingBits::Q_cp as u8 != 0 {
                            if cboard.layout[board::square::D1 as usize] == board::piece::Empty as u8 {
                                if cboard.layout[board::square::C1 as usize] == board::piece::Empty as u8 {
                                    if cboard.layout[board::square::B1 as usize] == board::piece::Empty as u8 {
                                        if !square::attacked(board::square::E1 as u8, board::BLACK, cboard) &&
                                            !square::attacked(board::square::D1 as u8, board::BLACK, cboard) {
                                            addMove(Move::new(piece, board::square::C1 as u8, 0, 0,
                                                false, false, true, 0), list);
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        if cboard.castling as u8 & board::CastlingBits::k_cp as u8 != 0 {
                            if cboard.layout[board::square::F8 as usize] == board::piece::Empty as u8 {
                                if cboard.layout[board::square::G8 as usize] == board::piece::Empty as u8 {
                                    if !square::attacked(board::square::E8 as u8, board::WHITE, cboard) &&
                                        !square::attacked(board::square::F8 as u8, board::WHITE, cboard) {
                                        addMove(Move::new(piece, board::square::G8 as u8, 0, 0,
                                            false, false, true, 0), list);
                                    }
                                }
                            }
                        }
                        if cboard.castling as u8 & board::CastlingBits::q_cp as u8 != 0 {
                            if cboard.layout[board::square::D8 as usize] == board::piece::Empty as u8 {
                                if cboard.layout[board::square::C8 as usize] == board::piece::Empty as u8 {
                                    if cboard.layout[board::square::B8 as usize] == board::piece::Empty as u8 {
                                        if !square::attacked(board::square::E8 as u8, board::WHITE, cboard) &&
                                            !square::attacked(board::square::D8 as u8, board::WHITE, cboard) {
                                            addMove(Move::new(piece, board::square::C8 as u8, 0, 0,
                                                false, false, true, 0), list);
                                        }
                                    }
                                }
                            }
                        }
                    }
                },

                2 => { // queen
                    for y in 0..4 {
                        let mut mark = piece as i8;
                        loop {
                            mark += square::cross[y];
                            let target = cboard.layout[mark as usize];
                            if target == board::piece::Empty as u8 {
                                addMove(Move::new(piece, mark as u8, 0, 0, false, false, false, 0), list);
                            } else if target != board::VOID_SQUARE && (target < baseline as u8 + 1 || target > baseline as u8 + 6) {
                                addMove(Move::new(piece, mark as u8, target, 0, false, false, false, MVV[target as usize] + 500), list);
                                break;
                            } else {
                                break;
                            }
                        }
                    }
                    for y in 0..4 {
                        let mut mark = piece as i8;
                        loop {
                            mark += square::diagonal[y];
                            let target = cboard.layout[mark as usize];
                            if target == board::piece::Empty as u8 {
                                addMove(Move::new(piece, mark as u8, 0, 0, false, false, false, 0), list);
                            } else if target != board::VOID_SQUARE && (target < baseline as u8 + 1 || target > baseline as u8 + 6) {
                                addMove(Move::new(piece, mark as u8, target, 0, false, false, false, MVV[target as usize] + 500), list);
                                break;
                            } else {
                                break;
                            }
                        }
                    }
                },

                3 => { // rook
                    for y in 0..4 {
                        let mut mark = piece as i8;
                        loop {
                            mark += square::cross[y];
                            let target = cboard.layout[mark as usize];
                            if target == board::piece::Empty as u8 {
                                addMove(Move::new(piece, mark as u8, 0, 0, false, false, false, 0), list);
                            } else if target != board::VOID_SQUARE && (target < baseline as u8 + 1 || target > baseline as u8 + 6) {
                                addMove(Move::new(piece, mark as u8, target, 0, false, false, false, MVV[target as usize] + 400), list);
                                break;
                            } else {
                                break;
                            }
                        }
                    }
                },

                4 => { // bishop
                    for y in 0..4 {
                        let mut mark = piece as i8;
                        loop {
                            mark += square::diagonal[y];
                            let target = cboard.layout[mark as usize];
                            if target == board::piece::Empty as u8 {
                                addMove(Move::new(piece, mark as u8, 0, 0, false, false, false, 0), list);
                            } else if target != board::VOID_SQUARE && (target < baseline as u8 + 1 || target > baseline as u8 + 6) {
                                addMove(Move::new(piece, mark as u8, target, 0, false, false, false, MVV[target as usize] + 300), list);
                                break;
                            } else {
                                break;
                            }
                        }
                    }
                },

                5 => { // knight
                    for y in 0..8 {
                        let target = cboard.layout[(piece.wrapping_add(square::knight[y] as u8)) as usize];
                        if target != board::VOID_SQUARE && (target < baseline as u8 + 1 || target > baseline as u8 + 6) {
                            if target == board::piece::Empty as u8 {
                                addMove(Move::new(piece, piece.wrapping_add(square::knight[y] as u8), 0, 0,
                                false, false, false, 0), list);
                            } else {
                                addMove(Move::new(piece, piece.wrapping_add(square::knight[y] as u8), target, 0,
                                false, false, false, MVV[target as usize] + 200), list);
                            }
                        }
                    }
                },

                6 => { // pawn
                    let mut dir : i8 = 10;
                    if baseline != 0 { dir = -10 };

                    // quiet moves
                    if cboard.layout[piece.wrapping_add(dir as u8) as usize]
                            == board::piece::Empty as u8 {
                        if baseline == 0 {
                            if piece / 10 == 8 {
                                addMove(Move::new(piece, piece.wrapping_add(dir as u8), 0,
                                    board::piece::Q as u8, false, false, false, 0), list);
                                addMove(Move::new(piece, piece.wrapping_add(dir as u8), 0,
                                    board::piece::R as u8, false, false, false, 0), list);
                                addMove(Move::new(piece, piece.wrapping_add(dir as u8), 0,
                                    board::piece::B as u8, false, false, false, 0), list);
                                addMove(Move::new(piece, piece.wrapping_add(dir as u8), 0,
                                    board::piece::N as u8, false, false, false, 0), list);
                            } else {
                                if piece / 10 == 3 && cboard.layout[piece.wrapping_add((dir * 2) as u8)
                                                                as usize]== board::piece::Empty as u8 {
                                    addMove(Move::new(piece, piece.wrapping_add((dir * 2) as u8), 0, 0, false, true, false,0), list);
                                }
                                addMove(Move::new(piece, piece.wrapping_add(dir as u8), 0, 0, false, false, false,0), list);
                            }
                        } else {
                            if piece / 10 == 3 {
                                addMove(Move::new(piece, piece.wrapping_add(dir as u8), 0,
                                    board::piece::q as u8, false, false, false,0), list);
                                addMove(Move::new(piece, piece.wrapping_add(dir as u8), 0,
                                    board::piece::r as u8, false, false, false,0), list);
                                addMove(Move::new(piece, piece.wrapping_add(dir as u8), 0,
                                    board::piece::b as u8, false, false, false,0), list);
                                addMove(Move::new(piece, piece.wrapping_add(dir as u8), 0,
                                    board::piece::n as u8, false, false, false,0), list);
                            } else {
                                if piece / 10 == 8 && cboard.layout[piece.wrapping_add((dir * 2) as u8)
                                                                as usize]== board::piece::Empty as u8 {
                                    addMove(Move::new(piece, piece.wrapping_add((dir * 2) as u8), 0, 0, false, true, false,0), list);
                                }
                                addMove(Move::new(piece, piece.wrapping_add(dir as u8), 0, 0, false, false, false,0), list);
                            }
                        }
                    }

                    // captures
                    for offs in &around[0..2] {
                        let offset = offs + dir;
                        let target = cboard.layout[piece.wrapping_add(offset as u8) as usize];
                        // TODO this condition may be too computation-heavy
                        if target != board::VOID_SQUARE &&
                            ((target != board::piece::Empty as u8 &&
                            (target < baseline as u8 + 1 || target > baseline as u8 + 6)) ||
                            (piece.wrapping_add(offset as u8) == cboard.en_passant)) {
                            if (baseline == 0) && (piece / 10 == 8) {
                                addMove(Move::new(piece, piece.wrapping_add(offset as u8), target,
                                    board::piece::Q as u8, false, false, false, MVV[target as usize]), list);
                                addMove(Move::new(piece, piece.wrapping_add(offset as u8), target,
                                    board::piece::R as u8, false, false, false, MVV[target as usize]), list);
                                addMove(Move::new(piece, piece.wrapping_add(offset as u8), target,
                                    board::piece::B as u8, false, false, false, MVV[target as usize]), list);
                                addMove(Move::new(piece, piece.wrapping_add(offset as u8), target,
                                    board::piece::N as u8, false, false, false, MVV[target as usize]), list);
                            } else if (baseline != 0) && (piece / 10 == 3) {
                                addMove(Move::new(piece, piece.wrapping_add(offset as u8), target,
                                    board::piece::q as u8, false, false, false, MVV[target as usize]), list);
                                addMove(Move::new(piece, piece.wrapping_add(offset as u8), target,
                                    board::piece::r as u8, false, false, false, MVV[target as usize]), list);
                                addMove(Move::new(piece, piece.wrapping_add(offset as u8), target,
                                    board::piece::b as u8, false, false, false, MVV[target as usize]), list);
                                addMove(Move::new(piece, piece.wrapping_add(offset as u8), target,
                                    board::piece::n as u8, false, false, false, MVV[target as usize]), list);
                            } else if piece.wrapping_add(offset as u8) == cboard.en_passant{
                                addMove(Move::new(piece, piece.wrapping_add(offset as u8), target,
                                     0, true, false, false, 1000), list);
                            }
                                else {
                                addMove(Move::new(piece, piece.wrapping_add(offset as u8), target,
                                     0, false, false, false, MVV[target as usize]), list);
                            }
                        }
                    }
                },

                _ => unreachable!()
            }
        }
    }
}
