
use rltk::{RGB};
use specs::prelude::*;
use crate::{Renderable,player::Player, Position};

/// Spawns the player and returns his/her entity object.
pub fn player(ecs: &mut World, player_x: i32, player_y: i32) -> Entity{
    ecs.create_entity()
        .with(Position{x: player_x, y: player_y})
        .with(Renderable{
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::ANTIQUEWHITE2),
            bg: RGB::named(rltk::GREEN3),
        })
        .with(Player{})
        .build()
}
