use crate::chess::{Board, Colour};

mod square_values;
mod piece_values;

pub fn get_board_eval(board: &Board, current_move: Colour) -> i32 {
    let mut eval = 0;
    for y in 0..8 {
        for x in 0..8 {
            let current_piece = board[y][x];
            let score = piece_values::get_piece_eval(&current_piece) 
                           + square_values::get_square(&current_piece, square_values::GamePhase::MIDGAME)[y][x];

            if current_piece.colour == current_move {
                eval += score;
            } else {
                eval -= score;
            }
        }
    }

    eval
}

#[test]
fn test_eval() {
    use crate::chess::fen;
    let board = fen::parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_string()).unwrap();

    assert_eq!(get_board_eval(&board, Colour::White), 0);
}