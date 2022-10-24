use macroquad::prelude::*;
use std::collections::{HashMap, HashSet};

extern crate models;
pub use models::{Board, Piece, PieceType, Player, Position};

const PADDING_RATIO: f32 = 0.05;

// #[derive(Hash, PartialEq, Eq)]
// pub enum PieceType {
//     Pawn,
//     Rook,
//     Knight, Bishop,
//     Queen,
//     King,
// }
//
// #[derive(Hash, PartialEq, Eq)]
// pub enum Player {
//     Black,
//     White,
// }

fn get_padding() -> f32 {
    let (w, h) = (screen_width(), screen_height());
    if w > h {
        return h * PADDING_RATIO;
    }
    w * PADDING_RATIO
}

fn maximum_square_len() -> f32 {
    let (w, h) = (screen_width(), screen_height());
    if w > h {
        return h / 8.;
    }
    w / 8.
}
fn get_square_len() -> f32 {
    maximum_square_len() - 2. * get_padding() / 8.
}

pub fn draw_chessboard() {
    let padding = get_padding();
    let square_len = get_square_len();
    clear_background(WHITE);
    for y in 0..8 {
        for x in 0..8 {
            let color = {
                if (x + y) % 2 == 0 {
                    WHITE
                } else {
                    BLACK
                }
            };
            draw_rectangle(
                x as f32 * square_len + padding,
                y as f32 * square_len + padding,
                square_len,
                square_len,
                color,
            );
        }
    }
    draw_rectangle_lines(
        padding,
        padding,
        square_len * 8.,
        square_len * 8.,
        square_len * 0.1,
        BLACK,
    );
}

pub fn draw_moves(chosen_pos: Position, moves: &HashSet<Position>) {
    let square_len = get_square_len();
    let padding = get_padding();
    draw_rectangle(
        chosen_pos.x as f32 * square_len + padding,
        chosen_pos.y as f32 * square_len + padding,
        square_len,
        square_len,
        GREEN,
    );
    for pos in moves.iter() {
        draw_rectangle(
            pos.x as f32 * square_len + padding,
            pos.y as f32 * square_len + padding,
            square_len,
            square_len,
            GRAY,
        );
    }
}

pub fn draw_piece(piece: &Piece, textures: &HashMap<(Player, PieceType), Texture2D>) {
    let texture = textures.get(&(piece.player, piece.piece_type)).unwrap();

    let Position { x, y } = piece.pos;
    let padding = get_padding();
    let square_len = get_square_len();
    draw_texture_ex(
        *texture,
        x as f32 * square_len + padding,
        y as f32 * square_len + padding,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(square_len, square_len)),
            ..Default::default()
        },
    );
}

pub async fn create_textures() -> HashMap<(Player, PieceType), Texture2D> {
    let mut pieces = HashMap::new();
    for color in ["w", "b"] {
        for piece_type in ["p", "r", "n", "b", "q", "k"] {
            let chosen_color = match color {
                "b" => Player::Black,
                "w" => Player::White,
                _ => unreachable!(),
            };
            let chosen_piece = match piece_type {
                "p" => PieceType::Pawn,
                "r" => PieceType::Rook,
                "n" => PieceType::Knight,
                "b" => PieceType::Bishop,
                "q" => PieceType::Queen,
                "k" => PieceType::King,
                _ => unreachable!(),
            };
            let path = format!("./src/res/{}{}.png", color, piece_type);
            let texture = load_texture(path.as_str()).await.unwrap();
            pieces.insert((chosen_color, chosen_piece), texture);
        }
    }
    pieces
}

#[derive(Debug)]
pub struct Cords(pub i32, pub i32);

pub fn get_click() -> Option<Position> {
    if !is_mouse_button_down(MouseButton::Left) {
        return None;
    }

    let (x, y) = mouse_position();
    let transform = |pos: f32| {
        let pos: i32 = ((pos - get_padding()) / get_square_len()).floor() as i32;
        if (0..8).contains(&pos) {
            Some(pos)
        } else {
            None
        }
    };

    match (transform(x), transform(y)) {
        (Some(a), Some(b)) => Some(Position { x: a, y: b }),
        _ => None,
    }
}
