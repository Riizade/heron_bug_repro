use bevy::prelude::*;
use lazy_static::lazy_static;

pub const PLAYER_SPEED: f32 = 250.0;

lazy_static! {
    pub static ref TILE_SIZE: Vec2 = Vec2::new(32.0, 32.0);
}
