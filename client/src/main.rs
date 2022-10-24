#[allow(unused_imports)]
use std::{
    io::{self, Read, Write},
    net::TcpStream,
};

pub mod gui;
use macroquad::prelude::*;
use models::Player;
pub use models::{logic, Board, PieceType, Position};
use std::collections::HashSet;

#[macroquad::main("Chess")]
async fn main() {
    let piece_textures = gui::create_textures().await;
    let mut g = logic::Game {
        board: Board::new(),
        starting_player: Player::White,
        current_player: Player::White,
        in_check: None,
    };
    let mut storage = EventStorage{
        event_type: Event::Waiting,
        old_pos: Position{x: 9, y: 9},
        moves: None,
        new_pos: Position{x: 9, y: 9},
    };

    loop {
        gui::draw_chessboard();
        // let mut h = HashSet::new();
        // h.insert(Position { x: 0, y: 5 });
        // h.insert(Position { x: 3, y: 6 });
        // h.insert(Position { x: 2, y: 5 });
        if let Some(pos) = gui::get_click() {
            println!("{:?}", pos);
            // println!("{:?}", storage);
            match storage.event_type {
                Event::Waiting => {
                    if pos != storage.new_pos{
                        storage.event_type = Event::StoringLocation;
                        storage.moves = Some(g.get_moves(pos));
                        storage.old_pos = pos;
                    }
                }
                Event::StoringLocation => {
                    if let Some(moves) = &storage.moves{
                        if moves.contains(&pos) {
                            g.move_piece(storage.old_pos, pos);
                            storage.new_pos = pos;
                            storage.moves = None;
                            storage.event_type = Event::Waiting;
                        }
                        else{
                            storage.moves = Some(g.get_moves(pos));
                            storage.old_pos = pos;
                        }
                    }
                }
            }
        }
        if let Some(m) = &storage.moves{
            gui::draw_moves(storage.old_pos, m);
        }
        for piece in &mut g.board {
            if let Some(piece) = piece {
                gui::draw_piece(&piece, &piece_textures);
            }
        }
        next_frame().await;
    }
}

#[derive(Debug)]
struct EventStorage{
    event_type: Event,
    old_pos:Position,
    moves: Option<HashSet<Position>>,
    new_pos: Position,
}
#[derive(Debug)]
enum Event {
    Waiting,
    StoringLocation,
}
