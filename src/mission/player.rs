use serde::{Deserialize, Serialize};

use crate::mission::IceId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub position: IceId,
}

impl Player {
    pub fn new() -> Self {
        Self { position: IceId(0) }
    }
}
