use board;

pub fn repetition (cboard : &board::chessboard) -> bool{
    for x in (cboard.depth - cboard.fifty as u16) as usize..cboard.depth as usize - 1 {
        if cboard.zobrist == cboard.past[x].zobrist {
            return true
        }
    }
    false
}
