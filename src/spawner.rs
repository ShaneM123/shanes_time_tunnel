
use rltk::{ RGB, RandomNumberGenerator};
use specs::prelude::*;
use super::{CombatStats, Renderable, Name, Viewshed, Monster, BlocksTile};
use crate::rect::Rect;
use crate::map::MAPWIDTH;
use crate::components::{Item, Position, Player, ProvidesHealing, Consumable, Ranged, InflictsDamage, AreaOfEffect, WantsToExplode, Protects, SerializeMe};
use specs::saveload::{MarkedBuilder, SimpleMarker};

const MAX_MONSTERS: i32 = 4;
const MAX_ITEMS: i32 = 2;

pub fn spawn_room(ecs: &mut World, room: &Rect) {
    let mut monster_spawn_points: Vec<usize> = Vec::new();
    let mut item_spwan_points: Vec<usize> = Vec::new();

    //Scope to keep the borrow checker happy
    //TODO: refactor the hell out of this duplicated code
    {
        let mut rng = ecs.write_resource::<RandomNumberGenerator>();
        let num_monsters = rng.roll_dice(1, MAX_MONSTERS + 2 ) - 3;
        let num_items = rng.roll_dice(1, MAX_ITEMS + 2 ) - 3;

        for _i in 0..num_monsters{
            let mut added = false;
            while !added {
                let x = (room.x1 + rng.roll_dice(1, i32::abs(room.x2 - room.x1))) as usize;
                let y = (room.y1 + rng.roll_dice(1, i32::abs(room.y2 - room.y1))) as usize;
                let idx = (y * MAPWIDTH) + x;
                if !monster_spawn_points.contains(&idx){
                    monster_spawn_points.push(idx);
                    added = true;
                }
            }
        }
        for _i in 0..num_items {
            let mut added = false;
            while !added {
                let x = (room.x1 + rng.roll_dice(1, i32::abs(room.x2 - room.x1))) as usize;
                let y = (room.y1 + rng.roll_dice(1, i32::abs(room.y2 - room.y1))) as usize;
                let idx = (y * MAPWIDTH) + x;
                if !item_spwan_points.contains(&idx) {
                    item_spwan_points.push(idx);
                    added = true;
                }
            }
        }
    }
    //spawn the monsters
    for idx in monster_spawn_points.iter(){
        let x = *idx % MAPWIDTH;
        let y = *idx / MAPWIDTH;
        random_monster(ecs, x as i32, y as i32);
    }
    //spawn the items
    for idx in item_spwan_points.iter(){
        let x = *idx % MAPWIDTH;
        let y = *idx / MAPWIDTH;
        random_item(ecs, x as i32, y as i32);
    }
    for idx in item_spwan_points.iter(){
        let x = *idx % MAPWIDTH;
        let y = *idx / MAPWIDTH;
        random_item(ecs, x as i32, y as i32);
    }
}

fn random_item(ecs: &mut World, x: i32, y: i32) {
    let roll: i32;
    {
        let mut rng = ecs.write_resource::<RandomNumberGenerator>();
        roll = rng.roll_dice(1,4);
    }
    match roll {
        1 => { vampire_shield(ecs, x, y) }
        2 => { vampire_shield(ecs, x, y)  }
        3 => { vampire_shield(ecs, x, y) }
        _ => { bomb(ecs, x, y) }
    }
}

/// Spawns the player and returns his/her entity object.
pub fn player(ecs: &mut World, player_x: i32, player_y: i32) -> Entity {
    ecs.create_entity()
        .with(Position{ x: player_x, y: player_y })
        .with(Renderable{
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
            render_order: 0,
        })
        .with(Player{})
        .with(Viewshed{
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .with(Name{name: "Player".to_string()})
        .with(CombatStats{max_hp: 110, hp: 80, defense: 2, power: 5, deflects: 0})
        .build()
}

//Spawn a random monster at a given location
pub fn random_monster(ecs: &mut World, x: i32, y: i32) {
    let roll: i32;
    {
        let mut rng = ecs.write_resource::<RandomNumberGenerator>();
        roll = rng.roll_dice(1,2);
    }
    match roll {
        1 => { orc(ecs, x, y) }
        _ => { goblin(ecs, x, y) }
    }
}
fn orc(ecs: &mut World, x: i32, y: i32){
    monster(ecs, x, y, rltk::to_cp437('o'), "Orc");
}
fn goblin(ecs: &mut World, x: i32, y: i32){
    monster(ecs, x, y, rltk::to_cp437('g'), "Goblin");
}

fn monster<S: ToString>(ecs: &mut World, x: i32, y: i32, glyph: rltk::FontCharType, name: S){
    ecs.create_entity()
        .with(Position{x,y})
        .with(Renderable{
            glyph,
            fg: RGB::named(rltk::RED),
            bg: RGB::named(rltk::BLACK),
            render_order: 1,
        })
        .with(Viewshed{visible_tiles: Vec::new(), range: 8, dirty: true})
        .with(Monster{})
        .with(Name{name: name.to_string() })
        .with(BlocksTile{})
        .with(CombatStats{max_hp: 16, hp: 16, defense: 1, power: 4, deflects: 0})
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

fn health_potion(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437('+'),
            fg: RGB::named(rltk::GREEN),
            bg: RGB::named(rltk::WHITE),
            render_order: 2,
        })
        .with(Name{ name: "Health Potion".to_string()})
        .with(Item{})
        .with(Consumable{})
        .with(ProvidesHealing { heal_amount: 8,})
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

fn magic_missile_scroll(ecs: &mut World, x: i32, y: i32,) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437(')'),
            fg: RGB::named(rltk::CYAN),
            bg: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .with(Name{ name: "Magic Missile Scroll".to_string()})
        .with(Item{})
        .with(Consumable{})
        .with(InflictsDamage{ damage: 10 })
        .with(Ranged { range: 6,})
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

fn fireball_scroll(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437(')'),
            fg: RGB::named(rltk::ORANGE),
            bg: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .with(Name{ name: "Fireball Scroll".to_string()})
        .with(Item{})
        .with(Consumable{})
        .with(InflictsDamage{ damage: 10 })
        .with(Ranged { range: 6,})
        .with(AreaOfEffect{radius: 3})
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

fn bomb(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437('T'),
            fg: RGB::named(rltk::ORANGE1),
            bg: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .with(Name{ name: "Bomb".to_string()})
        .with(Item{})
        .with(Consumable{})
        .with(InflictsDamage{ damage: 10 })
        .with(WantsToExplode{ set: false, timer: 3})
        .with(AreaOfEffect{radius: 3})
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

fn vampire_shield(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437('V'),
            fg: RGB::named(rltk::PURPLE),
            bg: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .with(Name{ name: "Vampire Shield".to_string()})
        .with(Item{})
        .with(Consumable{})
        .with(Protects{ deflections: 3 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}
