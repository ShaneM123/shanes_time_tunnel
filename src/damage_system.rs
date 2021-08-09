

use specs::prelude::*;
use super::{CombatStats, SufferDamage};
use crate::player::Player;
use rltk::console;
use crate::gamelog::GameLog;
use crate::components::Name;

pub struct DamageSystem {}

impl<'a> System<'a> for DamageSystem {
    type SystemData = (
    WriteStorage<'a, CombatStats>,
        WriteStorage<'a, SufferDamage>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut stats, mut damage) = data;

        for (mut stats, damage) in (&mut stats, &damage).join() {
            //dont inflict damage if the entity has deflections
            if stats.deflects > 0 {
                stats.deflects -= 1;
                if stats.hp < stats.max_hp {
                    stats.hp += 1;
                }
            }
            else {
                stats.hp -= damage.amount.iter().sum::<i32>();
            }
        }
        damage.clear();
    }

}
pub fn delete_the_dead(ecs: &mut World) {
    let mut dead: Vec<Entity> = Vec::new();
    //using a scope to make the borrow checker happy
    {
        let combat_stats = ecs.read_storage::<CombatStats>();
        let players = ecs.read_storage::<Player>();
        let entities = ecs.entities();
        let names = ecs.read_storage::<Name>();
        let mut log = ecs.write_resource::<GameLog>();

        for (entity, stats) in (&entities, &combat_stats).join() {
            if stats.hp < 1  {
                let player = players.get(entity);
                match player {
                    None => {
                        let victim_name = names.get(entity);
                        if let Some(victim_name) = victim_name {
                            log.entries.push(format!("{} is dead", &victim_name.name));
                        }
                        dead.push(entity);
                    },
                    Some(_) => console::log("you are dead")
                }
                 }
        }
    }

    //TODO: use delete_entities
    for victim in dead {
        ecs.delete_entity(victim).expect("unable to delete");
    }
}
