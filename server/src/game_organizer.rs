use models::{IdPair, PlayerId};
use tokio::sync::mpsc;

use crate::lobby::TcpHandle;

pub struct GameOrganizerHandle {}
type Neki = i32;

impl GameOrganizerHandle {
    pub fn run(lobby_reciever: mpsc::Receiver<[(PlayerId, TcpHandle); 2]>) {
        let organizer = GameOrganizer::new(lobby_reciever);
        tokio::spawn(run_organizer(organizer));
    }
}

async fn run_organizer(mut game_organizer: GameOrganizer) {
    println!("happens");
    loop {
        if let Some(ids) = game_organizer.lobby_reciever.recv().await {
            let [(i1, s1), (i2, s2)] = ids;
            let (tx, rx) = mpsc::channel(32);
            let game = Game::new(i1, i2, s1, s2, tx);

            tokio::spawn(run_game(game));
        }
    }
}

struct Game {
    i1: PlayerId,
    i2: PlayerId,
    s1: TcpHandle,
    s2: TcpHandle,
    send_to_organizer: mpsc::Sender<Neki>,
}

impl Game {
    fn new(
        i1: PlayerId,
        i2: PlayerId,
        s1: TcpHandle,
        s2: TcpHandle,
        tx: mpsc::Sender<Neki>,
    ) -> Self {
        Self {
            i1,
            i2,
            s1,
            s2,
            send_to_organizer: tx,
        }
    }
}

async fn run_game(mut game: Game) {
    loop {
        tokio::select! {
            res = game.s1.read() => {
                let res = res.unwrap();
                println!("{}", res);
                game.s2.write(res).await;
            }
            res = game.s2.read() => {
                let res = res.unwrap();
                println!("{}", res);
                game.s1.write(res).await;
            }
        }
    }
}

struct GameOrganizer {
    lobby_reciever: mpsc::Receiver<[(PlayerId, TcpHandle); 2]>,
    games: Vec<i32>,
}

impl GameOrganizer {
    fn new(lobby_reciever: mpsc::Receiver<[(PlayerId, TcpHandle); 2]>) -> Self {
        Self {
            lobby_reciever,
            games: Vec::new(),
        }
    }
}
