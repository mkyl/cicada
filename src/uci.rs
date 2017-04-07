use board;
use fen;

pub fn looping(cboard : &mut board::chessboard) {
    use std::io;

    let stdin = io::stdin();
    let mut input = &mut String::new();

    loop {
        input.clear();
        stdin.read_line(input).unwrap();


        if input == "" || input == "\n" {
            continue
        }
        else if input == "uci\n" {
            startup_info();
        }
        else if input == "isready\n" {
            println!("readyok");
        }
        else if input.split_whitespace().any(|x| x == "ucinewgame") {
            new_game(cboard);
        }
        else if input.split_whitespace().any(|x| x == "position") {
            parse_position(input, cboard);
        }
        else if input.split_whitespace().any(|x| x == "go") {
            parse_search(input, cboard);
        }
        else if input.split_whitespace().any(|x| x == "quit") {
            break
        }
    }
}

fn startup_info() {
    println!("id name Cicada");
    println!("id author Kayali");
    println!("uciok");
}

fn new_game(cboard : &mut board::chessboard) {
    fen::parse("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", cboard);
    // TODO clear Transposition table!
}

fn parse_position(input : &str, cboard : &mut board::chessboard) {
    let v: Vec<&str> = input.split_whitespace().collect();

    if v[1] == "startpos" {
        fen::parse("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", cboard);
        parse_moves(&v, 2, cboard);
    } else if v[1] == "fen" {
        let fen_code = v[2..8].join(" ");
        fen::parse(&fen_code, cboard);
        parse_moves(&v, 8, cboard);
    }

    board::print(cboard);
}

fn parse_moves(input : &[&str], input_index : usize, cboard : &mut board::chessboard) {
    use moves;
    use movement;

    for move_str in &input[input_index + 1..] {
        let move_bytes = move_str.as_bytes();
        let move_ = moves::from_AN(move_bytes, cboard);
        let success = movement::make(&move_, cboard);
        assert!(success);
    }
}

fn parse_search(input : &str, cboard : &mut board::chessboard) {
    use think;

    let v: Vec<&str> = input.split_whitespace().collect();
    let mut time = 0;
    let mut depth = 0;

    let time_mark = if cboard.side == board::WHITE { "wtime" } else { "btime" };
    if let Some(i) = v.iter().position(|s| *s == time_mark) {
        time = v[i + 1].parse::<i64>().unwrap();
    }

    if let Some(i) = v.iter().position(|s| *s == "depth") {
        depth = v[i + 1].parse::<u8>().unwrap();
    }

    println!("time: {}, depth: {}", time, depth);
    if time != 0 || depth != 0 {
        think::start(cboard, depth, time);
    }
}
