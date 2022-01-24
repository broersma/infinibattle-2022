use json;

pub struct BoardState {
    board: [u8; 16],
    current_player: u8,
    scores: (i32, i32)
}

impl BoardState {
    fn new() -> BoardState {
        BoardState { board: [4, 0, 0, 0, 1, 2, 2, 2, 1, 1, 1, 2, 0, 0, 0, 4], current_player: 1, scores: (0,0) }
    }

    pub fn load(board_state_json: json::JsonValue) -> BoardState {
        let scores = (board_state_json["gameState"]["scorePlayer0"].as_i32().unwrap(), board_state_json["gameState"]["scorePlayer1"].as_i32().unwrap());
        let current_player = board_state_json["player"].as_u8().unwrap();
        let mut board_state = BoardState { board: [0; 16], current_player, scores };
        for y in 0..4 {
            for x in 0..4 {
                board_state.board[y*4+x] = board_state_json["gameState"]["board"]["board"][y][x].as_u8().unwrap();
            }
        }
        board_state
    }

    fn get_value_at_pos(&self, x: i32, y: i32) -> Option<u8> {
        if x < 0 || x >= 4 || y < 0 || y >= 4 {
            None
        } else {
            Some(self.board[(y*4+x) as usize])
        }
    }

    fn get_neutral_piece_positions(&self) -> Vec<[i32; 2]>
    {
        self.get_positions_with_value(4)
    }

    fn get_positions_with_value(&self, value: u8) -> Vec<[i32; 2]>
    {
        let mut positions = vec![];
        for y in 0..4i32 {
            for x in 0..4i32 {
                if self.get_value_at_pos(x,y) == Some(value) {
                    positions.push([x,y]);
                }
            }
        }
        positions
    }

    fn current_player(&self) -> u8 {
        self.current_player
    }

    fn other_player(&self) -> u8 {
        3 - self.current_player
    }

