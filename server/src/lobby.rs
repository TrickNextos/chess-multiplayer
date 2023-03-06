use models::{PlayerId, lobby_message::CurrentPlayers};
use std::{collections::HashMap, time::Duration};

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
    sync::{mpsc, oneshot},
};

use models::*;


enum AddToLobby {
    AddNewPlayer(TcpHandle),
}

#[derive(Debug)]
enum LobbyCommand {
    RegisterNewPlayer(PlayerId, PlayerInfo),
    RemovePlayer(PlayerId, mpsc::Sender<TcpHandle>),
}

#[derive(Debug)]
enum LobyPlayerAction {
    ChooseOpponent(PlayerId, PlayerId),
    WasChosen(PlayerId, TcpHandle),
}

pub struct LobbyHandle {
    send_to_lobby: mpsc::Sender<AddToLobby>,
}

enum TcpSend {
    Read(String),
    Disconnected,
}

enum TcpRequest {
    Write(String),
}

struct TcpConnection {
    send_to_handle: mpsc::Sender<Option<String>>,
    read_from_hande: mpsc::Receiver<TcpRequest>,
}

async fn run_connection(mut stream: TcpStream, mut con: TcpConnection) {
    let (reader, mut writter) = stream.split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();

    loop {
        tokio::select! {
            result = reader.read_line(&mut line) => {
                if result.is_err() {
                    println!("hi {}", line);
                    line.clear();
                    continue;
                }
                if result.unwrap() == 0 {
                    let _ = con.send_to_handle.send(None).await;
                    println!("Client disconnected");
                    break;
                } // client disconnected

                let _ = con.send_to_handle.send(Some(line.clone())).await;
                line.clear();
            }
            result = con.read_from_hande.recv() => {
                if let Some(TcpRequest::Write(text)) = result {
                    let _ = writter.write_all(text.as_bytes()).await;
                }
            }

        }
    }
}

#[derive(Debug)]
pub struct TcpHandle {
    send_to_stream: mpsc::Sender<TcpRequest>,
    read_from_stream: mpsc::Receiver<Option<String>>,
}

impl TcpHandle {
    pub fn new(stream: TcpStream) -> Self {
        let (tx1, rx1) = mpsc::channel(32);
        let (tx2, rx2) = mpsc::channel(32);
        let con = TcpConnection {
            send_to_handle: tx2,
            read_from_hande: rx1,
        };

        tokio::spawn(run_connection(stream, con));

        Self {
            send_to_stream: tx1,
            read_from_stream: rx2,
        }
    }

    pub async fn read(&mut self) -> Option<String> {
        loop {
            if let Some(res) = self.read_from_stream.recv().await {
                return res;
            }
        }
    }

    pub async fn write(&mut self, text: String) {
        let _ = self.send_to_stream.send(TcpRequest::Write(text)).await;
    }
}

impl LobbyHandle {
    pub fn new(game_organizer_sender: mpsc::Sender<[(PlayerId, TcpHandle); 2]>) -> Self {
        let (tx, rx) = mpsc::channel(32);

        let lobby = Lobby::new(rx, game_organizer_sender);
        tokio::spawn(run_lobby(lobby));

        Self { send_to_lobby: tx }
    }

    pub async fn add_player(&mut self, stream: TcpHandle) {
        let _ = self
            .send_to_lobby
            .send(AddToLobby::AddNewPlayer(stream))
            .await;
    }
}

async fn run_lobby(mut lobby: Lobby) {
    loop {
        tokio::select! {
            result = lobby.handle_communication.recv() => {
                if let Some(AddToLobby::AddNewPlayer(stream)) = result{
                    lobby.add_player(stream).await;
                }
            }
            result = lobby.command_reciever.recv() => {
                println!("wow {:?}", result);
                if let Some(LobyPlayerAction::ChooseOpponent(id1, id2)) = result {
                    let stream1 = lobby.remove_player(id1).await;
                    let stream2 = lobby.remove_player(id2).await;
                    let _ = lobby.game_organizer.send([(id1, stream1), (id2, stream2)]).await;
                    println!("send ids");
                }
            }
        }
    }
}

/// This struct is going to run in a task, called by the "run_lobby". It will be controlled by
/// the LobbyHandle struct. You shouldn't make this class.
struct Lobby {
    waiting_list: WaitingPlayers,
    next_player_id: PlayerId,
    /// communication between lobby and handle
    handle_communication: mpsc::Receiver<AddToLobby>,
    /// recieves commands from lobby tasks
    command_reciever: mpsc::Receiver<LobyPlayerAction>,
    /// only used for creating new lobby tasks
    command_sender: mpsc::Sender<LobyPlayerAction>,
    waiting_handles: Vec<LobbyUserHandle>,
    /// connection to GameOrganizer instance, which creates games from 2 IDs
    game_organizer: mpsc::Sender<[(PlayerId, TcpHandle); 2]>,
}

