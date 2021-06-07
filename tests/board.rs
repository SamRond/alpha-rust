use alpha_rust::Board;

fn init() -> Board {
    Board::new("".to_string())
}

#[cfg(test)]
mod tests {
    use super::Board;

    #[test]
    fn test_board_init() {
        let board = crate::init();

        print!("\n\n");

        print!("Checking if FEN is set to correct default value... ");
        assert_eq!(board.get_fen(), "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        println!("true");

        print!("Checking if white pieces are correctly initialized... ");
        assert_eq!(board.get_white_pieces().len(), 16);
        println!("true");

        print!("Checking if black pieces are correctly initialized... ");
        assert_eq!(board.get_black_pieces().len(), 16);
        println!("true");

        print!("Checking if black king is in correct position... ");
        assert_eq!(board.get_black_pieces()[15].get_position(), (8, 5));
        println!("true");

        print!("Checking if white king is in correct position... ");
        assert_eq!(board.get_white_pieces()[15].get_position(), (1, 5));
        println!("true");

        print!("\n\n");
    }

    #[test]
    fn test_pawn_move() {
        let mut board = crate::init();

        print!("\n\n");

        // make move 1. e4
        print!("Checking if pawn move is successful... ");

        let mv = board.make_move(&board.get_white_pieces()[0], 4, 5);
        assert!(mv);

        println!("true");

        print!("\n\n");
    }
}