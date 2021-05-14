mod utils;

use wasm_bindgen::prelude::*;
use array2d::Array2D;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Clone)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}

#[derive(Clone)]
pub enum Color {
    White,
    Black
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Piece {
    kind: PieceType,
    color: Color
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Square {
    piece: Option<Piece>
}

#[wasm_bindgen]
pub struct Board {
    squares: Array2D<Square>,
    fen: String
}

impl Board {
    fn get_fen(&self) -> &String {
        &self.fen
    }

    pub fn new(fen_in: &String) -> Board {
        let squares = Array2D::filled_with(Square {piece: None}, 8, 8);
        let mut fen = fen_in.clone();

        if fen.is_empty() {
            fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string();
        }
        
        let ret = Board {
            squares,
            fen: "".to_string()
        };

        ret.set_fen(fen);

        return ret;
    }

    pub fn set_fen(&self, fen_in: String) {

    }
}