impl Lobby {
    fn new(
        handle_rx: mpsc::Receiver<AddToLobby>,
        game_organizer: mpsc::Sender<[(PlayerId, TcpHandle); 2]>,
    ) -> Self {
        let (tx, rx) = mpsc::channel(32); // one for Lobby to user
        Self {
            waiting_list: WaitingPlayers::new(),
            next_player_id: PlayerId(0),
            command_reciever: rx,
            waiting_handles: Vec::new(),
            command_sender: tx,
            handle_communication: handle_rx,
            game_organizer,
        }
    }

    async fn remove_player(&mut self, id: PlayerId) -> TcpHandle {
        let (tx, mut rx) = mpsc::channel(32);
        for handle in &self.waiting_handles {
            let _ = handle
                .send_to_user
                .send(LobbyCommand::RemovePlayer(id, tx.clone()))
                .await;
        }

        self.waiting_list.remove(&id);
        let stream = rx.recv().await.unwrap();
        stream
    }

    async fn add_player(&mut self, stream: TcpHandle) {
        // get new id, send message to all waiting handles, insert new handle into waiting list

        loop {
            if !self.waiting_list.contains_key(&self.next_player_id) {
                break;
            }
            self.next_player_id.add();
        }
        let new_player = PlayerInfo {
            username: "".to_owned(),
        };

        for handle in &self.waiting_handles {
            let _ = handle
                .send_to_user
                .send(LobbyCommand::RegisterNewPlayer(
                    self.next_player_id,
                    new_player.clone(),
                ))
                .await;
        }

        self.waiting_list.insert(self.next_player_id, new_player);

        let new_handle = LobbyUserHandle::new(
            self.command_sender.clone(),
            self.next_player_id,
            self.waiting_list.clone(),
            stream,
        );
        self.waiting_handles.push(new_handle);
    }
}

/// struct, that is getting input from a waiting user and sending them updates when a player
/// joins/get removed.
struct LobbyUser {
    send_to_lobby: mpsc::Sender<LobyPlayerAction>,
    id: PlayerId,
    reciever: mpsc::Receiver<LobbyCommand>,
    players: WaitingPlayers,
    stream: Option<TcpHandle>,
}

impl LobbyUser {
    pub fn new(
        id: PlayerId,
        players: WaitingPlayers,
        send_to_lobby: mpsc::Sender<LobyPlayerAction>,
        reciever: mpsc::Receiver<LobbyCommand>,
        stream: TcpHandle,
    ) -> Self {
        Self {
            id,
            players,
            send_to_lobby,
            reciever,
            stream: Some(stream),
        }
    }
}

/// task, that uses LobbyUser struct to run it (its actual running method)
/// should be spawned in a new task
async fn active_lobby_user(mut user: LobbyUser) {
    let mut connection = user.stream.take().unwrap();
    let mut line;

    let _ = connection
            .write(serde_json::to_string(&user.players).unwrap())
            .await;
    println!("sent");
    loop {
        tokio::select! {
            result = connection.read() => {
                if result.is_none() {break}
                line = result.unwrap();
                let id = PlayerId(match line.trim().parse::<u32>(){
                    Ok(id) => id,
                    Err(err) => {
                        println!("{err}");
                        connection.write("type a number".to_owned()).await;
                        line.clear();
                        continue;
                    }
                });

                if user.players.contains_key(&id) && id != user.id{
                    let _ = user.send_to_lobby.send(LobyPlayerAction::ChooseOpponent(user.id, id)).await;
                    connection.write(format!("Found match {}", id.0).to_owned()).await;
                }
                else {
                    connection.write("Wrong number".to_owned()).await;
                }

            }
            result = user.reciever.recv() => {
                    match result.unwrap() {
                        LobbyCommand::RegisterNewPlayer(id, info) => user.players.insert(id, info),
                        LobbyCommand::RemovePlayer(id, tx) => {
                            if user.id == id {
                                println!("sent :)");
                                tx.send(connection).await.unwrap();
                                break;
                            }
                            println!("no sent");
                            user.players.remove(&id)
                        },
                    };
                    let _ = connection
                        .write(serde_json::to_string(&user.players).unwrap())
                        .await;
            }
        }
    }
    println!("Client {} Disconnected", user.id.0);
}

struct LobbyUserHandle {
    send_to_user: mpsc::Sender<LobbyCommand>,
}

impl LobbyUserHandle {
    pub fn new(
        send_to_lobby: mpsc::Sender<LobyPlayerAction>,
        id: PlayerId,
        players: WaitingPlayers,
        stream: TcpHandle,
    ) -> Self {
        let (tx, rx) = mpsc::channel(32);

        let user = LobbyUser::new(id, players, send_to_lobby, rx, stream);

        tokio::spawn(active_lobby_user(user));

        Self { send_to_user: tx }
    }
}
