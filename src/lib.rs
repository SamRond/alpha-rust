mod utils;

use core::str;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Clone)]
pub enum PieceType {
    Pawn {val: i8},
    Knight {val: i8},
    Bishop {val: i8},
    Rook {val: i8},
    Queen {val: i8},
    King {val: i8}
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
    color: Color,
    rank: u32,
    file: u32
}

#[wasm_bindgen]
pub struct Board {
    fen: String,

    // White pieces
    white_pieces: Vec<Piece>,

    // Black pieces
    black_pieces: Vec<Piece>
}

impl Board {

    fn get_fen(&self) -> &str {
        &self.fen
    }

    pub fn set_fen(&mut self, fen_in: String) {
        self.fen = fen_in
    }

    pub fn new(fen_in: &String) -> Board {
        let mut fen = fen_in.clone();

        if fen.is_empty() {
            fen = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        }

        // generate pieces
        let mut ret = Board {
            fen: String::new(),
            white_pieces: Board::init_white_pieces(),
            black_pieces: Board::init_black_pieces()
        };

        ret.set_fen(fen);
        ret.set_piece_coords();

        return ret;
    }

    // Fills array of white pieces
    //
    // 0 - 7: pawns
    // 8 - 9: rooks
    // 10 - 11: knights
    // 12 - 13: bishops
    // 14: queen
    // 15: king
    fn init_white_pieces() -> Vec<Piece> {
        let mut white_pieces = Vec::with_capacity(16); 

        white_pieces[0] = Piece {
            kind: PieceType::Pawn { val: 1 },
            color: Color::White,
            rank: 0,
            file: 0
        };
        white_pieces[1] = Piece {
            kind: PieceType::Pawn { val: 1 },
            color: Color::White,
            rank: 0,
            file: 0
        };
        white_pieces[2] = Piece {
            kind: PieceType::Pawn { val: 1 },
            color: Color::White,
            rank: 0,
            file: 0
        };
        white_pieces[3] = Piece {
            kind: PieceType::Pawn { val: 1 },
            color: Color::White,
            rank: 0,
            file: 0
        };
        white_pieces[4] = Piece {
            kind: PieceType::Pawn { val: 1 },
            color: Color::White,
            rank: 0,
            file: 0
        };
        white_pieces[5] = Piece {
            kind: PieceType::Pawn { val: 1 },
            color: Color::White,
            rank: 0,
            file: 0
        };
        white_pieces[6] = Piece {
            kind: PieceType::Pawn { val: 1 },
            color: Color::White,
            rank: 0,
            file: 0
        };
        white_pieces[7] = Piece {
            kind: PieceType::Pawn { val: 1 },
            color: Color::White,
            rank: 0,
            file: 0
        };
        white_pieces[8] = Piece {
            kind: PieceType::Rook { val: 5 },
            color: Color::White,
            rank: 0,
            file: 0
        };
        white_pieces[9] = Piece {
            kind: PieceType::Rook { val: 5 },
            color: Color::White,
            rank: 0,
            file: 0
        };
        white_pieces[10] = Piece {
            kind: PieceType::Knight { val: 3 },
            color: Color::White,
            rank: 0,
            file: 0
        };
        white_pieces[11] = Piece {
            kind: PieceType::Knight { val: 3 },
            color: Color::White,
            rank: 0,
            file: 0
        };
        white_pieces[12] = Piece {
            kind: PieceType::Bishop { val: 3 },
            color: Color::White,
            rank: 0,
            file: 0
        };
        white_pieces[13] = Piece {
            kind: PieceType::Bishop { val: 3 },
            color: Color::White,
            rank: 0,
            file: 0
        };
        white_pieces[14] = Piece {
            kind: PieceType::Queen { val: 9 },
            color: Color::White,
            rank: 0,
            file: 0
        };
        white_pieces[15] = Piece {
            kind: PieceType::King { val: 127 },
            color: Color::White,
            rank: 0,
            file: 0
        };

        white_pieces
    }

