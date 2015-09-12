use board;
use zobrist;

pub fn parse(fen : &str, cboard: &mut board::chessboard) {
    use board::castling_bits;
    use board::piece;
    use board::rank;
    use board::file;

    board::reset(cboard);

    let mut rank : u8 = rank::rank_8 as u8;
    let mut file : u8 = file::file_a as u8;

    let fen = fen.as_bytes();
    let mut counter = 0;

    // board parsing, starting with a8
    while fen[counter] as char != ' ' {
        let mut empty = 0;
        match fen[counter] as char {
            'P' => {cboard.layout[board::AN_to_board(file, rank) as usize] = piece::P as u8; file += 1},
            'N' => {cboard.layout[board::AN_to_board(file, rank) as usize] = piece::N as u8; file += 1},
            'B' => {cboard.layout[board::AN_to_board(file, rank) as usize] = piece::B as u8; file += 1},
            'R' => {cboard.layout[board::AN_to_board(file, rank) as usize] = piece::R as u8; file += 1},
            'Q' => {cboard.layout[board::AN_to_board(file, rank) as usize] = piece::Q as u8; file += 1},
            'K' => {cboard.layout[board::AN_to_board(file, rank) as usize] = piece::K as u8; file += 1},
            'p' => {cboard.layout[board::AN_to_board(file, rank) as usize] = piece::p as u8; file += 1},
            'n' => {cboard.layout[board::AN_to_board(file, rank) as usize] = piece::n as u8; file += 1},
            'b' => {cboard.layout[board::AN_to_board(file, rank) as usize] = piece::b as u8; file += 1},
            'r' => {cboard.layout[board::AN_to_board(file, rank) as usize] = piece::r as u8; file += 1},
            'q' => {cboard.layout[board::AN_to_board(file, rank) as usize] = piece::q as u8; file += 1},
            'k' => {cboard.layout[board::AN_to_board(file, rank) as usize] = piece::k as u8; file += 1},

            '1' ... '8' => empty = fen[counter],

            '/' => { rank -= 1; file = file::file_a as u8 },

             _  => {println!("{}", fen[counter]) ; panic!("[!] Critical: Invalid FEN layout code")},
        }


        if empty != 0 {
            let start = board::AN_to_board(file, rank) as usize;
            empty = empty - ('0' as u8);

            for i in 0..empty {
                cboard.layout[start + i as usize] = piece::Empty as u8;
            }

            file += empty;
        }

        counter += 1; 
    }
    counter += 1;

    // side parsing
    if fen[counter] as char == 'w' {
        cboard.side = board::white;
    } else {
        cboard.side = board::black;
    }
    counter += 2;

    // castling parsing
    cboard.castling = 0;
    while fen[counter] as char != ' ' {
        match fen[counter] as char {
            'K' => cboard.castling |= castling_bits::K_cp as u8,
            'Q' => cboard.castling |= castling_bits::Q_cp as u8,
            'k' => cboard.castling |= castling_bits::k_cp as u8,
            'q' => cboard.castling |= castling_bits::q_cp as u8,
            '-' => cboard.castling = 0,
             _ => panic!("[!] Critical: Invalid FEN castling code"),
        }
        counter += 1;
    }
    counter += 1;

    // en passant parsing
    if fen[counter] as char == '-' {
        cboard.en_passant = board::void_square;
        counter += 1;
    } else {
        cboard.en_passant =
            board::AN_to_chocolate(fen[counter] as char, fen[counter + 1]);
        counter += 2;
    }
    counter += 1;

    // fifty move parsing
    if fen[counter + 1] as char == ' ' {
        cboard.fifty = fen[counter] - '0' as u8;
        counter +=1;
    } else if fen[counter + 2] as char == ' ' {
        cboard.fifty = (fen[counter] - '0' as u8) * 10 + fen[counter + 1] - '0' as u8;
        counter +=2;
    } else {
        panic!("[!] Critical: Invalid FEN fifty move code");
    }
    counter += 1;

    // game depth parsing
    if counter + 3 == fen.len() {
        cboard.depth = (fen[counter] as u16 - '0' as u16) * 100 + (fen[counter + 1] as u16 - '0' as u16) 
            * 10 + fen[counter + 2] as u16 - '0' as u16;
    } else if counter + 2 == fen.len() {
        cboard.depth = (fen[counter] as u16 - '0' as u16) * 10 + fen[counter + 1] as u16 - '0' as u16;
    } else if counter + 1 == fen.len() {
        cboard.depth = fen[counter] as u16 - '0' as u16;
    }

    // update piece list
    board::update_pieces(cboard);

    // hash update
    cboard.zobrist = zobrist::hash(cboard); 
}
