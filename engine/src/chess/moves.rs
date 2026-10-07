use crate::chess::{Board, Colour};

// limited range pieces

const KING: [(i32, i32); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    (0,  -1),          (0,  1),
    (1,  -1), ( 1, 0), (1 , 1)
];

// note: the knight can jump over pieces,
//       this should be easily implementable
//       for all these pieces we just need to ensure
//       target square is empty

const KNIGHT: [(i32, i32); 8] = [
        (-2, -1), (-2, 1),
    (-1, -2),        (-1, 2),

    ( 1, -2),        ( 1, 2),
        ( 2, -1), ( 2, 1)
];

const PAWN: [(i32, i32); 4] = [
              (-2, 0),
    (-1, -1), (-1, 0), (-1, 1)
];

// unlimited range pieces
const BISHOP: [(i32, i32); 4] = [
    (-1, -1), (-1,  1),

    (1,  -1), ( 1, -1)
];

const ROOK: [(i32, i32); 4] = [
            (-1, 0),
        (0, -1), (0, 1),
            ( 1, 0)
];

const QUEEN: [(i32, i32); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    (0,  -1),          (0,  1),
    (1,  -1), ( 1, 0), (1 , 1)
];

pub struct Move {
    pub from: (i32, i32),
    pub to: (i32, i32),
}

pub fn get_all_moves(board: &Board, current_move: Colour) -> Vec<Move> {
    let moves = vec![];
    
    for (y, row) in board.iter().enumerate() {
        for (x, piece) in row.iter().enumerate() {
            if piece.colour != current_move {
                continue;
            }
        }
    };

    moves
}

fn get_moveable_squares(piece: &ChessPiece) -> MoveList {
    match piece.
}

enum MoveList {
    Finite(Vec<i32>),
    Infinite(Vec<i32>)
}