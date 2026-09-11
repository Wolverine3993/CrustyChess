use crate::chess::{Board, ChessPiece, PieceType::*, Colour::*};

pub fn parse_fen(fen: String) -> Result<Board, String> {
    let mut board = new_board();

    let mut row_num = 0;
    let rows = fen.split("/");
    for fen_row in rows {
        if row_num >= 8 {
            return Err("Invalid FEN - too many rows".to_string());
        }

        let mut row: [ChessPiece; 8] = [ChessPiece {colour: Colourless, piece_type: Space}; 8];
        let mut pointer = 0;
        for piece_char in fen_row.chars() {
            if pointer >= 8 {
                return Err("Invalid FEN - too many pieces in a row".to_string());
            }

            match get_piece(&piece_char)? {
                PieceResult::ChessPiece(piece) => row[pointer] = piece,
                PieceResult::Space(num) => pointer += (num - 1) as usize
            };

            pointer += 1;
        }

        board[row_num] = row;
        row_num += 1;
    }

    Ok(board)
}

enum PieceResult {
    ChessPiece(ChessPiece),
    Space(u32)
}

fn new_board() -> Board {
    [[ChessPiece { piece_type: Space, colour: Colourless }; 8]; 8]
}

fn get_piece(character: &char) -> Result<PieceResult, String> {
    let lower_char = character.to_ascii_lowercase();

    let colour = if lower_char == *character {
        Black
    } else {
        White
    };

    let piece_type = match lower_char {
        'k' => King,
        'q' => Queen,
        'r' => Rook,
        'b' => Bishop,
        'n' => Knight,
        'p' => Pawn,
        _ => match character.to_digit(9) {
            Some(num) => return Ok(PieceResult::Space(num)),
            None => return Err("Invalid FEN".to_string())
        }
    };

    Ok(PieceResult::ChessPiece(ChessPiece {
        colour,
        piece_type
    }))
}


#[test]
fn test_get_piece() {
    let piece = match get_piece(&'k').unwrap() {
        PieceResult::ChessPiece(piece) => piece,
        PieceResult::Space(_) => panic!()
    };
    assert_eq!(piece.piece_type, King);
    assert_eq!(piece.colour, Black);

    let piece = match get_piece(&'k').unwrap() {
        PieceResult::ChessPiece(piece) => piece,
        PieceResult::Space(_) => panic!()
    };
    assert_eq!(piece.piece_type, King);
    assert_eq!(piece.colour, White);
}