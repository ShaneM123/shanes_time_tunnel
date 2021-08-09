use specs::prelude::*;
use super::{WantsToPickupItem, Name, InBackpack, gamelog::GameLog};
use crate::components::{ProvidesHealing, CombatStats, Position, WantsToUseItem, WantsToDropItem, Consumable, SufferDamage, InflictsDamage, AreaOfEffect, WantsToExplode};
use crate::map::Map;

pub struct SetExplosiveSystem {}

impl<'a>System<'a> for SetExplosiveSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadExpect<'a, Entity>,
        WriteExpect<'a, GameLog>,
        Entities<'a>,
        ReadStorage<'a, Consumable>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, ProvidesHealing>,
        WriteStorage<'a,CombatStats>,
        ReadStorage<'a, InflictsDamage>,
        ReadExpect<'a, Map>,
        WriteStorage<'a, SufferDamage>,
        ReadStorage<'a, AreaOfEffect>,
        WriteStorage<'a,WantsToExplode>,
        ReadStorage<'a, Position>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (player_entity,
            mut gamelog,
            entities,
            consumables,
            names,
            healing,
            mut combat_stats,
            inflict_damage,
            map,
            mut suffer_damage,
            aoe,
        mut wants_explode,
        pos) = data;

        //TODO: figure out how to get a blas radius thats not associated with the users field of view or direct target

        // for (entity, wants_explode,pos) in (&entities, &wants_explode, &pos).join() {
        //     if wants_explode.set {
        //         if wants_explode.timer <= 0 {
        //             let area_effect = aoe.get(entity);
        //             let mut targets = Vec::new();
        //             match area_effect {
        //                 None => {
        //                     let idx = map.xy_idx(pos.x, pos.y);
        //                     for mob in map.tile_content[idx].iter(){
        //                         targets.push(*mob)
        //                     }
        //                 }
        //                 Some(area_effect) => {
        //                     let idx = map.xy_idx(pos.x, pos.y);
        //                     let mut blast_tiles = rltk::(idx, area_effect.radius,&*map);
        //                     blast_tiles.retain(|p| p.x > 0 && p.x < map.width-1 && p.y > 0 && p.y < map.height -1 );
        //                     for tile_idx in blast_tiles.iter() {
        //                         let idx = map.xy_idx(tile_idx.x,tile_idx.y);
        //                         for mob in map.tile_content[idx].iter() {
        //                             targets.push(*mob);
        //                         }
        //                     }
        //                 }
        //             }
        //
        //         }
        //         else {*wants_explode.timer -= 1;}
        //     }
        // }

        }
}
