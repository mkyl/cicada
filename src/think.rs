use board;

const hash_map_size : usize = 0x100000;

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
    println!("storing hash {} in index: {}\n", cboard.zobrist, i);
    cboard.transposition_table.entries[i].hash = cboard.zobrist;
    cboard.transposition_table.entries[i].move_ = move_;
}

pub fn find_transposition(cboard: &mut board::chessboard) -> u32 {
    let x = cboard.zobrist % hash_map_size as u64;
    let i = x as usize;

    println!("retreiving hash {} from index: {}\n", cboard.zobrist, i);
    // TODO collisions here should be astronomically rare
    // but is this really the case?
    if cboard.zobrist == cboard.transposition_table.entries[i].hash {
        return cboard.transposition_table.entries[i].move_
    } else {
        //println!("expected: {} \n found: {}", cboard.zobrist, cboard.transposition_table.entries[i].hash);
        return 1
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
