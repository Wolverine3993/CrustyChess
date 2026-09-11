pub mod fen;

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
            let mut piece_char = match piece.piece_type {
                PieceType::King => "k",
                PieceType::Bishop => "b",
                PieceType::Knight => "n",
                PieceType::Pawn => "p",
                PieceType::Queen => "q",
                PieceType::Rook => "r",
                PieceType::Space => "o"
            }.to_string();

            if piece.colour == Colour::White {
                piece_char = piece_char.to_uppercase();
            }
            print_str += &piece_char;
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