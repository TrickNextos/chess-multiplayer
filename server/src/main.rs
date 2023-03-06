use std::collections::{HashMap, HashSet};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader, ReadHalf, WriteHalf},
    net::{TcpListener, TcpStream},
    sync::{mpsc, oneshot},
};

mod lobby;
use lobby::{LobbyHandle, TcpHandle};

mod game_organizer;
use game_organizer::GameOrganizerHandle;

#[tokio::main]
async fn main() {
    println!("Hello world!");

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    let (tx, rx) = mpsc::channel(32);
    GameOrganizerHandle::run(rx);
    let mut lobby = LobbyHandle::new(tx);

    loop {
        let (stream, _addr) = listener.accept().await.unwrap();
        let handle = TcpHandle::new(stream);
        lobby.add_player(handle).await;
    }

    // loop {
    //     let (mut socket1, _addr1) = listener.accept().await.unwrap();
    //     let (mut socket2, _addr2) = listener.accept().await.unwrap();
    //
    //     tokio::spawn(async move {
    //         let (p1_reader, mut p1_writer) = socket1.split();
    //         let (p2_reader, mut p2_writer) = socket2.split();
    //
    //         let mut p1_reader = BufReader::new(p1_reader);
    //         let mut p2_reader = BufReader::new(p2_reader);
    //
    //         let mut line1 = String::new();
    //         let mut line2 = String::new();
    //
    //         loop {
    //             tokio::select! {
    //                 result = p1_reader.read_line(&mut line1) => {
    //                     if result.unwrap() == 0 {
    //                         break;
    //                     }
    //                     p2_writer.write_all(line1.as_bytes()).await.unwrap();
    //                     line1.clear();
    //                 }
    //                 result = p2_reader.read_line(&mut line2) => {
    //                     if result.unwrap() == 0 {
    //                         break;
    //                     }
    //                     p1_writer.write_all(line2.as_bytes()).await.unwrap();
    //                     line2.clear();
    //                 }
    //             }
    //         }
    //     });
    // }
}
