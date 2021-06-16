use core::str;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq, Debug)]
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

// mostly exists as a wrapper class so Javascript can interact with the board
//
// complex values (vec, arr, etc.) cannot be passed directly to javascript, 
// so no methods in this struct can use them directly as params or return values
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
    white_pieces: [Vec<Piece>; 6],

    // Black pieces
    black_pieces: [Vec<Piece>; 6]
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
    pub fn get_white_pieces(&self) -> [Vec<Piece>; 6] {
        self.white_pieces.clone()
    }

    // mostly exists for testing; returns a cloned vec of the white pieces
    pub fn get_black_pieces(&self) -> [Vec<Piece>; 6] {
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

    // Fills 2d array of white pieces
    //
    // 0: pawns
    // 1: knights
    // 2: bishops
    // 3: rooks
    // 4: queens
    // 5: kings
    fn init_white_pieces() -> [Vec<Piece>; 6] {
        let mut white_pieces = [
            Vec::with_capacity(8), // max number of pawns is 8
            Vec::with_capacity(10), // max number of knights, bishops, and rooks is 10 (2 on board + 8 promotions)
            Vec::with_capacity(10),
            Vec::with_capacity(10),
            Vec::with_capacity(9), // max number of queens on the board (1 on board + 8 promotions)
            Vec::with_capacity(1), // max number of kings on the board
        ]; 

        white_pieces[0].push(Piece {
            kind: PieceType::Pawn,
            color: Color::White,
            rank: 0,
            file: 0
        });
        white_pieces[0].push(Piece {
            kind: PieceType::Pawn,
            color: Color::White,
            rank: 0,
            file: 0
        });
        white_pieces[0].push(Piece {
            kind: PieceType::Pawn,
            color: Color::White,
            rank: 0,
            file: 0
        });
        white_pieces[0].push(Piece {
            kind: PieceType::Pawn,
            color: Color::White,
            rank: 0,
            file: 0
        });
        white_pieces[0].push(Piece {
            kind: PieceType::Pawn,
            color: Color::White,
            rank: 0,
            file: 0
        });
        white_pieces[0].push(Piece {
            kind: PieceType::Pawn,
            color: Color::White,
            rank: 0,
            file: 0
        });
        white_pieces[0].push(Piece {
            kind: PieceType::Pawn,
            color: Color::White,
            rank: 0,
            file: 0
        });
        white_pieces[0].push(Piece {
            kind: PieceType::Pawn,
            color: Color::White,
            rank: 0,
            file: 0
        });
        white_pieces[3].push(Piece {
            kind: PieceType::Rook,
            color: Color::White,
            rank: 0,
            file: 0
        });
        white_pieces[3].push(Piece {
            kind: PieceType::Rook,
            color: Color::White,
            rank: 0,
            file: 0
        });
        white_pieces[1].push(Piece {
            kind: PieceType::Knight,
            color: Color::White,
            rank: 0,
            file: 0
        });
        white_pieces[1].push(Piece {
            kind: PieceType::Knight,
            color: Color::White,
            rank: 0,
            file: 0
        });
        white_pieces[2].push(Piece {
            kind: PieceType::Bishop,
            color: Color::White,
            rank: 0,
            file: 0
        });
        white_pieces[2].push(Piece {
            kind: PieceType::Bishop,
            color: Color::White,
            rank: 0,
            file: 0
        });
        white_pieces[4].push(Piece {
            kind: PieceType::Queen,
            color: Color::White,
            rank: 0,
            file: 0
        });
        white_pieces[5].push(Piece {
            kind: PieceType::King,
            color: Color::White,
            rank: 0,
            file: 0
        });

        white_pieces
    }

    // Fills 2d array of black pieces
    //
    // 0: pawns
    // 1: knights
    // 2: bishops
    // 3: rooks
    // 4: queens
    // 5: kings
    fn init_black_pieces() -> [Vec<Piece>; 6] {
        let mut black_pieces = [
            Vec::with_capacity(8), // max number of pawns is 8
            Vec::with_capacity(10), // max number of knights, bishops, and rooks is 10 (2 on board + 8 promotions)
            Vec::with_capacity(10),
            Vec::with_capacity(10),
            Vec::with_capacity(9), // max number of queens on the board (1 on board + 8 promotions)
            Vec::with_capacity(1), // max number of kings on the board
        ]; 

        black_pieces[0].push(Piece {
            kind: PieceType::Pawn,
            color: Color::Black,
            rank: 0,
            file: 0
        });
        black_pieces[0].push(Piece {
            kind: PieceType::Pawn,
            color: Color::Black,
            rank: 0,
            file: 0
        });
        black_pieces[0].push(Piece {
            kind: PieceType::Pawn,
            color: Color::Black,
            rank: 0,
            file: 0
        });
        black_pieces[0].push(Piece {
            kind: PieceType::Pawn,
            color: Color::Black,
            rank: 0,
            file: 0
        });
        black_pieces[0].push(Piece {
            kind: PieceType::Pawn,
            color: Color::Black,
            rank: 0,
            file: 0
        });
        black_pieces[0].push(Piece {
            kind: PieceType::Pawn,
            color: Color::Black,
            rank: 0,
            file: 0
        });
        black_pieces[0].push(Piece {
            kind: PieceType::Pawn,
            color: Color::Black,
            rank: 0,
            file: 0
        });
        black_pieces[0].push(Piece {
            kind: PieceType::Pawn,
            color: Color::Black,
            rank: 0,
            file: 0
        });
        black_pieces[3].push(Piece {
            kind: PieceType::Rook,
            color: Color::Black,
            rank: 0,
            file: 0
        });
        black_pieces[3].push(Piece {
            kind: PieceType::Rook,
            color: Color::Black,
            rank: 0,
            file: 0
        });
        black_pieces[1].push(Piece {
            kind: PieceType::Knight,
            color: Color::Black,
            rank: 0,
            file: 0
        });
        black_pieces[1].push(Piece {
            kind: PieceType::Knight,
            color: Color::Black,
            rank: 0,
            file: 0
        });
        black_pieces[2].push(Piece {
            kind: PieceType::Bishop,
            color: Color::Black,
            rank: 0,
            file: 0
        });
        black_pieces[2].push(Piece {
            kind: PieceType::Bishop,
            color: Color::Black,
            rank: 0,
            file: 0
        });
        black_pieces[4].push(Piece {
            kind: PieceType::Queen,
            color: Color::Black,
            rank: 0,
            file: 0
        });
        black_pieces[5].push(Piece {
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

        // keeps track of the number of each piece in the array of pieces, and subtracts for this number for each one that has been seen
        // [pawns, knights, bishops, rooks, queen]
        let mut white_piece_counts = [0, 0, 0, 0, 0];
        let mut black_piece_counts = [0, 0, 0, 0, 0];

        // populate count arrays 
        self.count_pieces(&self.white_pieces, &mut white_piece_counts);
        self.count_pieces(&self.black_pieces, &mut black_piece_counts);

        // indexes in 2d array of pieces
        // 0: pawns
        // 1: knights
        // 2: bishops
        // 3: rooks
        // 4: queens
        // 5: kings

        for ch in self.fen.chars() {
            white = ch.is_ascii_uppercase();
            
            let pieces;
            let counts;
            
            if white {
                pieces = &mut (self.white_pieces);
                counts = &mut white_piece_counts;
            } else {
                pieces = &mut (self.black_pieces);
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
        
                file += 1; // all match arms except '/' do this

                match ch.to_ascii_lowercase() {
                    '/' => {
                        rank -= 1;
                        file = 0;
                    },
                    'p' => {
                        Board::update_piece_coordinates(pieces, counts, 0, white, rank, file);
                    },
                    'n' => {
                        Board::update_piece_coordinates(pieces, counts, 1, white, rank, file);
                    },
                    'b' => {
                        Board::update_piece_coordinates(pieces, counts, 2, white, rank, file);
                    },
                    'r' => {
                        Board::update_piece_coordinates(pieces, counts, 3, white, rank, file);
                    },
                    'q' => {
                        Board::update_piece_coordinates(pieces, counts, 4, white, rank, file);
                    },
                    'k' => {
                        pieces[5][0].rank = rank;
                        pieces[5][0].file = file;
                    }
                    ' ' => break, // space indicates the end of the position section
                    _ => continue
                }
            }
        }

        // index positions of pieces in arr of pieces
        // 0: pawns
        // 1: knights
        // 2: bishops
        // 3: rooks
        // 4: queens
        // 5: kings

        // update missing pieces to have rank/file of 0 (lost)
        
        for i in 0..5 {
            while black_piece_counts[i] > 0 {
                let index = self.black_pieces[i].len() - black_piece_counts[i];
                self.black_pieces[i][index].rank = 0;
                self.black_pieces[i][index].file = 0;

                black_piece_counts[i] -= 1;
            }

            while white_piece_counts[i] > 0 {
                let index = self.white_pieces[i].len() - white_piece_counts[i];
                self.white_pieces[i][index].rank = 0;
                self.white_pieces[i][index].file = 0;

                white_piece_counts[i] -= 1;
            }
        }
    }

    // updates piece coordinates for set_piece_coords method
    fn update_piece_coordinates(pieces:&mut [Vec<Piece>; 6], counts:&mut [usize; 5], index:usize, white:bool, rank:i32, file:i32) {
        if counts[index] == 0 {
            let kind = pieces[index][0].kind.clone();

            let mut piece = Piece {
                kind,
                color: Color::Black,
                rank: 0,
                file: 0
            };

            if white { piece.color = Color::White }


            pieces[index].push(piece);
            counts[index] += 1;

            println!("Added new piece of type: {:?}", kind);
        }

        let index2 = pieces[index].len() - counts[index];
        counts[index] -= 1;
        pieces[index][index2].rank = rank;
        pieces[index][index2].file = file;
    }

    // counts array indexes: [pawns, rooks, knights, bishops, queen]
    fn count_pieces(&self, pieces:&[Vec<Piece>; 6], counts:&mut [usize; 5]) {
        for i in 0..5 {
            counts[i] = pieces[i].len();
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
                for x in -2..3 {
                    for y in -2..3 {
                        // when x is +/- 1       and y is +/- 2,        OR   x is +/- 2          and y is +/- 1,             and the target square is valid
                        if (((x == 1 || x == -1) && (y == 2 || y == -2)) || ((x == 2 || x == -2) && (y == 1 || y == -1))) && Board::valid_square((piece.rank + x, piece.file + y)) {
                            let same_team = match self.find_piece_by_coords(piece.rank + x, piece.file + y) {
                                Some(x) => x.color == piece.color, // when the target square is not occupied by a same-color piece
                                None => false,
                            };
                            if !same_team {coords.push((piece.rank + x, piece.file + y))} // add to coords
                        }
                    }
                }
            },
            PieceType::Bishop => {
                let mut quad_one = true;
                let mut quad_two = true;
                let mut quad_three = true;
                let mut quad_four = true;
                for x in 1..8 {
                    if quad_one {
                        if !Board::valid_square((piece.rank + x, piece.file + x)) {
                            quad_one = false;
                        }
                        else {
                            let same_team = match self.find_piece_by_coords(piece.rank + x, piece.file + x) {
                                Some(y) => {
                                    if y.color != piece.color {
                                        coords.push((piece.rank + x, piece.file + x));
                                    }
                                    true
                                },
                                None => false,
                            };
                            if same_team {quad_one = false;}
                            else {coords.push((piece.rank + x, piece.file + x))}
                        }
                    }
                    if quad_two {
                        if !Board::valid_square((piece.rank - x, piece.file + x)) {
                            quad_two = false;
                        }
                        else {
                            let same_team = match self.find_piece_by_coords(piece.rank - x, piece.file + x) {
                                Some(y) => {
                                    if y.color != piece.color {
                                        coords.push((piece.rank - x, piece.file + x));
                                    }
                                    y.color == piece.color
                                },
                                None => false,
                            };
                            if same_team {quad_two = false;}
                            else {coords.push((piece.rank - x, piece.file + x))}
                        }
                    }
                    if quad_three {
                        if !Board::valid_square((piece.rank - x, piece.file - x)) {
                            quad_three = false;
                        }
                        else {
                            let same_team = match self.find_piece_by_coords(piece.rank - x, piece.file - x) {
                                Some(y) => {
                                    if y.color != piece.color {
                                        coords.push((piece.rank - x, piece.file - x));
                                    }
                                    y.color == piece.color
                                },
                                None => false,
                            };
                            if same_team {quad_three = false;}
                            else {coords.push((piece.rank - x, piece.file - x))}
                        }
                    }
                    if quad_four {
                        if !Board::valid_square((piece.rank + x, piece.file - x)) {
                            quad_four = false;
                        }
                        else {
                            let same_team = match self.find_piece_by_coords(piece.rank + x, piece.file - x) {
                                Some(y) => {
                                    if y.color != piece.color {
                                        coords.push((piece.rank + x, piece.file - x));
                                    }
                                    y.color == piece.color
                                },
                                None => false,
                            };
                            if same_team {quad_four = false;}
                            else {coords.push((piece.rank + x, piece.file - x))}
                        }
                    }
                }
            },
            PieceType::Rook => {
                let mut r = true;
                let mut u = true;
                let mut l = true;
                let mut d = true;
                for x in 1..8 {
                    if r {
                        if !Board::valid_square((piece.rank, piece.file + x)) {
                            r = false;
                        }
                        else {
                            let same_team = match self.find_piece_by_coords(piece.rank, piece.file + x) {
                                Some(y) => {
                                    if y.color == piece.color {
                                        coords.push((piece.rank, piece.file + x));
                                    }
                                    y.color == piece.color
                                },
                                None => false,
                            };
                            if same_team {r = false;}
                            else {coords.push((piece.rank, piece.file + x))}
                        }
                    }
                    if u {
                        if !Board::valid_square((piece.rank, piece.file + x)) {
                            u = false;
                        }
                        else {
                            let same_team = match self.find_piece_by_coords(piece.rank + x, piece.file) {
                                Some(y) => {
                                    if y.color == piece.color {
                                        coords.push((piece.rank + x, piece.file));
                                    }
                                    y.color == piece.color
                                },
                                None => false,
                            };
                            if same_team {u = false;}
                            else {coords.push((piece.rank + x, piece.file))}
                        }
                    }
                    if l {
                        if !Board::valid_square((piece.rank + x, piece.file)) {
                            l = false;
                        }
                        else {
                            let same_team = match self.find_piece_by_coords(piece.rank, piece.file - x) {
                                Some(y) => {
                                    if y.color == piece.color {
                                        coords.push((piece.rank, piece.file - x));
                                    }
                                    y.color == piece.color
                                },
                                None => false,
                            };
                            if same_team {l = false;}
                            else {coords.push((piece.rank, piece.file - x))}
                        }
                    }
                    if d {
                        if !Board::valid_square((piece.rank - x, piece.file)) {
                            d = false;
                        }
                        else {
                            let same_team = match self.find_piece_by_coords(piece.rank - x, piece.file) {
                                Some(y) => {
                                    if y.color == piece.color {
                                        coords.push((piece.rank - x, piece.file));
                                    }
                                    y.color == piece.color
                                },
                                None => false,
                            };
                            if same_team {d = false;}
                            else {coords.push((piece.rank - x, piece.file))}
                        }
                    }
                }
            },
            PieceType::Queen => {
                let mut r = true;
                let mut u = true;
                let mut l = true;
                let mut d = true;
                let mut quad_one = true;
                let mut quad_two = true;
                let mut quad_three = true;
                let mut quad_four = true;
                for x in 1..8 {
                    if r {
                        if !Board::valid_square((piece.rank, piece.file + x)) {
                            r = false;
                            quad_one = false;
                            quad_four = false;
                        }
                        else {
                            let same_team = match self.find_piece_by_coords(piece.rank, piece.file + x) {
                                Some(y) => {
                                    if y.color == piece.color {
                                        coords.push((piece.rank, piece.file + x));
                                    }
                                    y.color == piece.color
                                },
                                None => false,
                            };
                            if same_team {r = false;}
                            else {coords.push((piece.rank, piece.file + x))}
                        }
                    }
                    if u {
                        if !Board::valid_square((piece.rank, piece.file + x)) {
                            u = false;
                            quad_one = false;
                            quad_two = false;
                        }
                        else {
                            let same_team = match self.find_piece_by_coords(piece.rank + x, piece.file) {
                                Some(y) => {
                                    if y.color == piece.color {
                                        coords.push((piece.rank + x, piece.file));
                                    }
                                    y.color == piece.color
                                },
                                None => false,
                            };
                            if same_team {u = false;}
                            else {coords.push((piece.rank + x, piece.file))}
                        }
                    }
                    if l {
                        if !Board::valid_square((piece.rank + x, piece.file)) {
                            l = false;
                            quad_two = false;
                            quad_three = false;
                        }
                        else {
                            let same_team = match self.find_piece_by_coords(piece.rank, piece.file - x) {
                                Some(y) => {
                                    if y.color == piece.color {
                                        coords.push((piece.rank, piece.file - x));
                                    }
                                    y.color == piece.color
                                },
                                None => false,
                            };
                            if same_team {l = false;}
                            else {coords.push((piece.rank, piece.file - x))}
                        }
                    }
                    if d {
                        if !Board::valid_square((piece.rank - x, piece.file)) {
                            d = false;
                            quad_three = false;
                            quad_four = false;
                        }
                        else {
                            let same_team = match self.find_piece_by_coords(piece.rank - x, piece.file) {
                                Some(y) => {
                                    if y.color == piece.color {
                                        coords.push((piece.rank - x, piece.file));
                                    }
                                    y.color == piece.color
                                },
                                None => false,
                            };
                            if same_team {d = false;}
                            else {coords.push((piece.rank - x, piece.file))}
                        }
                    }
                    if quad_one {
                        let same_team = match self.find_piece_by_coords(piece.rank + x, piece.file + x) {
                            Some(y) => {
                                if y.color != piece.color {
                                    coords.push((piece.rank + x, piece.file + x));
                                }
                                true
                            },
                            None => false,
                        };
                        if same_team {quad_one = false;}
                        else {coords.push((piece.rank + x, piece.file + x))}
                    }
                    if quad_two {
                        let same_team = match self.find_piece_by_coords(piece.rank - x, piece.file + x) {
                            Some(y) => {
                                if y.color != piece.color {
                                    coords.push((piece.rank - x, piece.file + x));
                                }
                                y.color == piece.color
                            },
                            None => false,
                        };
                        if same_team {quad_two = false;}
                        else {coords.push((piece.rank - x, piece.file + x))}
                    }
                    if quad_three {
                        let same_team = match self.find_piece_by_coords(piece.rank - x, piece.file - x) {
                            Some(y) => {
                                if y.color != piece.color {
                                    coords.push((piece.rank - x, piece.file - x));
                                }
                                y.color == piece.color
                            },
                            None => false,
                        };
                        if same_team {quad_three = false;}
                        else {coords.push((piece.rank - x, piece.file - x))}
                    }
                    if quad_four {
                        let same_team = match self.find_piece_by_coords(piece.rank + x, piece.file - x) {
                            Some(y) => {
                                if y.color != piece.color {
                                    coords.push((piece.rank + x, piece.file - x));
                                }
                                y.color == piece.color
                            },
                            None => false,
                        };
                        if same_team {quad_four = false;}
                        else {coords.push((piece.rank + x, piece.file - x))}
                    }
                    
                }
            },
            PieceType::King => {
                let castles = self.get_castle_ability(piece.color);
                if castles[0] {
                    let square_one = match self.find_piece_by_coords(piece.rank, piece.file + 1) {
                        Some(x) => false,
                        None => true,
                    };
                    let square_two = match self.find_piece_by_coords(piece.rank, piece.file + 2) {
                        Some(x) => false,
                        None => true,
                    };
                    if square_one && square_two {coords.push((piece.rank, piece.file + 2))}
                }
                if castles[1] {
                    let square_one = match self.find_piece_by_coords(piece.rank, piece.file - 1) {
                        Some(x) => false,
                        None => true,
                    };
                    let square_two = match self.find_piece_by_coords(piece.rank, piece.file - 2) {
                        Some(x) => false,
                        None => true,
                    };
                    if square_one && square_two {coords.push((piece.rank, piece.file - 2))}
                }
                for i in -1..2 {
                    for j in -1..2 {
                        if i != 0 || j != 0 {
                            let same_team = match self.find_piece_by_coords(piece.rank + i, piece.file + j) {
                                Some(x) => {
                                    if x.color == piece.color {
                                        false
                                    }
                                    else {
                                        true
                                    }
                                },
                                None => true,
                            };
                            if same_team {coords.push((piece.rank + i, piece.file + j))}
                        }
                    }
                }
            },
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
        for i in 0..6 {
            for p in self.white_pieces[i].iter() {
                if p.rank == rank && p.file == file { return Some(p); }
            }
    
            for p in self.black_pieces[i].iter() {
                if p.rank == rank && p.file == file { return Some(p); }
            }
        }

        return None;
    }

    // moves a given piece to the specified rank/file
    // eliminates any pieces that exist there, and updates the FEN
    // returns true if move is successfully made
    pub fn make_move(&mut self, piece:&mut Piece, rank:i32, file:i32) -> bool {
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
        let mut ranks = fields[0].split('/').collect::<Vec<&str>>();

        // save these to update 
        let old_rank = piece.rank;
        let old_file = piece.file;
        
        // actually make the move here. Don't forget to increment the halfmove and full move counters (fields[4] and fields[5] respectively)
        piece.rank = rank;
        piece.file = file;

        // replaces old rank char with 1. numbers still need to be collapsed (i.e. 1,2 needs to be collapsed to 3)
        let new_rank = &Board::replace_nth_char(ranks[old_rank as usize], (old_file - 1) as usize, '1')[..];
        ranks[old_rank as usize] = new_rank;

        let new_position = ranks[rank as usize].chars().nth((file - 1) as usize).unwrap();

        if new_position.is_ascii_digit() {
            let num = new_position.to_digit(10).unwrap();
            if num > 1 {
                // needs to split the int into two sides, if the piece isn't moving to the edge of the space the int covers
            }
        } else {
            // capture a piece
        }

        // also needs to handle promotion

        let new_rank_2 = &Board::replace_nth_char(ranks[rank as usize], (old_file - 1) as usize, '1')[..];
        ranks[rank as usize] = new_rank_2;

        // this converts the halfmoves to an int, adds 1, parses back to a String, and then using the shorthand &(value)[..] converts to &str
        let mut halfmoves = &((&mut fields[4]).parse::<i32>().unwrap() + 1).to_string()[..];
        if piece.kind != PieceType::Pawn {
            fields[4] = &mut halfmoves;
        }

        // this converts the fullmoves to an int, adds 1, parses back to a String, and then using the shorthand &(value)[..] converts to &str
        let mut fullmoves = &((&mut fields[5]).parse::<i32>().unwrap() + 1).to_string()[..];
        if piece.color != Color::Black {
            fields[5] = &mut fullmoves;
        }

        // update "to move" value
        if piece.color == Color::White {
            fields[2] = "b";
        } else {
            fields[2] = "w";
        }
    
        // rejoin ranks with '/' delimeter and assign to dereferenced position_section (fields[0])
        let new_position = ranks.join("/");
        fields[0] = new_position.as_str();


        // rejoin FEN fields with a whitespace delimeter
        let fen = fields.join(" ");
        self.set_fen(fen);

        return true;
    }

    // utility to replace the nth character in a &str
    fn replace_nth_char(s:&str, index:usize, newchar:char) -> String {
        s.chars().enumerate().map(|(i,c)| if i == index { newchar } else { c }).collect()
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

    fn get_castle_ability(color:Color, ) -> bool {
        
        return true;
    }
}