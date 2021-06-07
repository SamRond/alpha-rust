use alpha_rust::Board;
use alpha_rust::Coordinates;

mod utils;

fn init(fen:String) -> Board {
    utils::set_panic_hook();
    Board::new(fen)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_board_init() {
        let board = crate::init("".to_string());

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
        assert_eq!(board.get_black_pieces()[15].get_position(), alpha_rust::Coordinates { rank: 8, file: 5});
        println!("true");

        print!("Checking if white king is in correct position... ");
        assert_eq!(board.get_white_pieces()[15].get_position(), alpha_rust::Coordinates { rank: 1, file: 5});
        println!("true");

        print!("Checking if e2 pawn is in correct position... ");
        assert_eq!(board.get_white_pieces()[4].get_position(), alpha_rust::Coordinates { rank: 2, file: 5});
        println!("true");

        print!("Checking if e7 pawn is in correct position... ");
        assert_eq!(board.get_black_pieces()[4].get_position(), alpha_rust::Coordinates { rank: 7, file: 5});
        println!("true");

        print!("\n\n");
    }

    #[test]
    fn test_board_init_with_fen() {
        let mut board = crate::init("rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2".to_string());

        print!("Checking if FEN is set to correct value... ");
        assert_eq!(board.get_fen(), "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2");
        println!("true");

        print!("Checking if white e pawn is in correct position... ");
        assert_eq!(board.get_white_pieces()[0].get_position(), alpha_rust::Coordinates { rank: 4, file: 5});
        println!("true");

        print!("Checking if black c pawn is in correct position... ");
        assert_eq!(board.get_black_pieces()[7].get_position(), alpha_rust::Coordinates { rank: 5, file: 3});
        println!("true");

        print!("Checking if white knight is in correct position... ");
        assert_eq!(board.get_white_pieces()[10].get_position(), alpha_rust::Coordinates { rank: 3, file: 6});
        println!("true");
    }

    #[test]
    fn test_pawn_move() {
        let mut board = crate::init("".to_string());

        print!("\n\n");

        // make move 1. e4
        print!("Checking if pawn move is successful... ");
        let pawn = &board.get_white_pieces()[4];

        let mv = board.make_move(pawn, 4, 5);
        assert!(mv);

        println!("true");

        print!("\n\n");
    }
}