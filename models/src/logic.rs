use std::collections::HashSet;
use crate::{Board, Piece, Position, PieceType, Player};

pub struct Game{
    pub board: Board,
    pub starting_player: Player,
    pub in_check: Option<Player>,
    pub current_player: Player,
}

impl Game{
    pub fn move_piece(&mut self, old_pos: Position, new_pos: Position){
        // let mut p = self.board.raw[old_pos.y as usize][old_pos.x as usize];
        // if let Some(mut p) = p {
        //     p.pos = Position { x: new_pos.x, y: new_pos.y };
        //     self.board.raw[old_pos.y as usize][old_pos.x as usize] = None;
        //     self.board.raw[old_pos.y as usize][old_pos.x as usize] = Some(p);
        // }
        self.board.move_piece(old_pos, new_pos);
    }

    fn get_lines_moves(&self, piece: &Piece, moves:&mut HashSet<Position>, direction_list: [(i32, i32); 4]){
        for direction in direction_list{
            let Position{x, y} = piece.pos;
            for j in 1..8{
                let (x, y) = (x+direction.0*j, y+direction.1*j);
                if (0..8).contains(&x) && (0..8).contains(&y){
                    if let Some(p) = self.board.get(x, y){
                        if p.player != piece.player{
                            moves.insert(Position { x, y });
                        }
                        break;
                    }
                    else{
                        moves.insert(Position {x, y});
                    }
                } 
            }
        }
    }

    pub fn get_moves(&self, piece_pos: Position) -> HashSet<Position>{
        println!("moves {piece_pos:?}");
        let mut moves = HashSet::new();
        let piece = match self.board.get(piece_pos.x, piece_pos.y){
            Some(p) => p,
            None => return moves,
        };
        println!("Valid piece {piece_pos:?}");
        match piece.piece_type{
            PieceType::Pawn => {
               let direction = {
                   if piece.player == Player::White{
                       -1
                   }
                   else{
                       1
                   }
               }; 
                if (0..8).contains(&(piece.pos.y + direction)) && self.board.get(piece.pos.x, piece.pos.y + direction).is_none(){
                    moves.insert(Position{x: piece.pos.x, y: piece.pos.y + direction});
                    if (piece.pos.y == 6 && Player::White == piece.player) || (piece.pos.y == 1 && Player::White != piece.player){
                        if self.board.get(piece.pos.x, piece.pos.y + 2*direction).is_none(){
                            moves.insert(Position{x: piece.pos.x, y: piece.pos.y + 2*direction});
                        }
                    } 
                }
                for capture_direction in [1, -1]{
                    if !((0..8).contains(&(piece.pos.y + direction)) && (0..8).contains(&(piece.pos.x + capture_direction))){ continue; }
                    if let Some(p) = self.board.get(piece.pos.x + capture_direction, piece.pos.y + direction){
                        if p.player != piece.player{
                            moves.insert(Position {x: piece.pos.x + capture_direction, y: piece.pos.y + direction});
                        }
                    }
                }
            }
            PieceType::Rook => {
                self.get_lines_moves(&piece, &mut moves, [(0, 1), (0, -1), (1, 0), (-1, 0)]);
            }
            PieceType::Bishop => {
                self.get_lines_moves(&piece, &mut moves, [(1, 1), (1, -1), (-1, 1), (-1, -1)]);
            }
            PieceType::Queen => {
                self.get_lines_moves(&piece, &mut moves, [(1, 1), (1, -1), (-1, 1), (-1, -1)]);
                self.get_lines_moves(&piece, &mut moves, [(0, 1), (0, -1), (1, 0), (-1, 0)]);
            }
            PieceType::Knight => {
                for (cx, cy) in [(-2, -1), (-2, 1), (2, -1), (2, 1), (-1, -2), (-1, 2), (1, -2), (1, 2)]{
                    if !((0..8).contains(&(piece.pos.x + cx)) && (0..8).contains(&(piece.pos.y + cy))){ continue; }
                    if let Some(p) = self.board.get(piece.pos.x + cx, piece.pos.y + cy){
                        if p.player == piece.player {
                            continue;
                        }
                    }
                    moves.insert(Position { x: piece.pos.x + cx, y: piece.pos.y + cy });
                }
            }
            PieceType::King => {
                for (cx, cy) in [(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (1, -1), (-1, 1), (-1, -1)]{
                    if !((0..8).contains(&(piece.pos.x + cx)) && (0..8).contains(&(piece.pos.y + cy))){ continue; }
                    if let Some(p) = self.board.get(piece.pos.x + cx, piece.pos.y + cy){
                        if p.player == piece.player {
                            continue;
                        }
                    }
                    moves.insert(Position { x: piece.pos.x + cx, y: piece.pos.y + cy });
                }
            }
        }
        moves
    }
}


trait BoardMethods{
        fn new() -> Self;
        fn get(&self, x: i32, y: i32) -> Option<Piece>;
}


impl BoardMethods for Board {
    fn get(&self, x: i32, y: i32) -> Option<Piece>{
        self.raw[y as usize][x as usize]
    }

