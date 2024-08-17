use pumpkin_macros::packet;
use serde::Serialize;

use crate::{position::WorldPosition, VarInt};

#[derive(Serialize)]
#[packet(0x09)]
pub struct CBlockUpdate {
    location: WorldPosition,
    block_id: VarInt,
}