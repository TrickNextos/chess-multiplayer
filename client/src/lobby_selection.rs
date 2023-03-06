use std::{
    net::TcpStream,
    io::Read,
    str,
};
use models::WaitingPlayers;

use macroquad::prelude::*;

const PADDING_RATIO: f32 = 0.1;

fn get_padding() -> f32 {
    let (w, h) = (screen_width(), screen_height());
    if w > h {
        return h * PADDING_RATIO;
    }
    w * PADDING_RATIO
}

pub async fn get_opponent(connection: &mut TcpStream){
    loop {
        let mut response = [0 as u8; 2048];
        let bytes_read = connection.read(&mut response).unwrap();
        let response = str::from_utf8(&response[0..bytes_read]).unwrap();
        println!("{}", response);
        let players: WaitingPlayers = serde_json::from_str(&response.to_owned()).unwrap();    

        println!("{:?}", players);
        clear_background(WHITE);

        let mut i = 0;
        for (id, info) in players{
            draw_rectangle(screen_width() * 0.1, (i * 100 + 5) as f32, screen_width() * 0.8, 95.0, GRAY);
            draw_text(&format!("id: {id:?}, info: {info:?}"), screen_width() * 0.15, (i * 100 + 50) as f32, 10.0, BLACK);
            i += 1;
        }
        next_frame().await;

    };
    
}
