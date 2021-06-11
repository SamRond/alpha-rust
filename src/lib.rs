use core::str;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[derive(Clone, PartialEq, Debug)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}

#[wasm_bindgen]
#[derive(Clone, PartialEq, Debug)]
pub enum Color {
    White,
    Black
}

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq)]
pub struct Piece {
    kind: PieceType,
    color: Color,
    rank: i32,
    file: i32
}

#[wasm_bindgen]
impl Piece {
    // gets tuple of (rank, file)
    pub fn get_position(&self) -> Coordinates {
        Coordinates {
            rank: self.rank,
            file: self.file
        }
    }
}

#[wasm_bindgen]
#[derive(PartialEq, Debug)]
pub struct Coordinates {
    pub rank: i32,
    pub file: i32
}

#[wasm_bindgen]
pub struct BoardSingleton {
    board: Board
}

#[wasm_bindgen]
impl BoardSingleton {
    #[wasm_bindgen(constructor)]
    pub fn new() -> BoardSingleton {
        let singleton = BoardSingleton {
            board: Board::new("".to_string())
        };

        return singleton;
    }

    pub fn set_fen(&mut self, fen: &str) {
        self.board.set_fen(fen.to_string());
    }

    pub fn get_board_string(&self) -> String {
        let mut string = "<table><tbody>".to_string();
        let mut char = ' ';
    
        for i in (1..9).rev() {
            string += "<tr>";
    
            for j in 1..9 {
                match self.board.find_piece_by_coords(i, j) {
                    Some(x) => {
                        match x.kind {
                            PieceType::Pawn => {
                                char = 'p';
                            },
                            PieceType::Rook => {
                                char = 'r';
                            },
                            PieceType::Knight => {
                                char = 'n';
                            },
                            PieceType::Bishop => {
                                char = 'b';
                            }
                            PieceType::Queen => {
                                char = 'q';
                            },
                            PieceType::King => {
                                char = 'k';
                            },
                        }
    
                        if x.color == Color::White {
                            char = char.to_ascii_uppercase();
                        }
                    },
                    None => {
                        char = '+';
                    }
                }
    
                string += "<td>";
                string.push(char);
                string += "</td>";
            }
    
            string += "</tr>";
        }
    
        string += "</tbody></table>";
    
        return string;
    }
}

