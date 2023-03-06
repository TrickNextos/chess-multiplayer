use crate::{PlayerId, PlayerInfo};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentPlayers (
    Vec<(PlayerId, PlayerInfo)>,
);