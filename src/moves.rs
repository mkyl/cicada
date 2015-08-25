use board;

pub struct _move {
    /* 0000 0000 0000 0000 0000 0000 0000
     * 000C DEpp ppcc ccff ffff fttt tttt */
    container : u32,
    score: u8 
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
