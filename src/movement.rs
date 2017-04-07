use board;
use moves;
use square;
use sanity;
use zobrist;

pub fn make(m : &moves::Move, cboard : &mut board::chessboard) -> bool {
    let origin = moves::from(m);
    let target = moves::to(m);

    let Snapshot = board::Snapshot{
        move_key: m.container,
        castling: cboard.castling,
        en_passant: cboard.en_passant,
        fifty: cboard.fifty,
        zobrist: cboard.zobrist
    };

    cboard.past[cboard.depth as usize] = Snapshot;

    cboard.fifty += 1;

    // unhash all the things
    zobrist::castle(cboard);
    zobrist::en_passant(cboard);
    zobrist::sides(cboard);

    cboard.en_passant = board::VOID_SQUARE;

    if moves::en_passant(m) {
        if cboard.side == board::WHITE {
            square::clear(target - 10, cboard);
        } else {
            square::clear(target + 10, cboard);
        }
    } else if moves::castling(m) {
        match target {
            27  => square::plsmove(28, 26, cboard), // WHITE kingside
            23  => square::plsmove(21, 24, cboard), // WHITE queenside
            97  => square::plsmove(98, 96, cboard), // BLACK kingside
            93  => square::plsmove(91, 94, cboard), // BLACK queenside
            _ => {panic!("invalid castling");}
        }
    }

    if cboard.layout[origin as usize] == board::piece::P as u8 ||
                    cboard.layout[origin as usize] == board::piece::p as u8{
        cboard.fifty = 0;
        if moves::pawn_double(m) {
            if cboard.side == board::WHITE {
                cboard.en_passant = origin + 10;
            } else {
                cboard.en_passant = origin - 10;
            }
        }
    }

    if moves::capture(m) != 0 {
        cboard.fifty = 0;
        square::clear(target, cboard);
    }

    square::plsmove(origin, target, cboard);

    if moves::promoted(m) != 0 {
        square::clear(target, cboard);
        square::add(target, moves::promoted(m), cboard);
    }

    match origin {
        25 => {cboard.castling &= !(board::CastlingBits::K_cp as u8);
               cboard.castling &= !(board::CastlingBits::Q_cp as u8)},
        28 =>  cboard.castling &= !(board::CastlingBits::K_cp as u8),
        21 =>  cboard.castling &= !(board::CastlingBits::Q_cp as u8),
        95 => {cboard.castling &= !(board::CastlingBits::k_cp as u8);
               cboard.castling &= !(board::CastlingBits::q_cp as u8)},
        98 =>  cboard.castling &= !(board::CastlingBits::k_cp as u8),
        91 =>  cboard.castling &= !(board::CastlingBits::q_cp as u8),
        _ => {;}
    }

    match target {
        25 => {cboard.castling &= !(board::CastlingBits::K_cp as u8);
               cboard.castling &= !(board::CastlingBits::Q_cp as u8)},
        28 =>  cboard.castling &= !(board::CastlingBits::K_cp as u8),
        21 =>  cboard.castling &= !(board::CastlingBits::Q_cp as u8),
        95 => {cboard.castling &= !(board::CastlingBits::k_cp as u8);
               cboard.castling &= !(board::CastlingBits::q_cp as u8)},
        98 =>  cboard.castling &= !(board::CastlingBits::k_cp as u8),
        91 =>  cboard.castling &= !(board::CastlingBits::q_cp as u8),
        _ => {;}
    }

    cboard.side = !cboard.side;
    cboard.depth += 1;
    cboard.ply += 1;

    // hash back the things
    zobrist::castle(cboard);
    zobrist::en_passant(cboard);
    zobrist::sides(cboard);

    debug_assert!(sanity::sane(cboard));

    if square::attacked(cboard.piece_list[if cboard.side == board::BLACK {
            board::piece::K } else { board::piece::k } as usize][0],
            cboard.side, cboard) {
        undo(cboard);
        return false;
    }
    true
}

pub fn undo(cboard : &mut board::chessboard) {
    zobrist::sides(cboard);
    zobrist::en_passant(cboard);
    zobrist::castle(cboard);

    cboard.depth -= 1;
    cboard.ply -= 1;
    cboard.side = !cboard.side;
    let move_key = cboard.past[cboard.depth as usize].move_key;
    cboard.castling = cboard.past[cboard.depth as usize].castling;
    cboard.en_passant = cboard.past[cboard.depth as usize].en_passant;
    cboard.fifty = cboard.past[cboard.depth as usize].fifty;

    let move_ = moves::Move{container:move_key, score: 0};
    let target = moves::to(&move_);
    let origin = moves::from(&move_);

    zobrist::sides(cboard);
    zobrist::en_passant(cboard);
    zobrist::castle(cboard);

    if moves::en_passant(&move_) {
        if cboard.side == board::WHITE {
            square::add(target - 10, board::piece::p as u8, cboard);
        } else {
            square::add(target + 10, board::piece::P as u8, cboard);
        }
    } else if moves::castling(&move_) {
        match target {
            27  => square::plsmove(26, 28, cboard), // WHITE kingside
            23  => square::plsmove(24, 21, cboard), // WHITE queenside
            97  => square::plsmove(96, 98, cboard), // BLACK kingside
            93  => square::plsmove(94, 91, cboard), // BLACK queenside
            _ => unreachable!()
        }
    }

    square::plsmove(target, origin, cboard);

    if moves::capture(&move_) != 0 {
        square::add(target, moves::capture(&move_), cboard);
    }

    if moves::promoted(&move_) != 0 {
        square::clear(origin, cboard);
        if cboard.side == board::WHITE {
            square::add(origin, board::piece::P as u8, cboard);
        } else {
            square::add(origin, board::piece::p as u8, cboard);
        }
    }

    debug_assert!(sanity::sane(cboard));
}
