use serde::{Deserialize, Serialize};

use crate::mission::Ice;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct System {
    pub name: String,
    pub ice: Vec<Ice>,
}
