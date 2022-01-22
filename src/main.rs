use rltk::{RltkBuilder, GameState, Rltk, RGB, VirtualKeyCode, Point};
use specs::{prelude::*, saveload::{SimpleMarker, SimpleMarkerAllocator}, };
use std::cmp::{max, min};
use specs_derive::Component;
use crate::{
    player::{player_input, check_if_dead},
    map::{Map,TileType, draw_map},
    components::{Viewshed,Monster, Name,
                 BlocksTile, CombatStats,
                 WantsToMelee, SufferDamage,
                 Item, ProvidesHealing,
                 InBackpack, WantsToPickupItem,
                 WantsToUseItem, WantsToDropItem,
                 Consumable, Ranged, InflictsDamage,
                 AreaOfEffect, WantsToExplode, Protects, Renderable, Position, Player,
    },
    visibility_system::VisibilitySystem,
    Monster_ai_system::MonsterAI,
    map_indexing_system::MapIndexingSystem,
    damage_system::DamageSystem,
    melee_combat_system::MeleeCombatSystem,
    inventory_system::{ItemCollectionSystem, ItemUseSystem, ItemDropSystem},
    set_explosives::SetExplosiveSystem,
};
use crate::components::{SerializeMe, SerializationHelper};

mod map;
mod player;
mod rect;
mod components;
mod visibility_system;
mod Monster_ai_system;
mod map_indexing_system;
mod melee_combat_system;
mod damage_system;
mod gui;
mod gamelog;
mod spawner;
mod inventory_system;
mod set_explosives;
mod saveload_system;


#[derive(PartialEq, Copy, Clone)]
pub enum RunState { AwaitingInput,
    PreRun,
    PlayerTurn,
    MonsterTurn,
    ShowInventory,
    ShowDropItem,
    ShowTargeting { range: i32, item: Entity},
    MainMenu { menu_selection: gui::MainMenuSelection },
    SaveGame,
    Dead,
    NextLevel,
}

pub struct State {
    pub ecs: World,
}


