use rltk::{RGB, GameState, BLUE3, BTerm, Point};
use specs::prelude::*;
use crate::player::{Player, player_input};
use specs_derive::Component;

mod spawner;
mod player;

#[derive(Component)]
pub struct Position {
    x: i32,
    y: i32,
}
#[derive(Component)]
pub struct Renderable {
    glyph: rltk::FontCharType,
    fg: RGB,
    bg: RGB,
}

#[derive(PartialEq, Copy, Clone)]
pub enum RunState { PlayerTurn, }

struct State {
    pub ecs: World,
}

impl GameState for State{
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        let mut newrunstate;
        {
            let runstate = self.ecs.fetch::<RunState>();
            newrunstate = *runstate;
        }
        match newrunstate{
            RunState::PlayerTurn => {
                player_input(self, ctx);
            }
        }
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join(){
            ctx.set(pos.x,pos.y, render.fg, render.bg, render.glyph);
        }
    }
}
impl State {
    fn run_systems(&mut self){
    }

}

fn main() -> rltk::BError{
    use rltk::RltkBuilder;
    let mut context = RltkBuilder::simple80x50()
        .with_title("Shane's First Roguelike")
        .build()?;
    context.with_post_scanlines(true);
    context.screen_burn_color(RGB::from(BLUE3));

    let mut gs = State{ ecs: World::new()};

    gs.ecs.register::<Player>();
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();

    let (player_x, player_y) = (40,25);

    let player_entity = spawner::player(&mut gs.ecs, player_x, player_y);

    gs.ecs.insert(player_entity);
    gs.ecs.insert(Point::new(player_x,player_y));
    gs.ecs.insert(RunState::PlayerTurn);


    rltk::main_loop(context, gs)
}
