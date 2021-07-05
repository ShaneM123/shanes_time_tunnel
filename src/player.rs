
use rltk::{VirtualKeyCode, console, Rltk, Point};
use specs::prelude::*;
use specs_derive::Component;
use crate::{State, Position,RunState};
use std::cmp::{min, max};

#[derive(Component, Debug)]
pub struct Player {}

pub(crate) fn player_input(gs: &mut State, ctx: &mut Rltk) -> RunState {
    match ctx.key {
        None => {} // nothing happened
        Some(key) => match key {
            VirtualKeyCode::A => {
                try_move_player(-1,0, &mut gs.ecs)
            },
            VirtualKeyCode::D => {
                try_move_player(1,0, &mut gs.ecs)
            },
            VirtualKeyCode::S => {
                try_move_player(0,1, &mut gs.ecs)
            },
            VirtualKeyCode::W => {
                try_move_player(0,-1, &mut gs.ecs)
            },
            _ => {},
        }
    }
    return RunState::PlayerTurn
}

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World){
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();

    for ( _player, pos) in (&mut players, &mut positions).join() {
        pos.x = min(79, max(0, pos.x + delta_x));
        pos.y = min(49, max(0, pos.y + delta_y));
    }

}
