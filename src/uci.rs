use board;
use fen;

pub fn looping(cboard : &mut board::chessboard) {
    use std::io::{self,BufRead};

    let mut stdin = io::stdin();
    let mut input = &mut String::new();

    loop {
        input.clear();
        stdin.read_line(input);


        if input == "" || input == "\n" {
            continue
        }
        else if input == "uci\n" {
            startup_info();
        }
        else if input == "isready\n" {
            print!("readyok\n");
        }
        else if input.split_whitespace().any(|x| x == "ucinewgame") {
            new_game(cboard);
        }
        else if input.split_whitespace().any(|x| x == "position") {
            parse_position(&input, cboard);
        }
        else if input.split_whitespace().any(|x| x == "go") {
            parse_search(&input, cboard);
        }
        else if input.split_whitespace().any(|x| x == "quit") {
            break
        }
    }
}

fn startup_info() {
    print!("id name Cicada\n");
    print!("id author Kayali\n");
    print!("uciok\n");
}

fn new_game(cboard : &mut board::chessboard) {
    fen::parse(&"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", cboard);
    // TODO clear transposition table!
}

fn parse_position(input : &str, cboard : &mut board::chessboard) {
    let v: Vec<&str> = input.split_whitespace().collect();

    if v[1] == "startpos" {
        fen::parse(&"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", cboard);
        parse_moves(&v, 2, cboard);
    } else if v[1] == "fen" {
        let mut fen_code = v[2].to_string();

        for x in 3..8 {
            fen_code = fen_code + " "+ &v[x];
        }

        fen::parse(&fen_code, cboard);
        parse_moves(&v, 8, cboard);
    }

    board::print(cboard);
}

fn parse_moves(input : &Vec<&str>, input_index : usize, cboard : &mut board::chessboard) {
    use moves;
    use movement;

    for index in input_index + 1..input.len() {
        let move_str = input[index].as_bytes();
        let mut move_ = moves::from_AN(move_str, cboard);

        assert!(movement::make(&move_, cboard));
    }
}

fn parse_search(input : &str, cboard : &mut board::chessboard) {
    use think;

    let v: Vec<&str> = input.split_whitespace().collect();
    let mut time = 0;
    let mut depth = 0;

    if cboard.side == board::white {
        for x in 0..v.len() {
            if v[x] == "wtime" {
                time = v[x + 1].parse::<i64>().unwrap();
            }
        }
    } else {
        for x in 0..v.len() {
            if v[x] == "btime" {
                time = v[x + 1].parse::<i64>().unwrap();
            }
        }
    }

    for x in 0..v.len() {
        if v[x] == "depth" {
            depth = v[x + 1].parse::<u8>().unwrap();
        }
    }

    println!("time: {}, depth: {}", time, depth);
    if time != 0 || depth != 0 {
        think::start(cboard, depth, time); 
    }
}