impl GameState for State {
    fn tick(&mut self, mut ctx : &mut Rltk) {
        let mut newrunstate;
        {
            let runstate = self.ecs.fetch::<RunState>();
            newrunstate = *runstate
        }
        ctx.cls();

        match newrunstate {
            RunState::MainMenu {..} => {}
            _ => {
                draw_map(&self.ecs, ctx);

                {
                    let positions = self.ecs.read_storage::<Position>();
                    let renderables = self.ecs.read_storage::<Renderable>();
                    let map = self.ecs.fetch::<Map>();
                    let mut data = (&positions, &renderables).join().collect::<Vec<_>>();
                    data.sort_by(|&a, &b| b.1.render_order.cmp(&a.1.render_order));
                    for (pos, render) in data.iter() {
                        let idx = map.xy_idx(pos.x, pos.y);
                        if map.visible_tiles[idx] {
                            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
                        }
                    }
                    gui::draw_ui(&self.ecs, ctx);
                }
            }
        }

        let mut newrunstate;
        {
            let runstate = self.ecs.fetch::<RunState>();
            newrunstate = *runstate;
        }
        match newrunstate {
            RunState::PreRun => {
                self.run_systems();
                newrunstate = RunState::AwaitingInput;
            }
            RunState::AwaitingInput => {
                if check_if_dead(self){
                    newrunstate = RunState::Dead
                }
                else{
                newrunstate = player_input(self, ctx);
                }
            }
            RunState::PlayerTurn => {
                self.run_systems();
                newrunstate = RunState::MonsterTurn;
            }
            RunState::MonsterTurn => {
                self.run_systems();
                newrunstate = RunState::AwaitingInput;
            }
            RunState::ShowInventory => {
                let result = gui::show_inventory(self, ctx);
                match result.0 {
                    gui::ItemMenuResult::Cancel => newrunstate = RunState::AwaitingInput,
                    gui::ItemMenuResult::NoResponse => {},
                    gui::ItemMenuResult::Selected => {
                        let item_entity = result.1.unwrap();
                        let is_ranged = self.ecs.read_storage::<Ranged>();
                        let is_item_ranged = is_ranged.get(item_entity);
                        if let Some(is_item_ranged) = is_item_ranged {
                            newrunstate = RunState::ShowTargeting { range: is_item_ranged.range, item: item_entity};
                        }
                        else {
                            let mut intent = self.ecs.write_storage::<WantsToUseItem>();
                            intent.insert(*self.ecs.fetch::<Entity>(), WantsToUseItem { item: item_entity, target: None }).expect("Unable to insert intent");
                            newrunstate = RunState::PlayerTurn;
                        }
                    }

                }
                }
            RunState::ShowTargeting {range, item} => {
                let result = gui::ranged_target(self,ctx, range);
                match result.0 {
                    gui::ItemMenuResult::Cancel => newrunstate = RunState::AwaitingInput,
                    gui::ItemMenuResult::NoResponse => {},
                    gui::ItemMenuResult::Selected => {
                        let mut intent = self.ecs.write_storage::<WantsToUseItem>();
                        intent.insert(*self.ecs.fetch::<Entity>(), WantsToUseItem{ item, target: result.1}).expect("unable to insert intent");
                        newrunstate = RunState::PlayerTurn
                    }
                }
            }
            RunState::ShowDropItem => {
                let result = gui::drop_item_menu(self, ctx);
                match result.0 {
                    gui::ItemMenuResult::Cancel => newrunstate = RunState::AwaitingInput,
                    gui::ItemMenuResult::NoResponse => {},
                    gui::ItemMenuResult::Selected => {
                        let item_entity = result.1.unwrap();
                        let mut intent = self.ecs.write_storage::<WantsToDropItem>();
                        intent.insert(*self.ecs.fetch::<Entity>(), WantsToDropItem{ item: item_entity}).expect("Unable to insert intent");
                        newrunstate = RunState::PlayerTurn;
                    }

                }
            }
            RunState::MainMenu {..} => {
                let result = gui::main_menu(self,ctx);
                match result {
                    gui::MainMenuResult::NoSelection { selected } => newrunstate = RunState::MainMenu { menu_selection: selected },
                    gui::MainMenuResult::Selected { selected } => {
                    match selected {
                        gui::MainMenuSelection::NewGame => newrunstate = RunState::PreRun,
                        gui::MainMenuSelection::LoadGame => {
                            saveload_system::load_game(&mut self.ecs);
                            newrunstate = RunState::AwaitingInput;
                        },
                        gui::MainMenuSelection::Quit => { ::std::process::exit(0); }
                    }
                    }
                }
            }
            RunState::SaveGame => {
                saveload_system::save_game(&mut self.ecs);
                newrunstate = RunState::MainMenu {menu_selection: gui::MainMenuSelection::LoadGame};
            }
            RunState::Dead => {
                //saveload_system::save_game(&mut self.ecs);
                self.ecs = setup_game_state("U died :( try again!").ecs;
              //  ctx.print_color_centered(20, RGB::named(rltk::MAGENTA), RGB::named(rltk::BLACK), "U DIED :(");
                newrunstate = RunState::PreRun; // {menu_selection: gui::MainMenuSelection::NewGame};
            }
            RunState::NextLevel => {
                self.goto_next_level();
                newrunstate = RunState::PreRun;
            }

        }
        {
            let mut runwriter = self.ecs.write_resource::<RunState>();
            *runwriter = newrunstate;
        }
        damage_system::delete_the_dead(&mut self.ecs);


    }
}



impl State {
    fn entities_to_remove_on_level_change(&mut self) -> Vec<Entity>{
        let entities = self.ecs.entities();
        let player = self.ecs.read_storage::<Player>();
        let backpack = self.ecs.read_storage::<InBackpack>();
        let player_entity = self.ecs.fetch::<Entity>();

        let mut to_delete: Vec<Entity> = Vec::new();
        for entity in entities.join() {
            let mut should_delete = true;
            let p = player.get(entity);
           // Don't delete the player
            if let Some(_p) = p {
                should_delete = false;
            }
            // Don't delete the player's equipment
            let bp = backpack.get(entity);
            if let Some(bp) = bp {
                if bp.owner == *player_entity {
                    should_delete = false;
                }
            }
            if should_delete {
                to_delete.push(entity)
            }
        }
            to_delete
    }
    