    fn new() -> Board {
        Board {
            raw: [
                [
                    Some(Piece {
                        pos: Position { x: 0, y: 0 },
                        piece_type: PieceType::Rook,
                        player: Player::Black,
                    }),
                    Some(Piece {
                        pos: Position { x: 1, y: 0 },
                        piece_type: PieceType::Knight,
                        player: Player::Black,
                    }),
                    Some(Piece {
                        pos: Position { x: 2, y: 0 },
                        piece_type: PieceType::Bishop,
                        player: Player::Black,
                    }),
                    Some(Piece {
                        pos: Position { x: 3, y: 0 },
                        piece_type: PieceType::Queen,
                        player: Player::Black,
                    }),
                    Some(Piece {
                        pos: Position { x: 4, y: 0 },
                        piece_type: PieceType::King,
                        player: Player::Black,
                    }),
                    Some(Piece {
                        pos: Position { x: 6, y: 0 },
                        piece_type: PieceType::Bishop,
                        player: Player::Black,
                    }),
                    Some(Piece {
                        pos: Position { x: 5, y: 0 },
                        piece_type: PieceType::Knight,
                        player: Player::Black,
                    }),
                    Some(Piece {
                        pos: Position { x: 7, y: 0 },
                        piece_type: PieceType::Rook,
                        player: Player::Black,
                    }),
                ],
                [
                    Some(Piece {
                        pos: Position { x: 0, y: 1 },
                        piece_type: PieceType::Pawn,
                        player: Player::Black,
                    }),
                    Some(Piece {
                        pos: Position { x: 1, y: 1 },
                        piece_type: PieceType::Pawn,
                        player: Player::Black,
                    }),
                    Some(Piece {
                        pos: Position { x: 2, y: 1 },
                        piece_type: PieceType::Pawn,
                        player: Player::Black,
                    }),
                    Some(Piece {
                        pos: Position { x: 3, y: 1 },
                        piece_type: PieceType::Pawn,
                        player: Player::Black,
                    }),
                    Some(Piece {
                        pos: Position { x: 4, y: 1 },
                        piece_type: PieceType::Pawn,
                        player: Player::Black,
                    }),
                    Some(Piece {
                        pos: Position { x: 5, y: 1 },
                        piece_type: PieceType::Pawn,
                        player: Player::Black,
                    }),
                    Some(Piece {
                        pos: Position { x: 6, y: 1 },
                        piece_type: PieceType::Pawn,
                        player: Player::Black,
                    }),
                    Some(Piece {
                        pos: Position { x: 7, y: 1 },
                        piece_type: PieceType::Pawn,
                        player: Player::Black,
                    }),
                ],
                [None; 8],
                [None; 8],
                [None; 8],
                [None; 8],
                [
                    Some(Piece {
                        pos: Position { x: 0, y: 6 },
                        piece_type: PieceType::Pawn,
                        player: Player::White,
                    }),
                    Some(Piece {
                        pos: Position { x: 1, y: 6 },
                        piece_type: PieceType::Pawn,
                        player: Player::White,
                    }),
                    Some(Piece {
                        pos: Position { x: 2, y: 6 },
                        piece_type: PieceType::Pawn,
                        player: Player::White,
                    }),
                    Some(Piece {
                        pos: Position { x: 3, y: 6 },
                        piece_type: PieceType::Pawn,
                        player: Player::White,
                    }),
                    Some(Piece {
                        pos: Position { x: 4, y: 6 },
                        piece_type: PieceType::Pawn,
                        player: Player::White,
                    }),
                    Some(Piece {
                        pos: Position { x: 5, y: 6 },
                        piece_type: PieceType::Pawn,
                        player: Player::White,
                    }),
                    Some(Piece {
                        pos: Position { x: 6, y: 6 },
                        piece_type: PieceType::Pawn,
                        player: Player::White,
                    }),
                    Some(Piece {
                        pos: Position { x: 7, y: 6 },
                        piece_type: PieceType::Pawn,
                        player: Player::White,
                    }),
                ],
                [
                    Some(Piece {
                        pos: Position { x: 0, y: 7 },
                        piece_type: PieceType::Rook,
                        player: Player::White,
                    }),
                    Some(Piece {
                        pos: Position { x: 1, y: 7 },
                        piece_type: PieceType::Knight,
                        player: Player::White,
                    }),
                    Some(Piece {
                        pos: Position { x: 2, y: 7 },
                        piece_type: PieceType::Bishop,
                        player: Player::White,
                    }),
                    Some(Piece {
                        pos: Position { x: 3, y: 7 },
                        piece_type: PieceType::Queen,
                        player: Player::White,
                    }),
                    Some(Piece {
                        pos: Position { x: 4, y: 7 },
                        piece_type: PieceType::King,
                        player: Player::White,
                    }),
                    Some(Piece {
                        pos: Position { x: 6, y: 7 },
                        piece_type: PieceType::Bishop,
                        player: Player::White,
                    }),
                    Some(Piece {
                        pos: Position { x: 5, y: 7 },
                        piece_type: PieceType::Knight,
                        player: Player::White,
                    }),
                    Some(Piece {
                        pos: Position { x: 7, y: 7 },
                        piece_type: PieceType::Rook,
                        player: Player::White,
                    }),
                ],
            ],
            count: 0,
        }
    }
}

