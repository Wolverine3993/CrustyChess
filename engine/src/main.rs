mod chess;

fn main() {
    let board = chess::fen::parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_string()).unwrap();

    chess::print_board(&board);
}
