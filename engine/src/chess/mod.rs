pub mod fen;
pub mod square_values;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Colour {
    White,
    Black,
    Colourless
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum PieceType {
    King,
    Queen,
    Knight,
    Bishop,
    Pawn,
    Rook,
    Space
}

#[derive(Debug, Copy, Clone)]
pub struct ChessPiece {
    piece_type: PieceType,
    colour: Colour
}

pub type Board = [[ChessPiece; 8]; 8];

pub fn print_board(board: &Board) {
    let mut print_str = String::new();
    for row in board {
        for piece in row {
            let piece_char_string: String;
            let piece_char = match piece.piece_type {
                PieceType::King => ("♔", "♚"),
                PieceType::Bishop => ("♗", "♝"),
                PieceType::Knight => ("♘", "♞"),
                PieceType::Pawn => ("♙", "♟"),
                PieceType::Queen => ("♕", "♛"),
                PieceType::Rook => ("♖", "♜"),
                PieceType::Space => ("o", "o")
            };

            if piece.colour == Colour::White {
                piece_char_string = piece_char.0.to_string();
            }
            else {
                piece_char_string = piece_char.1.to_string();
            }
            print_str += &piece_char_string;
            print_str += " ";
        }

        print_str += "\n";
    }

    println!("{:}", print_str);
}

pub fn next_move(fen: &str, colour: Colour) -> Result<String, String> {
    let board = fen::parse_fen(fen.to_string())?;
    Ok("e4".to_string())
}