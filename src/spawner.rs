
use rltk::{ RGB, RandomNumberGenerator};
use specs::prelude::*;
use super::{CombatStats, Renderable, Name, Viewshed, Monster, BlocksTile};
use crate::rect::Rect;
use crate::map::MAPWIDTH;
use crate::random_table::{RandomTable};
use crate::components::{Item, Position, Player, ProvidesHealing,ProvidesWhack, Consumable, Ranged, InflictsDamage, AreaOfEffect, WantsToExplode, Protects, SerializeMe};
use specs::saveload::{MarkedBuilder, SimpleMarker};
use std::collections::HashMap;

const MAX_MONSTERS: i32 = 4;

pub fn spawn_room(ecs: &mut World, room: &Rect, map_depth: i32) {
    let spawn_table = room_table(map_depth);
    let mut spawn_points: HashMap<usize, String> = HashMap::new();

    {
        let mut rng = ecs.write_resource::<RandomNumberGenerator>();
        let num_spawns= rng.roll_dice(1,MAX_MONSTERS +3) + (map_depth-1) - 3;

        for _i in 0..num_spawns{
            let mut added = false;
            let mut tries = 0;
            while !added && tries < 20{
                let x = (room.x1 + rng.roll_dice(1, i32::abs(room.x2 - room.x1))) as usize;
                let y = (room.y1 + rng.roll_dice(1, i32::abs(room.y2 - room.y1))) as usize;
                let idx = (y * MAPWIDTH) + x;
                if !spawn_points.contains_key(&idx){
                    spawn_points.insert(idx, spawn_table.roll(&mut rng));
                    added = true;
                }
                else {
                    tries +=1;
                }
            }
        }
    }
        for spawn in spawn_points.iter() {
            let x = (*spawn.0 % MAPWIDTH) as i32;
            let y = (*spawn.0 / MAPWIDTH) as i32;

            match spawn.1.as_ref(){
                "Goblin" => goblin(ecs,x,y),
                "Orc" => orc(ecs,x,y),
                "Health Potion" => health_potion(ecs,x,y),
                "Magic Missile Scrol"=> magic_missile_scroll(ecs,x,y),
                "Team Lead Fire Sword" => team_lead_fire_sword(ecs,x,y),
                "Vampire Shield"=> vampire_shield(ecs,x,y),
                _ => {}
            }
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


fn room_table(map_depth: i32) -> RandomTable{
    RandomTable::new()
    .add("Goblin", 10)
    .add("Team Lead Fire Sword", 2+map_depth)
    .add("Orc", 1+ map_depth)
    .add("Health Potion", 6)
    .add("Magic Missile Scroll", 4)
    .add("Fireball Scroll", 2 + map_depth)
    .add("Vampire Shield", 2 + map_depth)
    // TODO: add bombs
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
//TODO: implement
fn _bomb(ecs: &mut World, x: i32, y: i32) {
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


fn team_lead_fire_sword(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437('T'),
            fg: RGB::named(rltk::ORANGE),
            bg: RGB::named(rltk::BLUE),
            render_order: 2,
        })
        .with(Name{ name: "Team Lead Fire Sword".to_string()})
        .with(Item{})
        .with(Consumable{})
        .with(ProvidesWhack{whack_amount: 25 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}
