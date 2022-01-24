
use specs::prelude::*;
use super::{WantsToPickupItem, Name, InBackpack,  gamelog::GameLog};
use crate::components::{ProvidesHealing,ProvidesWhack, CombatStats, Position, WantsToUseItem, WantsToDropItem, Consumable, SufferDamage, InflictsDamage, AreaOfEffect, WantsToExplode, Protects};
use crate::map::Map;

pub struct ItemCollectionSystem {}

impl<'a>System<'a> for ItemCollectionSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadExpect<'a, Entity>,
        WriteExpect<'a, GameLog>,
        WriteStorage<'a,WantsToPickupItem>,
        WriteStorage<'a,Position>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, InBackpack>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (player_entity, mut gamelog, mut wants_pickup, mut positions, names, mut backpack ) = data;
        for pickup in wants_pickup.join() {
            positions.remove(pickup.item);
            backpack.insert(pickup.item, InBackpack { owner: pickup.collected_by }).expect("Unable to insert backpack entry");

            if pickup.collected_by == *player_entity {
                gamelog.entries.push(format!("You pick up the {}.", names.get(pickup.item).unwrap().name));
            }
        }
        wants_pickup.clear();
    }
}


pub struct ItemUseSystem {}

impl<'a> System<'a> for ItemUseSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadExpect<'a, Entity>,
        WriteExpect<'a, GameLog>,
        Entities<'a>,
        ReadStorage<'a, Consumable>,
        WriteStorage<'a, WantsToUseItem>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, ProvidesHealing>,
        WriteStorage<'a,CombatStats>,
        ReadStorage<'a, InflictsDamage>,
        ReadExpect<'a, Map>,
        WriteStorage<'a, SufferDamage>,
        ReadStorage<'a, AreaOfEffect>,
        ReadStorage<'a, Protects>,
        ReadStorage<'a, ProvidesWhack>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (player_entity,
            mut gamelog,
            entities,
            consumables,
            mut wants_to_use,
            names,
            healing,
            mut combat_stats,
            inflict_damage,
        map,
            mut suffer_damage,
        aoe,
        protects,whack) = data;



        for (entity, use_item) in (&entities, &wants_to_use ).join() {

            let mut used_item = true;

            let mut targets: Vec<Entity> = Vec::new();
            match use_item.target {
                None => targets.push(*player_entity),
                Some(target) => {
                    let area_effect = aoe.get(use_item.item);
                    match area_effect {
                        None => {
                            let idx = map.xy_idx(target.x, target.y);
                            for mob in map.tile_content[idx].iter(){
                                targets.push(*mob)
                            }
                        }
                        Some(area_effect) => {
                            let mut blast_tiles = rltk::field_of_view(target, area_effect.radius,&*map);
                            blast_tiles.retain(|p| p.x > 0 && p.x < map.width-1 && p.y > 0 && p.y < map.height -1 );
                            for tile_idx in blast_tiles.iter() {
                                let idx = map.xy_idx(tile_idx.x,tile_idx.y);
                                for mob in map.tile_content[idx].iter() {
                                    targets.push(*mob);
                                }
                            }
                        }
                    }
                }
            }
            let item_heals = healing.get(use_item.item);
            match item_heals {
                None => {},
                Some(healer) => {
                    used_item = false;
                    for target in targets.iter() {
                        let stats = combat_stats.get_mut(*target);
                        if let Some(stats)= stats {
                            stats.hp = i32::min(stats.max_hp, stats.hp + healer.heal_amount);
                            if entity == *player_entity {
                                gamelog.entries.push(format!("You drink the {}, healing {} hp", names.get(use_item.item).unwrap().name, healer.heal_amount))
                            }
                            let consumable = consumables.get(use_item.item);
                            match consumable {
                                None => {},
                                Some(_) => {
                                    entities.delete(use_item.item).expect("Delete failed");
                                }
                            }
                        }
                        used_item = true;
                    }
                }
            }
            let item_whacks = whack.get(use_item.item);
            match item_whacks {
                None => {},
                Some(whack) => {
                    used_item = false;
                    for target in targets.iter() {
                        let stats = combat_stats.get_mut(*target);
                        if let Some(stats)= stats {
                            stats.power = i32::max(stats.power, stats.power + whack.whack_amount);
                            if entity == *player_entity {
                                gamelog.entries.push(format!("You bound by fire to the {}, the power of {} flows through", names.get(use_item.item).unwrap().name, whack.whack_amount))
                            }
                            let consumable = consumables.get(use_item.item);
                            match consumable {
                                None => {},
                                Some(_) => {
                                    entities.delete(use_item.item).expect("Delete failed");
                                }
                            }
                        }
                        used_item = true;
                    }
                }
            }
            let item_damages = inflict_damage.get(use_item.item);
            match item_damages {
                None => {},
                Some(damage) => {
                    let target_point = use_item.target.unwrap();
                    let idx = map.xy_idx(target_point.x, target_point.y);
                    used_item = false;
                    for mob in targets.iter() {
                        SufferDamage::new_damage(
                            &mut suffer_damage, *mob, damage.damage
                        );
                        if entity == *player_entity {
                            let mob_name = names.get(*mob).unwrap();
                            let item_name = names.get(use_item.item).unwrap();
                            gamelog.entries.push(format!("You use {} on {}, inflicting {} hp", item_name.name, mob_name.name, damage.damage));
                        }
                        used_item = true;
                    }
                }
            }
            let item_protects = protects.get(use_item.item);
            match item_protects {
                None => {},
                Some(protects) => {
                    used_item = false;
                    for target in targets.iter() {
                        let stats = combat_stats.get_mut(*target);
                        if let Some(stats)= stats {
                            stats.deflects += protects.deflections;
                            if entity == *player_entity {
                                gamelog.entries.push(format!("You drink the {}, adding {} deflections", names.get(use_item.item).unwrap().name, protects.deflections))
                            }
                            let consumable = consumables.get(use_item.item);
                            match consumable {
                                None => {},
                                Some(_) => {
                                    entities.delete(use_item.item).expect("Delete failed");
                                }
                            }
                        }
                        used_item = true;
                    }
                }
            }
            if used_item{
                let consumable = consumables.get(use_item.item);
                match consumable {
                    None => {},
                    Some(_) =>
                    entities.delete(use_item.item).expect("Delete used item failed"),
                }
            }
        }
        wants_to_use.clear();
    }

}