    fn calculate_legal_moves(&self) -> Vec<BoardMove> {

        let orientations: Vec<Vec<(i32, i32)>> = vec![
        //023
        //1
        vec![(0,0), (0,1), (1,0), (2,0)],
        //10
        // 2
        // 3
        vec![(0,0), (-1,0), (0,1), (0,2)],
        //  1
        //320
        vec![(0,0), (0,-1), (-1,0), (-2,0)],
        // 3
        // 2
        // 01
        vec![(0,0), (1,0), (0,-1), (0,-2)],
        //1
        //023
        vec![(0,0), (0,-1), (1,0), (2,0)],
        // 3
        // 2
        //10
        vec![(0,0), (-1,0), (0,-1), (0,-2)],
        //320
        //  1
        vec![(0,0), (0,1), (-1,0), (-2,0)],
        // 01
        // 2
        // 3
        vec![(0,0), (1,0), (0,1), (0,2)]
        ];

        let mut legal_moves = vec![];

        let move_template = BoardMove{ lPiece: [[0,0], [0,0],[0,0],[0,0]], neutralPieces: [[0,0],[0,0]]};

        for y in 0..4i32 {
            for x in 0..4i32 {
                if self.get_value_at_pos(x, y) == Some(0) || self.get_value_at_pos(x, y) == Some(self.current_player()) {

                    for orientation in &orientations {
                        // TODO: clean up, need iterators and correct types...
                        let mut orientation_fits = true;
                        for coord in orientation {
                            let dx = coord.0 + x as i32;
                            let dy = coord.1 + y as i32;
                            let value = self.get_value_at_pos(dx, dy);
                            if value == None || value == Some(4) || value == Some(self.other_player()) {
                                orientation_fits = false;
                            }
                        }

                        if orientation_fits {
                            let mut new_move = move_template.clone();
                            for (i, coord) in orientation.iter().enumerate() {
                                let dx = coord.0 + x as i32;
                                let dy = coord.1 + y as i32;
                                new_move.lPiece[i][0] = dx;
                                new_move.lPiece[i][1] = dy;
                            }

                            // filter "no move" of L-piece
                            for [x,y] in new_move.lPiece {
                                if self.get_value_at_pos(x, y) != Some(self.current_player()) {
                                    let neutral_piece_positions = self.get_neutral_piece_positions();
                                    new_move.neutralPieces[0] = neutral_piece_positions[0];
                                    new_move.neutralPieces[1] = neutral_piece_positions[1];

                                    // move without changing neutral pieces
                                    legal_moves.push(new_move.clone());

                                    // add legal neutral piece moves
                                    let temp_board = self.apply_move(&new_move);

                                    for i in 0..2 {
                                        for y in 0..4i32 {
                                            for x in 0..4i32 {
                                                if temp_board.get_value_at_pos(x,y) == Some(0) {
                                                    let mut new_move_with_neutral_piece = new_move.clone();
                                                    new_move_with_neutral_piece.neutralPieces[i] = [x,y];
                                                    legal_moves.push(new_move_with_neutral_piece);
                                                }
                                            }
                                        }
                                    }
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }

        legal_moves
    }

    fn apply_move(&self, board_move: &BoardMove) -> Self {

        let mut new_board = self.board.clone();
        let mut new_scores = self.scores;

        // remove pieces from board
        for y in 0..4 {
            for x in 0..4 {
                if new_board[y*4+x] == self.current_player() || new_board[y*4+x] == 4 {
                    new_board[y*4+x] = 0;
                }
            }
        }

        // replace pieces
        for y in 0..4 {
            for x in 0..4 {
                if board_move.lPiece.iter().any(|[lx,ly]| *lx==x && *ly == y) {
                    new_board[(y*4+x) as usize] = self.current_player();
                }
                if board_move.neutralPieces.iter().any(|[lx,ly]| *lx==x && *ly == y) {
                    new_board[(y*4+x) as usize] = 4;
                }
            }
        }

        // update scores
        if new_board[0] == self.current_player()  ||
           new_board[3] == self.current_player()  ||
           new_board[12] == self.current_player() ||
           new_board[15] == self.current_player() {
            if self.current_player() == 1 {
                new_scores.0 += 1
            } else {
                new_scores.1 += 1
            }
        }

        BoardState{ board: new_board, current_player: if self.current_player() == 1 { 2 } else { 1 }, scores: new_scores }
    }

    fn evaluate(&self, depth: u32, player: u8) -> f32 {
        let legal_moves = self.calculate_legal_moves();

        if depth == 0 || legal_moves.len() == 0 {
            if player == 1 {
                (self.scores.0 - self.scores.1) as f32
            } else {
                (self.scores.1 - self.scores.0) as f32
            }
        } else {

            // IMPORTANT TODO: recurse
            0.0
        }
    }

    fn evaluate_move(&self, board_move: &BoardMove, depth: u32, player: u8) -> f32 {
        let new_board = self.apply_move(&board_move);
        new_board.evaluate(depth, player)
    }

    pub fn calculate_optimal_move(&self, depth: u32) -> Option<BoardMove> {
        self.calculate_legal_moves()
            .into_iter()
            .max_by(|x,y| self.evaluate_move(&x, depth, self.current_player()).partial_cmp(&self.evaluate_move(&y, depth, self.current_player()))
                                                .unwrap_or(std::cmp::Ordering::Equal))
    }
}


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BoardMove {
    pub lPiece: [[i32; 2]; 4],
    pub neutralPieces: [[i32; 2]; 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_is_applied_correctly_no_score() {
        let board_state = BoardState::new();

        let board_move = BoardMove{lPiece: [[0,2], [1,2], [2,2], [2,3]], neutralPieces: [[0,0], [3,0]]};

        let new_state = board_state.apply_move(&board_move);

        assert_eq!(new_state.board, [4, 0, 0, 4, 0, 2, 2, 2, 1, 1, 1, 2, 0, 0, 1, 0]);
        assert_eq!(new_state.current_player, 2);
        assert_eq!(new_state.scores, (0, 0));
    }

    #[test]
    fn move_is_applied_correctly_score() {
        let current_board_state = BoardState::new();

        // move player 1's l-piece downward
        let board_move = BoardMove{lPiece: [[0,2], [0,3], [1,3], [2,3]], neutralPieces: [[0,0], [3,0]]};
        let current_board_state = current_board_state.apply_move(&board_move);

        assert_eq!(current_board_state.board, [4, 0, 0, 4, 0, 2, 2, 2, 1, 0, 0, 2, 1, 1, 1, 0]);
        assert_eq!(current_board_state.current_player, 2);
        assert_eq!(current_board_state.scores, (1, 0));

        // move player 2's l-piece downward
        let board_move = BoardMove{lPiece: [[1,2], [2,2], [3,2], [3,3]], neutralPieces: [[0,0], [3,0]]};
        let current_board_state = current_board_state.apply_move(&board_move);

        assert_eq!(current_board_state.board, [4, 0, 0, 4, 0, 0, 0, 0, 1, 2, 2, 2, 1, 1, 1, 2]);
        assert_eq!(current_board_state.current_player, 1);
        assert_eq!(current_board_state.scores, (1, 1));
    }

    #[test]
    fn losing_state_has_no_more_moves() {
        let current_board_state = BoardState { board: [0, 0, 4, 0,
                                                       2, 2, 2, 0,
                                                       1, 0, 2, 0,
                                                       1, 1, 1, 4],
                                               current_player: 1,
                                               scores: (0,0) };


        assert_eq!(current_board_state.calculate_legal_moves(), vec![]);

        let current_board_state = BoardState { board: [0, 0, 2, 0,
                                                       0, 4, 2, 0,
                                                       4, 1, 2, 2,
                                                       0, 1, 1, 1],
                                               current_player: 1,
                                               scores: (0,0) };

        assert_eq!(current_board_state.calculate_legal_moves(), vec![]);
    }

    #[test]
    fn state_has_moves() {
        let current_board_state = BoardState::new();

        assert_eq!(current_board_state.calculate_legal_moves().len(), 5 * 13);

        assert_eq!(current_board_state.calculate_legal_moves(), vec![BoardMove { lPiece: [[0, 2], [0, 3], [1, 2], [2, 2]], neutralPieces: [[0, 0], [3, 3]] }, BoardMove { lPiece: [[0, 2], [0, 3], [1, 2], [2, 2]], neutralPieces: [[1, 0], [3, 3]] }, BoardMove { lPiece: [[0, 2], [0, 3], [1, 2], [2, 2]], neutralPieces: [[2, 0], [3, 3]] }, BoardMove { lPiece: [[0, 2], [0, 3], [1, 2], [2, 2]], neutralPieces: [[3, 0], [3, 3]] }, BoardMove { lPiece: [[0, 2], [0, 3], [1, 2], [2, 2]], neutralPieces: [[0, 1], [3, 3]] }, BoardMove { lPiece: [[0, 2], [0, 3], [1, 2], [2, 2]], neutralPieces: [[1, 3], [3, 3]] }, BoardMove { lPiece: [[0, 2], [0, 3], [1, 2], [2, 2]], neutralPieces: [[2, 3], [3, 3]] }, BoardMove { lPiece: [[0, 2], [0, 3], [1, 2], [2, 2]], neutralPieces: [[0, 0], [1, 0]] }, BoardMove { lPiece: [[0, 2], [0, 3], [1, 2], [2, 2]], neutralPieces: [[0, 0], [2, 0]] }, BoardMove { lPiece: [[0, 2], [0, 3], [1, 2], [2, 2]], neutralPieces: [[0, 0], [3, 0]] }, BoardMove { lPiece: [[0, 2], [0, 3], [1, 2], [2, 2]], neutralPieces: [[0, 0], [0, 1]] }, BoardMove { lPiece: [[0, 2], [0, 3], [1, 2], [2, 2]], neutralPieces: [[0, 0], [1, 3]] }, BoardMove { lPiece: [[0, 2], [0, 3], [1, 2], [2, 2]], neutralPieces: [[0, 0], [2, 3]] }, BoardMove { lPiece: [[2, 2], [2, 3], [1, 2], [0, 2]], neutralPieces: [[0, 0], [3, 3]] }, BoardMove { lPiece: [[2, 2], [2, 3], [1, 2], [0, 2]], neutralPieces: [[1, 0], [3, 3]] }, BoardMove { lPiece: [[2, 2], [2, 3], [1, 2], [0, 2]], neutralPieces: [[2, 0], [3, 3]] }, BoardMove { lPiece: [[2, 2], [2, 3], [1, 2], [0, 2]], neutralPieces: [[3, 0], [3, 3]] }, BoardMove { lPiece: [[2, 2], [2, 3], [1, 2], [0, 2]], neutralPieces: [[0, 1], [3, 3]] }, BoardMove { lPiece: [[2, 2], [2, 3], [1, 2], [0, 2]], neutralPieces: [[0, 3], [3, 3]] }, BoardMove { lPiece: [[2, 2], [2, 3], [1, 2], [0, 2]], neutralPieces: [[1, 3], [3, 3]] }, BoardMove { lPiece: [[2, 2], [2, 3], [1, 2], [0, 2]], neutralPieces: [[0, 0], [1, 0]] }, BoardMove { lPiece: [[2, 2], [2, 3], [1, 2], [0, 2]], neutralPieces: [[0, 0], [2, 0]] }, BoardMove { lPiece: [[2, 2], [2, 3], [1, 2], [0, 2]], neutralPieces: [[0, 0], [3, 0]] }, BoardMove { lPiece: [[2, 2], [2, 3], [1, 2], [0, 2]], neutralPieces: [[0, 0], [0, 1]] }, BoardMove { lPiece: [[2, 2], [2, 3], [1, 2], [0, 2]], neutralPieces: [[0, 0], [0, 3]] }, BoardMove { lPiece: [[2, 2], [2, 3], [1, 2], [0, 2]], neutralPieces: [[0, 0], [1, 3]] }, BoardMove { lPiece: [[0, 3], [1, 3], [0, 2], [0, 1]], neutralPieces: [[0, 0], [3, 3]] }, BoardMove { lPiece: [[0, 3], [1, 3], [0, 2], [0, 1]], neutralPieces: [[1, 0], [3, 3]] }, BoardMove { lPiece: [[0, 3], [1, 3], [0, 2], [0, 1]], neutralPieces: [[2, 0], [3, 3]] }, BoardMove { lPiece: [[0, 3], [1, 3], [0, 2], [0, 1]], neutralPieces: [[3, 0], [3, 3]] }, BoardMove { lPiece: [[0, 3], [1, 3], [0, 2], [0, 1]], neutralPieces: [[1, 2], [3, 3]] }, BoardMove { lPiece: [[0, 3], [1, 3], [0, 2], [0, 1]], neutralPieces: [[2, 2], [3, 3]] }, BoardMove { lPiece: [[0, 3], [1, 3], [0, 2], [0, 1]], neutralPieces: [[2, 3], [3, 3]] }, BoardMove { lPiece: [[0, 3], [1, 3], [0, 2], [0, 1]], neutralPieces: [[0, 0], [1, 0]] }, BoardMove { lPiece: [[0, 3], [1, 3], [0, 2], [0, 1]], neutralPieces: [[0, 0], [2, 0]] }, BoardMove { lPiece: [[0, 3], [1, 3], [0, 2], [0, 1]], neutralPieces: [[0, 0], [3, 0]] }, BoardMove { lPiece: [[0, 3], [1, 3], [0, 2], [0, 1]], neutralPieces: [[0, 0], [1, 2]] }, BoardMove { lPiece: [[0, 3], [1, 3], [0, 2], [0, 1]], neutralPieces: [[0, 0], [2, 2]] }, BoardMove { lPiece: [[0, 3], [1, 3], [0, 2], [0, 1]], neutralPieces: [[0, 0], [2, 3]] }, BoardMove { lPiece: [[0, 3], [0, 2], [1, 3], [2, 3]], neutralPieces: [[0, 0], [3, 3]] }, BoardMove { lPiece: [[0, 3], [0, 2], [1, 3], [2, 3]], neutralPieces: [[1, 0], [3, 3]] }, BoardMove { lPiece: [[0, 3], [0, 2], [1, 3], [2, 3]], neutralPieces: [[2, 0], [3, 3]] }, BoardMove { lPiece: [[0, 3], [0, 2], [1, 3], [2, 3]], neutralPieces: [[3, 0], [3, 3]] }, BoardMove { lPiece: [[0, 3], [0, 2], [1, 3], [2, 3]], neutralPieces: [[0, 1], [3, 3]] }, BoardMove { lPiece: [[0, 3], [0, 2], [1, 3], [2, 3]], neutralPieces: [[1, 2], [3, 3]] }, BoardMove { lPiece: [[0, 3], [0, 2], [1, 3], [2, 3]], neutralPieces: [[2, 2], [3, 3]] }, BoardMove { lPiece: [[0, 3], [0, 2], [1, 3], [2, 3]], neutralPieces: [[0, 0], [1, 0]] }, BoardMove { lPiece: [[0, 3], [0, 2], [1, 3], [2, 3]], neutralPieces: [[0, 0], [2, 0]] }, BoardMove { lPiece: [[0, 3], [0, 2], [1, 3], [2, 3]], neutralPieces: [[0, 0], [3, 0]] }, BoardMove { lPiece: [[0, 3], [0, 2], [1, 3], [2, 3]], neutralPieces: [[0, 0], [0, 1]] }, BoardMove { lPiece: [[0, 3], [0, 2], [1, 3], [2, 3]], neutralPieces: [[0, 0], [1, 2]] }, BoardMove { lPiece: [[0, 3], [0, 2], [1, 3], [2, 3]], neutralPieces: [[0, 0], [2, 2]] }, BoardMove { lPiece: [[2, 3], [2, 2], [1, 3], [0, 3]], neutralPieces: [[0, 0], [3, 3]] }, BoardMove { lPiece: [[2, 3], [2, 2], [1, 3], [0, 3]], neutralPieces: [[1, 0], [3, 3]] }, BoardMove { lPiece: [[2, 3], [2, 2], [1, 3], [0, 3]], neutralPieces: [[2, 0], [3, 3]] }, BoardMove { lPiece: [[2, 3], [2, 2], [1, 3], [0, 3]], neutralPieces: [[3, 0], [3, 3]] }, BoardMove { lPiece: [[2, 3], [2, 2], [1, 3], [0, 3]], neutralPieces: [[0, 1], [3, 3]] }, BoardMove { lPiece: [[2, 3], [2, 2], [1, 3], [0, 3]], neutralPieces: [[0, 2], [3, 3]] }, BoardMove { lPiece: [[2, 3], [2, 2], [1, 3], [0, 3]], neutralPieces: [[1, 2], [3, 3]] }, BoardMove { lPiece: [[2, 3], [2, 2], [1, 3], [0, 3]], neutralPieces: [[0, 0], [1, 0]] }, BoardMove { lPiece: [[2, 3], [2, 2], [1, 3], [0, 3]], neutralPieces: [[0, 0], [2, 0]] }, BoardMove { lPiece: [[2, 3], [2, 2], [1, 3], [0, 3]], neutralPieces: [[0, 0], [3, 0]] }, BoardMove { lPiece: [[2, 3], [2, 2], [1, 3], [0, 3]], neutralPieces: [[0, 0], [0, 1]] }, BoardMove { lPiece: [[2, 3], [2, 2], [1, 3], [0, 3]], neutralPieces: [[0, 0], [0, 2]] }, BoardMove { lPiece: [[2, 3], [2, 2], [1, 3], [0, 3]], neutralPieces: [[0, 0], [1, 2]] }]);
    }
}