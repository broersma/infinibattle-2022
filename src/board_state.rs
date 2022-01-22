use json;
use std::convert::From;

pub struct BoardState {
    pub board: [u8; 16]
}

impl BoardState {
    pub fn load(board_state_json: json::JsonValue) -> BoardState {
        let mut board_state = BoardState { board: [0; 16]};
        for y in 0..4 {
            for x in 0..4 {
                board_state.board[y*4+x] = board_state_json["gameState"]["board"]["board"][y][x].as_u8().unwrap();
            }
        }
        board_state
    }

    fn calculate_legal_moves(&self, current_player: u8) -> Vec<BoardMove> {
        // TODO: implement me
        vec![BoardMove{ lPiece: [ [2, 0], [2, 1],          [2, 2],     [3, 2]  ], neutralPieces: [[0, 0], [3, 3]]}, 
            BoardMove{ lPiece: [[0,0], [0,0],[0,0],[0,0]], neutralPieces: [[0,0],[0,0]]}]
    }

    fn apply_move(&self, board_move: &BoardMove) -> Self { 
        // TODO: implement me
        BoardState{ board: self.board }
    }

    fn evaluate(&self, current_player: u8) -> f32 {
        let legal_moves = self.calculate_legal_moves(current_player);


        0.0
    }
    
    pub fn calculate_optimal_move(&self, current_player: u8) -> BoardMove {
        let legal_moves = self.calculate_legal_moves(current_player);

        // TODO find optimal move

        legal_moves[0].clone()
    }
}


#[derive(Clone)]
pub struct BoardMove {
    pub lPiece: [[u32; 2]; 4],
    pub neutralPieces: [[u32; 2]; 2]
}