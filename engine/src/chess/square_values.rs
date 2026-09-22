use crate::chess::{ChessPiece, Colour, PieceType};

// I'm so excited to finally be able to have a file of random magic numbers
// I feel like a real programmer

pub const PAWN: [[i32; 8]; 8] = [
    [0,  0,  0,  0,  0,  0,  0,  0 ],
    [50, 50, 50, 50, 50, 50, 50, 50],
    [10, 10, 20, 30, 30, 20, 10, 10],
    [5,  5,  10, 25, 25, 10, 5,  5 ],
    [0,  0,  0,  20, 20, 0,  0,  0 ],
    [5, -5, -10, 0,  0, -10,-5,  5 ],
    [5, 10,  10,-20,-20, 10, 10, 5 ],
    [0, 0,   0,  0,  0,  0,  0,  0 ]];

pub const KNIGHT: [[i32; 8]; 8] = [
    [-50,-40,-30,-30,-30,-30,-40,-50],
    [-40,-20,  0,  0,  0,  0,-20,-40],
    [-30,  0, 10, 15, 15, 10,  0,-30],
    [-30,  5, 15, 20, 20, 15,  5,-30],
    [-30,  0, 15, 20, 20, 15,  0,-30],
    [-30,  5, 10, 15, 15, 10,  5,-30],
    [-40,-20,  0,  5,  5,  0,-20,-40],
    [-50,-40,-30,-30,-30,-30,-40,-50]];

pub const BISHOP: [[i32; 8]; 8] = [
    [-20,-10,-10,-10,-10,-10,-10,-20],
    [-10,  0,  0,  0,  0,  0,  0,-10],
    [-10,  0,  5, 10, 10,  5,  0,-10],
    [-10,  5,  5, 10, 10,  5,  5,-10],
    [-10,  0, 10, 10, 10, 10,  0,-10],
    [-10, 10, 10, 10, 10, 10, 10,-10],
    [-10,  5,  0,  0,  0,  0,  5,-10],
    [-20,-10,-10,-10,-10,-10,-10,-20]];

pub const ROOK: [[i32; 8]; 8] = [
    [-20,-10,-10,-10,-10,-10,-10,-20],
    [-10,  0,  0,  0,  0,  0,  0,-10],
    [-10,  0,  5, 10, 10,  5,  0,-10],
    [-10,  5,  5, 10, 10,  5,  5,-10],
    [-10,  0, 10, 10, 10, 10,  0,-10],
    [-10, 10, 10, 10, 10, 10, 10,-10],
    [-10,  5,  0,  0,  0,  0,  5,-10],
    [-20,-10,-10,-10,-10,-10,-10,-20]];

pub const QUEEN: [[i32; 8]; 8] = [
    [-20,-10,-10, -5, -5,-10,-10,-20],
    [-10,  0,  0,  0,  0,  0,  0,-10],
    [-10,  0,  5,  5,  5,  5,  0,-10],
    [ -5,  0,  5,  5,  5,  5,  0, -5],
    [  0,  0,  5,  5,  5,  5,  0, -5],
    [-10,  5,  5,  5,  5,  5,  0,-10],
    [-10,  0,  5,  0,  0,  0,  0,-10],
    [-20,-10,-10, -5, -5,-10,-10,-20]];

pub const KING_MIDGAME: [[i32; 8]; 8] = [
    [-30,-40,-40,-50,-50,-40,-40,-30],
    [-30,-40,-40,-50,-50,-40,-40,-30],
    [-30,-40,-40,-50,-50,-40,-40,-30],
    [-30,-40,-40,-50,-50,-40,-40,-30],
    [-20,-30,-30,-40,-40,-30,-30,-20],
    [-10,-20,-20,-20,-20,-20,-20,-10],
    [20, 20,  0,  0,  0,  0,  20, 20],
    [20, 30, 10,  0,  0, 10,  30, 20]];

pub const KING_ENDGAME: [[i32; 8]; 8] = [
    [-50,-40,-30,-20,-20,-30,-40,-50],
    [-30,-20,-10,  0,  0,-10,-20,-30],
    [-30,-10, 20, 30, 30, 20,-10,-30],
    [-30,-10, 30, 40, 40, 30,-10,-30],
    [-30,-10, 30, 40, 40, 30,-10,-30],
    [-30,-10, 20, 30, 30, 20,-10,-30],
    [-30,-30,  0,  0,  0,  0,-30,-30],
    [-50,-30,-30,-30,-30,-30,-30,-50]];


#[derive(PartialEq)]
pub enum GamePhase {
    MIDGAME,
    ENDGAME
}

pub fn get_square(piece: ChessPiece, game_phase: GamePhase) -> [[i32; 8]; 8] {
    match piece.piece_type {
        PieceType::Pawn => get_coloured_square(PAWN, piece.colour),
        PieceType::Bishop => get_coloured_square(BISHOP, piece.colour),
        PieceType::Knight => get_coloured_square(KNIGHT, piece.colour),
        PieceType::Rook => get_coloured_square(ROOK, piece.colour),
        PieceType::Queen => get_coloured_square(QUEEN, piece.colour),
        PieceType::King => {
            if game_phase == GamePhase::MIDGAME {
                get_coloured_square(KING_MIDGAME, piece.colour)
            } else {
                get_coloured_square(KING_ENDGAME, piece.colour)
            }
        },
        PieceType::Space => [[0; 8]; 8]
    }
}

fn get_coloured_square(square: [[i32; 8]; 8], colour: Colour) -> [[i32; 8]; 8] {
    match colour {
        Colour::Black => flip(square),
        _ => square
    }
}

fn flip(square: [[i32; 8]; 8]) -> [[i32; 8]; 8] {
    let mut flipped_square = [[0; 8]; 8];

    for y in 0..8 {
        for x in 0..8 {
            flipped_square[y][x] = square[7 - y][7 - x];
        }
    }

    flipped_square
}