    // Fills array of black pieces
    //
    // 0 - 7: pawns
    // 8 - 9: rooks
    // 10 - 11: knights
    // 12 - 13: bishops
    // 14: queen
    // 15: king
    fn init_black_pieces() -> Vec<Piece> {
        let mut black_pieces = Vec::with_capacity(16); 

        black_pieces[0] = Piece {
            kind: PieceType::Pawn { val: 1 },
            color: Color::Black,
            rank: 0,
            file: 0
        };
        black_pieces[1] = Piece {
            kind: PieceType::Pawn { val: 1 },
            color: Color::Black,
            rank: 0,
            file: 0
        };
        black_pieces[2] = Piece {
            kind: PieceType::Pawn { val: 1 },
            color: Color::Black,
            rank: 0,
            file: 0
        };
        black_pieces[3] = Piece {
            kind: PieceType::Pawn { val: 1 },
            color: Color::Black,
            rank: 0,
            file: 0
        };
        black_pieces[4] = Piece {
            kind: PieceType::Pawn { val: 1 },
            color: Color::Black,
            rank: 0,
            file: 0
        };
        black_pieces[5] = Piece {
            kind: PieceType::Pawn { val: 1 },
            color: Color::Black,
            rank: 0,
            file: 0
        };
        black_pieces[6] = Piece {
            kind: PieceType::Pawn { val: 1 },
            color: Color::Black,
            rank: 0,
            file: 0
        };
        black_pieces[7] = Piece {
            kind: PieceType::Pawn { val: 1 },
            color: Color::Black,
            rank: 0,
            file: 0
        };
        black_pieces[8] = Piece {
            kind: PieceType::Rook { val: 5 },
            color: Color::Black,
            rank: 0,
            file: 0
        };
        black_pieces[9] = Piece {
            kind: PieceType::Rook { val: 5 },
            color: Color::Black,
            rank: 0,
            file: 0
        };
        black_pieces[10] = Piece {
            kind: PieceType::Knight { val: 3 },
            color: Color::Black,
            rank: 0,
            file: 0
        };
        black_pieces[11] = Piece {
            kind: PieceType::Knight { val: 3 },
            color: Color::Black,
            rank: 0,
            file: 0
        };
        black_pieces[12] = Piece {
            kind: PieceType::Bishop { val: 3 },
            color: Color::Black,
            rank: 0,
            file: 0
        };
        black_pieces[13] = Piece {
            kind: PieceType::Bishop { val: 3 },
            color: Color::Black,
            rank: 0,
            file: 0
        };
        black_pieces[14] = Piece {
            kind: PieceType::Queen { val: 9 },
            color: Color::Black,
            rank: 0,
            file: 0
        };
        black_pieces[15] = Piece {
            kind: PieceType::King { val: 127 },
            color: Color::Black,
            rank: 0,
            file: 0
        };

        black_pieces
    }

    // Iterates through the FEN and properly sets the coordinates of each piece
    fn set_piece_coords(&mut self) {
        let mut rank = 8;
        let mut file = 0;
        
        let mut white = true;

        // keeps track of the number of each piece that has been seen so far
        // [pawns, rooks, knights, bishops, queen]
        let mut white_piece_counts = [0, 0, 0, 0, 0];
        let mut black_piece_counts = [0, 0, 0, 0, 0];

        for ch in self.fen.chars() {
            white = ch.is_ascii_uppercase();
            
            let pieces;
            let counts;
            
            if white {
                pieces = &mut self.white_pieces;
                counts = &mut white_piece_counts;
            } else {
                pieces = &mut self.black_pieces;
                counts = &mut black_piece_counts;
            };

            if ch.is_ascii_digit() {
                // if char is a number, it represents empty spaces
                // thus, any number should increment the 'file' counter
                //
                // convert the char to an integer and add it to 'file'
                file +=  match ch.to_digit(10) {
                    None => 0,
                    Some(x) => x, // x is now an integer value
                }
            } else {
                // 0 - 7: pawns
                // 8 - 9: rooks
                // 10 - 11: knights
                // 12 - 13: bishops
                // 14: queen
                // 15: king

                match ch.to_ascii_lowercase() {
                    '/' => rank -= 1,
                    'p' => {
                        counts[0] += 1;
                        pieces[counts[0]].rank = rank;
                        pieces[counts[0]].file = file;
                    },
                    'r' => {
                        counts[1] += 1;
                        pieces[8 + counts[1]].rank = rank;
                        pieces[8 + counts[1]].file = file;
                    },
                    'n' => {
                        counts[2] += 1;
                        pieces[10 + counts[2]].rank = rank;
                        pieces[10 + counts[2]].file = file;
                    },
                    'b' => {
                        counts[3] += 1;
                        pieces[12 + counts[3]].rank = rank;
                        pieces[12 + counts[3]].file = file;
                    },
                    'q' => {
                        counts[4] += 1;
                        pieces[14].rank = rank;
                        pieces[14].file = file;
                    },
                    ' ' => break,
                    _ => continue
                }
            }
        }

        if black_piece_counts[0] < 8 {
            for i in black_piece_counts[0]..9 {
                self.black_pieces[0 + black_piece_counts[0]].rank = 0;
                self.black_pieces[0 + black_piece_counts[0]].file = 0;
            }
        }
    }
}