    fn goto_next_level(&mut self){
        // Delete entities that aren't the player or his/her equipment
        let to_delete = self.entities_to_remove_on_level_change();
        for target in to_delete {
            self.ecs.delete_entity(target).expect("unable to delete entity");
        }
        //Build a new map and place the player
        let worldmap;
        {
            let mut worldmap_resource = self.ecs.write_resource::<Map>();
            let current_depth = worldmap_resource.depth;
            *worldmap_resource = Map::new_map_rooms_and_corridors(current_depth+1);
            worldmap = worldmap_resource.clone();
        }

        // spawn baddies
        for room in worldmap.rooms.iter().skip(1) {
            spawner::spawn_room(&mut self.ecs, room);
        }
        // Place player and update resource
        let (player_x, player_y) =worldmap.rooms[0].center();
        let mut player_position = self.ecs.write_resource::<Point>();
        *player_position = Point::new(player_x, player_y);
        let mut position_components = self.ecs.write_storage::<Position>();
        let player_entity = self.ecs.fetch::<Entity>();
        let player_pos_comp = position_components.get_mut(*player_entity);
        if let Some(player_pos_comp) = player_pos_comp {
            player_pos_comp.x = player_x;
            player_pos_comp.y = player_y;
        }
        // mark the players visibilty
        let mut viewshed_components = self.ecs.write_storage::<Viewshed>();
        let vs = viewshed_components.get_mut(*player_entity);
        if let Some(vs) = vs {
            vs.dirty = true;
        }

        // Notify the player and give health
        let mut gamelog = self.ecs.fetch_mut::<gamelog::GameLog>();
        gamelog.entries.push("you descend to the next level, and take a moment to heal".to_string());
        let mut player_health_store = self.ecs.write_storage::<CombatStats>();
        let player_health = player_health_store.get_mut(*player_entity);
        if let Some(player_health) = player_health {
            player_health.hp = i32::max(player_health.hp, player_health.max_hp /2);
        }
    }

    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem{};
        vis.run_now(&self.ecs);
        let mut mob = MonsterAI{};
        mob.run_now(&self.ecs);
        let mut mapindex = MapIndexingSystem {};
        mapindex.run_now(&self.ecs);
        let mut melee_sys = MeleeCombatSystem{};
        melee_sys.run_now(&self.ecs);
        let mut dmg_sys = DamageSystem{};
        dmg_sys.run_now(&self.ecs);
        let mut pickup = ItemCollectionSystem{};
        pickup.run_now(&self.ecs);
        let mut items = ItemUseSystem {};
        items.run_now(&self.ecs);
        let mut drop_items = ItemDropSystem{};
        drop_items.run_now(&self.ecs);
        let mut set_explosives = SetExplosiveSystem{};
        set_explosives.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

fn setup_context(title: &str,)-> Rltk{    
    RltkBuilder::simple80x50()
    .with_title(title)
    .with_dimensions(200,125)
    .build().unwrap()}

fn setup_game_state(log: &str,)-> State{
    let mut gs = State{ ecs: World::new() };

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<Monster>();
    gs.ecs.register::<Name>();
    gs.ecs.register::<BlocksTile>();
    gs.ecs.register::<CombatStats>();
    gs.ecs.register::<WantsToMelee>();
    gs.ecs.register::<SufferDamage>();
    gs.ecs.register::<Item>();
    gs.ecs.register::<ProvidesHealing>();
    gs.ecs.register::<InBackpack>();
    gs.ecs.register::<WantsToPickupItem>();
    gs.ecs.register::<WantsToUseItem>();
    gs.ecs.register::<WantsToDropItem>();
    gs.ecs.register::<Consumable>();
    gs.ecs.register::<Ranged>();
    gs.ecs.register::<InflictsDamage>();
    gs.ecs.register::<AreaOfEffect>();
    gs.ecs.register::<WantsToExplode>();
    gs.ecs.register::<Protects>();
    gs.ecs.register::<SimpleMarker<SerializeMe>>();
    gs.ecs.register::<SerializationHelper>();

    gs.ecs.insert(SimpleMarkerAllocator::<SerializeMe>::new());

    let map = Map::new_map_rooms_and_corridors(1);
    let (player_x, player_y) = map.rooms[0].center();

  let player_entity =  spawner::player(&mut gs.ecs, player_x, player_y);
    gs.ecs.insert(rltk::RandomNumberGenerator::new());
    for room in map.rooms.iter().skip(1){
        spawner::spawn_room(&mut gs.ecs, room);
    }

    gs.ecs.insert(player_entity);
    gs.ecs.insert(map);
    gs.ecs.insert(Point::new(player_x,player_y));
    gs.ecs.insert(RunState::PreRun);
    gs.ecs.insert(RunState::MainMenu{ menu_selection: gui::MainMenuSelection::NewGame });
    gs.ecs.insert(gamelog::GameLog{ entries: vec![log.to_string()]});
    gs
}


fn main() -> rltk::BError {
   let mut context = setup_context("Shanes Time Tunnel");
   context.with_post_scanlines(true);
   let mut gs = setup_game_state("Welcome to Shanes Time Tunnel, inspired by rltk");

    rltk::main_loop(context, gs)
 }
