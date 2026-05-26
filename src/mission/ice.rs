use serde::{Deserialize, Serialize};

use crate::{mission::Subroutine, util::Point};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash)]
pub struct IceId(pub u32);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ice {
    pub name: String,
    pub sprite: String,
    pub position: Point,
    pub id: IceId,
    pub subroutines: Vec<Subroutine>,
    pub inputs: Vec<IceId>,
    pub outputs: Vec<IceId>,
}
