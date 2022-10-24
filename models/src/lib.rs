pub mod logic;
use std::collections::{HashSet, HashMap};
#[allow(dead_code)]
#[derive(Hash, Debug, PartialEq, Eq, Clone, Copy)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Hash, Debug, PartialEq, Eq, Clone, Copy)]
pub enum Player {
    Black,
    White,
}

#[derive(Clone, Debug, Copy, Hash, PartialEq, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

pub struct Board {
    pub raw: [[Option<Piece>; 8]; 8],
    pub count: usize,
}


impl Board {
    pub fn new() -> Board {
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
                        pos: Position { x: 5, y: 0 },
                        piece_type: PieceType::Bishop,
                        player: Player::Black,
                    }),
                    Some(Piece {
                        pos: Position { x: 6, y: 0 },
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
                        pos: Position { x: 5, y: 7 },
                        piece_type: PieceType::Bishop,
                        player: Player::White,
                    }),
                    Some(Piece {
                        pos: Position { x: 6, y: 7 },
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



impl Iterator for Board {
    type Item = Option<Piece>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count >= 64 {
            self.count = 0;
            return None;
        }
        let res = Some(self.raw[self.count / 8][self.count % 8]);
        self.count += 1;
        res
    }
}
impl Default for Board {
    fn default() -> Self {
        Board::new()
    }
}

#[derive(Clone, Debug, Copy, Hash, PartialEq, Eq)]
pub struct Piece {
    pub pos: Position,
    pub piece_type: PieceType,
    pub player: Player,
}


impl Board {
    fn get(&self, x: i32, y: i32) -> Option<Piece>{
        self.raw[y as usize][x as usize]
    }

    pub fn move_piece(&mut self, old_pos: Position, new_pos: Position){
        let p = self.raw[old_pos.y as usize][old_pos.x as usize];
        if let Some(mut p) = p {
            p.pos = new_pos;
            self.raw[old_pos.y as usize][old_pos.x as usize] = None;
            self.raw[new_pos.y as usize][new_pos.x as usize] = Some(p);
            println!("old {:?}", self.raw[old_pos.y as usize][old_pos.x as usize]);
            println!("new {:?}", self.raw[new_pos.y as usize][new_pos.x as usize]);
        }
    }
}
