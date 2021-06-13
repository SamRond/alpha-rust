mod utils;


#[cfg(test)]
mod tests {
    use alpha_rust::Board;

    use std::sync::Once;

    static INIT: Once = Once::new();

    fn init() {
        INIT.call_once(|| {
            crate::utils::set_panic_hook();    
        });
    }

    #[test]
    fn test_board_init() {
        init();

        let board = Board::new("".to_string());

        print!("\n\n");

        print!("Checking if FEN is set to correct default value... ");
        assert_eq!(board.get_fen(), "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        println!("true");

        print!("Checking if white pieces are correctly initialized... ");
        assert_eq!(board.get_white_pieces()[0].len(), 8); // 8 pawns
        assert_eq!(board.get_white_pieces()[1].len(), 2);
        assert_eq!(board.get_white_pieces()[2].len(), 2);
        assert_eq!(board.get_white_pieces()[3].len(), 2);
        assert_eq!(board.get_white_pieces()[4].len(), 1);
        assert_eq!(board.get_white_pieces()[5].len(), 1);
        println!("true");

        print!("Checking if black pieces are correctly initialized... ");
        assert_eq!(board.get_black_pieces()[0].len(), 8); // 8 pawns
        assert_eq!(board.get_black_pieces()[1].len(), 2);
        assert_eq!(board.get_black_pieces()[2].len(), 2);
        assert_eq!(board.get_black_pieces()[3].len(), 2);
        assert_eq!(board.get_black_pieces()[4].len(), 1);
        assert_eq!(board.get_black_pieces()[5].len(), 1);
        println!("true");

        print!("Checking if black king is in correct position... ");
        assert_eq!(board.get_black_pieces()[5][0].get_position(), alpha_rust::Coordinates { rank: 8, file: 5});
        println!("true");

        print!("Checking if white king is in correct position... ");
        assert_eq!(board.get_white_pieces()[5][0].get_position(), alpha_rust::Coordinates { rank: 1, file: 5});
        println!("true");

        print!("Checking if e2 pawn is in correct position... ");
        assert_eq!(board.get_white_pieces()[0][4].get_position(), alpha_rust::Coordinates { rank: 2, file: 5});
        println!("true");

        print!("Checking if e7 pawn is in correct position... ");
        assert_eq!(board.get_black_pieces()[0][4].get_position(), alpha_rust::Coordinates { rank: 7, file: 5});
        println!("true");

        print!("\n\n");
    }

    #[test]
    fn test_board_init_with_fen() {
        init();
        
        let board = Board::new("rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2".to_string());

        print!("Checking if FEN is set to correct value... ");
        assert_eq!(board.get_fen(), "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2");
        println!("true");

        print!("Checking if white e pawn is in correct position... ");
        assert_eq!(board.get_white_pieces()[0][0].get_position(), alpha_rust::Coordinates { rank: 4, file: 5});
        println!("true");

        print!("Checking if black c pawn is in correct position... ");
        assert_eq!(board.get_black_pieces()[0][7].get_position(), alpha_rust::Coordinates { rank: 5, file: 3});
        println!("true");

        print!("Checking if white knight is in correct position... ");
        assert_eq!(board.get_white_pieces()[1][0].get_position(), alpha_rust::Coordinates { rank: 3, file: 6});
        println!("true");
    }

    #[test]
    fn test_pawn_move() {
        init();
        
        let mut board = Board::new("".to_string());

        print!("\n\n");

        // make move 1. e4
        print!("Checking if pawn move 1. e4 is successful... ");
        let pawn = &board.get_white_pieces()[0][4];

        let mv = board.make_move(pawn, 4, 5);
        assert!(mv);
        assert_eq!(board.find_piece_by_coords(4, 5).unwrap(), pawn);
        println!("true");
    }

    #[test]
    fn test_knight_move() {
        let mut board = Board::new("".to_string());

        print!("\n\n");

        // make move 1. Nf3
        print!("Checking if knight move is successful... ");

        
        let knight = &board.get_white_pieces()[1][2];

        let mv = board.make_move(knight, 3, 6);
        assert!(mv);

        let res = board.find_piece_by_coords(3, 6).unwrap();

        assert_eq!(res.get_position(), board.get_white_pieces()[0][4].get_position());

        println!("true");

        print!("\n\n");
    }

    #[test]
    fn test_for_in_check() {
        let mut board = Board::new("".to_string());

        print!("\n\n");

        //make moves 1. e4 f5 2. Qh5+
        let w_pawn = &board.get_white_pieces()[0][4];
        board.make_move(w_pawn, 4, 5);
        let b_pawn = &board.get_black_pieces()[0][5];
        board.make_move(b_pawn, 5, 6);
        let w_queen = &board.get_white_pieces()[4][0];
        board.make_move(w_queen, 5, 8);

        //check if board state includes checks
        //assert!(board.in_check());
    }
}