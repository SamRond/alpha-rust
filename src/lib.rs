mod utils;

use core::str;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Clone, PartialEq)]
pub enum PieceType {
    Pawn {val: i8},
    Knight {val: i8},
    Bishop {val: i8},
    Rook {val: i8},
    Queen {val: i8},
    King {val: i8}
}

#[derive(Clone, PartialEq)]
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

impl Piece {
    // gets tuple of (rank, file)
    pub fn get_position(&self) -> (u32, u32) {
        (self.rank, self.file)
    }
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
    pub fn get_fen(&self) -> String {
        self.fen.clone()
    }

    pub fn set_fen(&mut self, fen_in: String) {
        self.fen = fen_in
    }

    // mostly exists for testing; returns a cloned vec of the white pieces
    pub fn get_white_pieces(&self) -> Vec<Piece> {
        self.white_pieces.clone()
    }

    // mostly exists for testing; returns a cloned vec of the white pieces
    pub fn get_black_pieces(&self) -> Vec<Piece> {
        self.black_pieces.clone()
    }

    pub fn new(fen_in: String) -> Board {
        let mut fen = fen_in;

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

        white_pieces.push(Piece {
            kind: PieceType::Pawn { val: 1 },
            color: Color::White,
            rank: 0,
            file: 0
        });
        white_pieces.push(Piece {
            kind: PieceType::Pawn { val: 1 },
            color: Color::White,
            rank: 0,
            file: 0
        });
        white_pieces.push(Piece {
            kind: PieceType::Pawn { val: 1 },
            color: Color::White,
            rank: 0,
            file: 0
        });
        white_pieces.push(Piece {
            kind: PieceType::Pawn { val: 1 },
            color: Color::White,
            rank: 0,
            file: 0
        });
        white_pieces.push(Piece {
            kind: PieceType::Pawn { val: 1 },
            color: Color::White,
            rank: 0,
            file: 0
        });
        white_pieces.push(Piece {
            kind: PieceType::Pawn { val: 1 },
            color: Color::White,
            rank: 0,
            file: 0
        });
        white_pieces.push(Piece {
            kind: PieceType::Pawn { val: 1 },
            color: Color::White,
            rank: 0,
            file: 0
        });
        white_pieces.push(Piece {
            kind: PieceType::Pawn { val: 1 },
            color: Color::White,
            rank: 0,
            file: 0
        });
        white_pieces.push(Piece {
            kind: PieceType::Rook { val: 5 },
            color: Color::White,
            rank: 0,
            file: 0
        });
        white_pieces.push(Piece {
            kind: PieceType::Rook { val: 5 },
            color: Color::White,
            rank: 0,
            file: 0
        });
        white_pieces.push(Piece {
            kind: PieceType::Knight { val: 3 },
            color: Color::White,
            rank: 0,
            file: 0
        });
        white_pieces.push(Piece {
            kind: PieceType::Knight { val: 3 },
            color: Color::White,
            rank: 0,
            file: 0
        });
        white_pieces.push(Piece {
            kind: PieceType::Bishop { val: 3 },
            color: Color::White,
            rank: 0,
            file: 0
        });
        white_pieces.push(Piece {
            kind: PieceType::Bishop { val: 3 },
            color: Color::White,
            rank: 0,
            file: 0
        });
        white_pieces.push(Piece {
            kind: PieceType::Queen { val: 9 },
            color: Color::White,
            rank: 0,
            file: 0
        });
        white_pieces.push(Piece {
            kind: PieceType::King { val: 127 },
            color: Color::White,
            rank: 0,
            file: 0
        });

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

        black_pieces.push(Piece {
            kind: PieceType::Pawn { val: 1 },
            color: Color::Black,
            rank: 0,
            file: 0
        });
        black_pieces.push(Piece {
            kind: PieceType::Pawn { val: 1 },
            color: Color::Black,
            rank: 0,
            file: 0
        });
        black_pieces.push(Piece {
            kind: PieceType::Pawn { val: 1 },
            color: Color::Black,
            rank: 0,
            file: 0
        });
        black_pieces.push(Piece {
            kind: PieceType::Pawn { val: 1 },
            color: Color::Black,
            rank: 0,
            file: 0
        });
        black_pieces.push(Piece {
            kind: PieceType::Pawn { val: 1 },
            color: Color::Black,
            rank: 0,
            file: 0
        });
        black_pieces.push(Piece {
            kind: PieceType::Pawn { val: 1 },
            color: Color::Black,
            rank: 0,
            file: 0
        });
        black_pieces.push(Piece {
            kind: PieceType::Pawn { val: 1 },
            color: Color::Black,
            rank: 0,
            file: 0
        });
        black_pieces.push(Piece {
            kind: PieceType::Pawn { val: 1 },
            color: Color::Black,
            rank: 0,
            file: 0
        });
        black_pieces.push(Piece {
            kind: PieceType::Rook { val: 5 },
            color: Color::Black,
            rank: 0,
            file: 0
        });
        black_pieces.push(Piece {
            kind: PieceType::Rook { val: 5 },
            color: Color::Black,
            rank: 0,
            file: 0
        });
        black_pieces.push(Piece {
            kind: PieceType::Knight { val: 3 },
            color: Color::Black,
            rank: 0,
            file: 0
        });
        black_pieces.push(Piece {
            kind: PieceType::Knight { val: 3 },
            color: Color::Black,
            rank: 0,
            file: 0
        });
        black_pieces.push(Piece {
            kind: PieceType::Bishop { val: 3 },
            color: Color::Black,
            rank: 0,
            file: 0
        });
        black_pieces.push(Piece {
            kind: PieceType::Bishop { val: 3 },
            color: Color::Black,
            rank: 0,
            file: 0
        });
        black_pieces.push(Piece {
            kind: PieceType::Queen { val: 9 },
            color: Color::Black,
            rank: 0,
            file: 0
        });
        black_pieces.push(Piece {
            kind: PieceType::King { val: 127 },
            color: Color::Black,
            rank: 0,
            file: 0
        });

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
                file += match ch.to_digit(10) {
                    None => 0,
                    Some(x) => x, // x is now an integer value
                };
            } else {
                // index positions of pieces in arr of pieces
                // 0 - 7: pawns
                // 8 - 9: rooks
                // 10 - 11: knights
                // 12 - 13: bishops
                // 14: queen
                // 15: king

                match ch.to_ascii_lowercase() {
                    '/' => {
                        rank -= 1;
                        file = 0;
                    },
                    'p' => {
                        file += 1;
                        counts[0] += 1;
                        pieces[counts[0]].rank = rank;
                        pieces[counts[0]].file = file;
                    },
                    'r' => {
                        file += 1;
                        counts[1] += 1;
                        pieces[8 + counts[1]].rank = rank;
                        pieces[8 + counts[1]].file = file;
                    },
                    'n' => {
                        file += 1;
                        counts[2] += 1;
                        pieces[10 + counts[2]].rank = rank;
                        pieces[10 + counts[2]].file = file;
                    },
                    'b' => {
                        file += 1;
                        counts[3] += 1;
                        pieces[12 + counts[3]].rank = rank;
                        pieces[12 + counts[3]].file = file;
                    },
                    'q' => {
                        file += 1;
                        counts[4] += 1;
                        pieces[14].rank = rank;
                        pieces[14].file = file;
                    },
                    'k' => {
                        file += 1;
                        pieces[15].rank = rank;
                        pieces[15].file = file;
                    }
                    ' ' => break, // space indicates the end of the position section
                    _ => continue
                }
            }
        }

        // index positions of pieces in arr of pieces
        // 0 - 7: pawns
        // 8 - 9: rooks
        // 10 - 11: knights
        // 12 - 13: bishops
        // 14: queen
        // 15: king

        // update missing pieces to have rank/file of 0 (lost)
        
        // black pawns
        if black_piece_counts[0] < 8 {
            for i in black_piece_counts[0]..9 {
                self.black_pieces[black_piece_counts[0]].rank = 0;
                self.black_pieces[black_piece_counts[0]].file = 0;
            }
        }

        // white pawns
        if white_piece_counts[0] < 8 {
            for i in white_piece_counts[0]..9 {
                self.white_pieces[white_piece_counts[0]].rank = 0;
                self.white_pieces[white_piece_counts[0]].file = 0;
            }
        }

        // black rooks
        if black_piece_counts[1] < 2 {
            for i in black_piece_counts[1]..3 {
                self.black_pieces[8 + black_piece_counts[1]].rank = 0;
                self.black_pieces[8 + black_piece_counts[1]].file = 0;
            }
        }

        // white rooks
        if white_piece_counts[1] < 2 {
            for i in white_piece_counts[1]..3 {
                self.white_pieces[8 + white_piece_counts[1]].rank = 0;
                self.white_pieces[8 + white_piece_counts[1]].file = 0;
            }
        }

        // black knights
        if black_piece_counts[2] < 2 {
            for i in black_piece_counts[1]..3 {
                self.black_pieces[10 + black_piece_counts[2]].rank = 0;
                self.black_pieces[10 + black_piece_counts[2]].file = 0;
            }
        }

        // white knights
        if white_piece_counts[2] < 2 {
            for i in white_piece_counts[1]..3 {
                self.white_pieces[10 + white_piece_counts[2]].rank = 0;
                self.white_pieces[10 + white_piece_counts[2]].file = 0;
            }
        }

        // black bishops
        if black_piece_counts[3] < 2 {
            for i in black_piece_counts[1]..3 {
                self.black_pieces[12 + black_piece_counts[3]].rank = 0;
                self.black_pieces[12 + black_piece_counts[3]].file = 0;
            }
        }

        // white bishops
        if white_piece_counts[3] < 2 {
            for i in white_piece_counts[1]..3 {
                self.white_pieces[12 + white_piece_counts[3]].rank = 0;
                self.white_pieces[12 + white_piece_counts[3]].file = 0;
            }
        }

        // black queen
        if black_piece_counts[4] == 0 {
            self.black_pieces[14].rank = 0;
            self.black_pieces[14].file = 0;
        }

        // white queen
        if white_piece_counts[4] == 0 {
            self.white_pieces[14].rank = 0;
            self.white_pieces[14].file = 0;
        }
    }

    // given a piece, return a vec of all valid squares for it
    fn get_valid_moves(&self, piece:&Piece) -> Vec<(u32, u32)> {
        let mut coords:Vec<(u32, u32)> = Vec::new();

        coords.push((piece.rank, piece.file)); // a piece can usually stay on the same square I guess...
        // TODO: king can't stay on the same square if it's in check
        // actally, nobody can move unless they're blocking check


        coords.push((4, 5)); // added for testing purposes

        // must include move validity such as moving into/away from check
        // also must eliminate exposing king to check

        match piece.kind {
            PieceType::Pawn { val: 1 } => {
                if piece.color == Color::White {
                    println!("RANK {} AND FILE {}", piece.rank, piece.file);

                    let one_space = (piece.rank + 1, piece.file);
                    let two_space = (piece.rank + 2, piece.file);
                    let capture_square_1 = (piece.rank + 1, piece.file - 1);
                    let capture_square_2 = (piece.rank + 1, piece.file + 1);

                    if Board::valid_square(one_space) && self.find_piece_by_coords(one_space.0, one_space.1) == None { coords.push(one_space); }
                    if Board::valid_square(two_space) && piece.rank == 2 && self.find_piece_by_coords(two_space.0, two_space.1) == None { coords.push(two_space); }
                    if Board::valid_square(capture_square_1) && self.find_piece_by_coords(capture_square_1.0, capture_square_1.1) != None { coords.push(capture_square_1); }
                    if Board::valid_square(capture_square_2) && self.find_piece_by_coords(capture_square_2.0, capture_square_2.1) != None { coords.push(capture_square_2); }
                } else {
                    let one_space = (piece.rank - 1, piece.file);
                    let two_space = (piece.rank - 2, piece.file);
                    let capture_square_1 = (piece.rank - 1, piece.file - 1);
                    let capture_square_2 = (piece.rank - 1, piece.file + 1);

                    if self.find_piece_by_coords(one_space.0, one_space.1) == None { coords.push(one_space); }
                    if piece.rank == 7 && self.find_piece_by_coords(two_space.0, two_space.1) == None { coords.push(two_space); }
                    if self.find_piece_by_coords(capture_square_1.0, capture_square_1.1) != None { coords.push(capture_square_1); }
                    if self.find_piece_by_coords(capture_square_2.0, capture_square_2.1) != None { coords.push(capture_square_2); }
                }
            },
            _ => {

            }
        }

        return coords;
    }

    fn valid_square(coord:(u32,u32)) -> bool {
        if coord.0 > 8 || coord.0 < 1 { return false }
        if coord.1 > 8 || coord.0 < 1 { return false }

        return true;
    }

    // takes in rank/file coordinates, and returns the optional tuple (white:boolean, index:u32)
    pub fn find_piece_by_coords(&self, rank:u32, file:u32) -> Option<(bool, u32)> {
        for (i, p) in self.white_pieces.iter().enumerate() {
            if p.rank == rank && p.file == file { return Some((true, i as u32)); }
        }

        for (i, p) in self.black_pieces.iter().enumerate() {
            if p.rank == rank && p.file == file { return Some((false, i as u32)); }
        }

        return None;
    }

    // moves a given piece to the specified rank/file
    // eliminates any pieces that exist there, and updates the FEN
    // returns true if move is successfully made
    pub fn make_move(&mut self, piece:&Piece, rank:u32, file:u32) -> bool {
        // check that given rank/file are in the list of valid moves
        // if not, return false
        let moves = self.get_valid_moves(piece);
        if !moves.iter().any(|&i| i == (rank, file)) { return false; }

        // separate the six whitespace-separated fields in the FEN string
        let mut fields = self.fen.split_whitespace().collect::<Vec<&str>>();
        
        // check if the correct side is trying to move
        // if not, return false
        if !((fields[1] == "w" && piece.color == Color::White) || (fields[1] == "b" && piece.color == Color::Black)) {
            return false;
        }

        // single out the first field, the position section
        let mut position_section = &mut fields[0];
        let mut ranks = position_section.split('/').collect::<Vec<&str>>();
        

    
        // rejoin ranks with '/' delimeter and assign to dereferenced position_section (fields[0])
        let new_position = ranks.join("/");
        *position_section = new_position.as_str();


        // rejoin FEN fields with a whitespace delimeter
        let fen = fields.join(" ");
        self.set_fen(fen);

        return true;
    }
}