#[wasm_bindgen]
#[derive(Clone)]
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
        self.fen = fen_in;
        self.set_piece_coords()
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
            kind: PieceType::Pawn,
            color: Color::White,
            rank: 0,
            file: 0
        });
        white_pieces.push(Piece {
            kind: PieceType::Pawn,
            color: Color::White,
            rank: 0,
            file: 0
        });
        white_pieces.push(Piece {
            kind: PieceType::Pawn,
            color: Color::White,
            rank: 0,
            file: 0
        });
        white_pieces.push(Piece {
            kind: PieceType::Pawn,
            color: Color::White,
            rank: 0,
            file: 0
        });
        white_pieces.push(Piece {
            kind: PieceType::Pawn,
            color: Color::White,
            rank: 0,
            file: 0
        });
        white_pieces.push(Piece {
            kind: PieceType::Pawn,
            color: Color::White,
            rank: 0,
            file: 0
        });
        white_pieces.push(Piece {
            kind: PieceType::Pawn,
            color: Color::White,
            rank: 0,
            file: 0
        });
        white_pieces.push(Piece {
            kind: PieceType::Pawn,
            color: Color::White,
            rank: 0,
            file: 0
        });
        white_pieces.push(Piece {
            kind: PieceType::Rook,
            color: Color::White,
            rank: 0,
            file: 0
        });
        white_pieces.push(Piece {
            kind: PieceType::Rook,
            color: Color::White,
            rank: 0,
            file: 0
        });
        white_pieces.push(Piece {
            kind: PieceType::Knight,
            color: Color::White,
            rank: 0,
            file: 0
        });
        white_pieces.push(Piece {
            kind: PieceType::Knight,
            color: Color::White,
            rank: 0,
            file: 0
        });
        white_pieces.push(Piece {
            kind: PieceType::Bishop,
            color: Color::White,
            rank: 0,
            file: 0
        });
        white_pieces.push(Piece {
            kind: PieceType::Bishop,
            color: Color::White,
            rank: 0,
            file: 0
        });
        white_pieces.push(Piece {
            kind: PieceType::Queen,
            color: Color::White,
            rank: 0,
            file: 0
        });
        white_pieces.push(Piece {
            kind: PieceType::King,
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
            kind: PieceType::Pawn,
            color: Color::Black,
            rank: 0,
            file: 0
        });
        black_pieces.push(Piece {
            kind: PieceType::Pawn,
            color: Color::Black,
            rank: 0,
            file: 0
        });
        black_pieces.push(Piece {
            kind: PieceType::Pawn,
            color: Color::Black,
            rank: 0,
            file: 0
        });
        black_pieces.push(Piece {
            kind: PieceType::Pawn,
            color: Color::Black,
            rank: 0,
            file: 0
        });
        black_pieces.push(Piece {
            kind: PieceType::Pawn,
            color: Color::Black,
            rank: 0,
            file: 0
        });
        black_pieces.push(Piece {
            kind: PieceType::Pawn,
            color: Color::Black,
            rank: 0,
            file: 0
        });
        black_pieces.push(Piece {
            kind: PieceType::Pawn,
            color: Color::Black,
            rank: 0,
            file: 0
        });
        black_pieces.push(Piece {
            kind: PieceType::Pawn,
            color: Color::Black,
            rank: 0,
            file: 0
        });
        black_pieces.push(Piece {
            kind: PieceType::Rook,
            color: Color::Black,
            rank: 0,
            file: 0
        });
        black_pieces.push(Piece {
            kind: PieceType::Rook,
            color: Color::Black,
            rank: 0,
            file: 0
        });
        black_pieces.push(Piece {
            kind: PieceType::Knight,
            color: Color::Black,
            rank: 0,
            file: 0
        });
        black_pieces.push(Piece {
            kind: PieceType::Knight,
            color: Color::Black,
            rank: 0,
            file: 0
        });
        black_pieces.push(Piece {
            kind: PieceType::Bishop,
            color: Color::Black,
            rank: 0,
            file: 0
        });
        black_pieces.push(Piece {
            kind: PieceType::Bishop,
            color: Color::Black,
            rank: 0,
            file: 0
        });
        black_pieces.push(Piece {
            kind: PieceType::Queen,
            color: Color::Black,
            rank: 0,
            file: 0
        });
        black_pieces.push(Piece {
            kind: PieceType::King,
            color: Color::Black,
            rank: 0,
            file: 0
        });

        black_pieces
    }

    // Iterates through the FEN and properly sets the coordinates of each piece
    fn set_piece_coords(&mut self) {
        let mut rank: i32 = 8;
        let mut file: i32 = 0;
        
        let mut white:bool;

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
                    Some(x) => x as i32, // x is now an integer value
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
                        pieces[counts[0] - 1].rank = rank;
                        pieces[counts[0] - 1].file = file;

                        // println!("Placed {} pawn #{} at rank {} and file {}", if white { "white" } else { "black" }, counts[0], rank, file);
                    },
                    'r' => {
                        file += 1;
                        counts[1] += 1;
                        pieces[8 + counts[1] - 1].rank = rank;
                        pieces[8 + counts[1] - 1].file = file;
                        
                        // println!("Placed {} rook #{} at rank {} and file {}", if white { "white" } else { "black" }, counts[1], rank, file);
                    },
                    'n' => {
                        file += 1;
                        counts[2] += 1;
                        pieces[10 + counts[2] - 1].rank = rank;
                        pieces[10 + counts[2] - 1].file = file;
                        
                        // println!("Placed {} knight #{} at rank {} and file {}", if white { "white" } else { "black" }, counts[2], rank, file);
                    },
                    'b' => {
                        file += 1;
                        counts[3] += 1;
                        pieces[12 + counts[3] - 1].rank = rank;
                        pieces[12 + counts[3] - 1].file = file;
                        
                        // println!("Placed {} bishop #{} at rank {} and file {}", if white { "white" } else { "black" }, counts[3], rank, file);
                    },
                    'q' => {
                        file += 1;
                        counts[4] += 1;
                        pieces[14].rank = rank;
                        pieces[14].file = file;
                        
                        // println!("Placed {} queen #{} at rank {} and file {}", if white { "white" } else { "black" }, counts[4], rank, file);
                    },
                    'k' => {
                        file += 1;
                        pieces[15].rank = rank;
                        pieces[15].file = file;
                        
                        // println!("Placed {} king at rank {} and file {}", if white { "white" } else { "black" }, rank, file);
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
            for _ in black_piece_counts[0]..9 {
                self.black_pieces[black_piece_counts[0]].rank = 0;
                self.black_pieces[black_piece_counts[0]].file = 0;
            }
        }

        // white pawns
        if white_piece_counts[0] < 8 {
            for _ in white_piece_counts[0]..9 {
                self.white_pieces[white_piece_counts[0]].rank = 0;
                self.white_pieces[white_piece_counts[0]].file = 0;
            }
        }

        // black rooks
        if black_piece_counts[1] < 2 {
            for _ in black_piece_counts[1]..3 {
                self.black_pieces[8 + black_piece_counts[1]].rank = 0;
                self.black_pieces[8 + black_piece_counts[1]].file = 0;
            }
        }

        // white rooks
        if white_piece_counts[1] < 2 {
            for _ in white_piece_counts[1]..3 {
                self.white_pieces[8 + white_piece_counts[1]].rank = 0;
                self.white_pieces[8 + white_piece_counts[1]].file = 0;
            }
        }

        // black knights
        if black_piece_counts[2] < 2 {
            for _ in black_piece_counts[1]..3 {
                self.black_pieces[10 + black_piece_counts[2]].rank = 0;
                self.black_pieces[10 + black_piece_counts[2]].file = 0;
            }
        }

        // white knights
        if white_piece_counts[2] < 2 {
            for _ in white_piece_counts[1]..3 {
                self.white_pieces[10 + white_piece_counts[2]].rank = 0;
                self.white_pieces[10 + white_piece_counts[2]].file = 0;
            }
        }

        // black bishops
        if black_piece_counts[3] < 2 {
            for _ in black_piece_counts[1]..3 {
                self.black_pieces[12 + black_piece_counts[3]].rank = 0;
                self.black_pieces[12 + black_piece_counts[3]].file = 0;
            }
        }

        // white bishops
        if white_piece_counts[3] < 2 {
            for _ in white_piece_counts[1]..3 {
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
    fn get_valid_moves(&self, piece:&Piece) -> Vec<(i32, i32)> {
        let mut coords:Vec<(i32, i32)> = Vec::new();

        // if piece is not on the board, it has no valid moves
        if piece.rank == 0 || piece.file == 0 { return coords; }

        // if the king is in check, nobody else can move unless they're blocking check or attacker is eliminated
        // also must eliminate exposing king to check

        match piece.kind {
            PieceType::Pawn => {
                // team indicates pawn direction; pawns can only move forward and therefore movement is team dependent
                // white moves up the board, therefore direction is positive
                let team;
                if piece.color == Color::White {
                    team = 1;
                } else {
                    team = -1;
                }

                // each var represents a possible movement of the pawn
                let one_space = (piece.rank + team, piece.file);
                let two_space = (piece.rank + 2 * team, piece.file);
                let capture_square_1 = (piece.rank + team, piece.file - 1);
                let capture_square_2 = (piece.rank + team, piece.file + 1);

                // for pawn movements, forward squares must be clear, must be on second or seventh rank for double movement, and diagonal squares must be occupied by enemy pieces for a move to be valid
                if Board::valid_square(one_space) && self.find_piece_by_coords(one_space.0, one_space.1).is_none() { coords.push(one_space); }
                if Board::valid_square(two_space) && piece.rank == (4.5 - 2.5 * (team as f64)) as i32 && self.find_piece_by_coords(one_space.0, one_space.1).is_none() && self.find_piece_by_coords(two_space.0, two_space.1).is_none() { coords.push(two_space); }
                if Board::valid_square(capture_square_1) && !self.find_piece_by_coords(capture_square_1.0, capture_square_1.1).is_none() && self.find_piece_by_coords(capture_square_1.0, capture_square_1.1).unwrap().color != piece.color { coords.push(capture_square_1); }
                if Board::valid_square(capture_square_2) && !self.find_piece_by_coords(capture_square_2.0, capture_square_2.1).is_none() && self.find_piece_by_coords(capture_square_2.0, capture_square_2.1).unwrap().color != piece.color { coords.push(capture_square_2); }
            },
            PieceType::Knight => {
                for x in -2..2 {
                    for y in -2..2 {
                        // when x is +/- 1       and y is +/- 2,        OR   x is +/- 2          and y is +/- 1,             and the target square is valid
                        if (((x == 1 || x == -1) && (y == 2 || y == -2)) || ((x == 2 || x == -2) && (y == 1 || y == -1))) && Board::valid_square((piece.rank + x, piece.file + y)) {
                            let same_team = match self.find_piece_by_coords(piece.rank + x, piece.rank + y) {
                                Some(x) => x.color == piece.color, // when the target square is not occupied by a same-color piece
                                None => false,
                            };
                            if !same_team {coords.push((piece.rank + x, piece.file + y))} // add to coords
                        }
                    }
                }
            },
            PieceType::Bishop => todo!(),
            PieceType::Rook => todo!(),
            PieceType::Queen => todo!(),
            PieceType::King => todo!(),
        }

        return coords;
    }

    fn valid_square(coord:(i32,i32)) -> bool {
        if coord.0 > 8 || coord.0 < 1 { return false }
        if coord.1 > 8 || coord.1 < 1 { return false }

        return true;
    }

    // // takes in rank/file coordinates, and returns the optional tuple (white:boolean, index:i32)
    pub fn find_piece_by_coords(&self, rank:i32, file:i32) -> Option<&Piece> {
        for p in self.white_pieces.iter() {
            if p.rank == rank && p.file == file { return Some(p); }
        }

        for p in self.black_pieces.iter() {
            if p.rank == rank && p.file == file { return Some(p); }
        }

        return None;
    }

    // moves a given piece to the specified rank/file
    // eliminates any pieces that exist there, and updates the FEN
    // returns true if move is successfully made
    pub fn make_move(&mut self, piece:&Piece, rank:i32, file:i32) -> bool {
        // check that given rank/file are in the list of valid moves
        // if not, return false
        let moves = self.get_valid_moves(piece);
        if !moves.iter().any(|&i| i == (rank, file)) { return false; }

        // separate the six whitespace-separated fields in the FEN string
        let mut fields = self.fen.split_whitespace().collect::<Vec<&str>>();
        
        // check if the correct side is trying to move
        // if not, return false
        if !(self.get_side_to_move() == piece.color) {
            return false;
        }
        // single out the first field, the position section
        let mut position_section = &mut fields[0];
        let mut ranks = position_section.split('/').collect::<Vec<&str>>();
        

        
        // actually make the move here. Don't forget to increment the halfmove and full move counters (fields[4] and fields[5] respectively)
        

    
        // rejoin ranks with '/' delimeter and assign to dereferenced position_section (fields[0])
        let new_position = ranks.join("/");
        *position_section = new_position.as_str();


        // rejoin FEN fields with a whitespace delimeter
        let fen = fields.join(" ");
        self.set_fen(fen);

        return true;
    }

    fn get_side_to_move(&self) -> Color {
        // break up FEN into six whitespace-delimited sections
        let fields = self.fen.split_whitespace().collect::<Vec<&str>>();

        if fields[1] == "w" {
            return Color::White;
        } else {
            return Color::Black;
        }
    }
}