pub struct ItemDropSystem {}

impl<'a>System<'a> for ItemDropSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadExpect<'a, Entity>,
        WriteExpect<'a, GameLog>,
        Entities<'a>,
        WriteStorage<'a,WantsToDropItem>,
        ReadStorage<'a, Name>,
        WriteStorage<'a,Position>,
        WriteStorage<'a, InBackpack>,
        WriteStorage<'a, WantsToExplode>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (player_entity,
            mut gamelog,
            entities,
            mut wants_drop,
            names,
            mut positions,
            mut backpack,
        mut wants_explode) = data;

        for (entity,to_drop) in (&entities, &wants_drop).join() {
            let mut dropper_pos : Position = Position{x:0, y:0};
            {
                let dropped_pos = positions.get(entity).unwrap();
                dropper_pos.x = dropped_pos.x;
                dropper_pos.y = dropped_pos.y;
            }
            positions.insert(to_drop.item, Position{ x: dropper_pos.x, y: dropper_pos.y}).expect("Unable to insert position");
            backpack.remove(to_drop.item);

            if entity == *player_entity {
                gamelog.entries.push(format!("You Drop a {}.", names.get(to_drop.item).unwrap().name));
            }
          //  let  exploding_item = wants_explode.get_mut(to_drop.item).unwrap();
            if let Some(exploding_item) = wants_explode.get_mut(to_drop.item){
                exploding_item.set = true;
                gamelog.entries.push(format!("Bomb set"));
            }
        }

        wants_drop.clear();
    }
}
