use specs::{prelude::*, saveload::{Marker, ConvertSaveload}, error::NoError};
use specs_derive::*;
use serde::{Serialize, Deserialize};
use rltk::RGB;


#[derive(Component, Serialize, Deserialize, Clone, Debug)]
pub struct Player {}

#[derive(Component, ConvertSaveload, Clone, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}
#[derive(Component, ConvertSaveload, Clone, Debug)]
pub(crate) struct Renderable {
    pub glyph: rltk::FontCharType,
    pub fg: RGB,
    pub bg: RGB,
    pub render_order: i32,
}

#[derive(Component, ConvertSaveload, Clone, Debug)]
pub struct Viewshed {
    pub visible_tiles: Vec<rltk::Point>,
    pub range: i32,
    pub dirty: bool,
}

#[derive(Component, Serialize, Deserialize, Clone, Debug)]
pub struct Monster {}

#[derive(Component, ConvertSaveload, Clone, Debug)]
pub struct Name {
    pub name: String,
}

#[derive(Component, Serialize, Deserialize, Clone, Debug)]
pub struct BlocksTile {
}

pub struct SerializeMe;

#[derive(Component, ConvertSaveload, Clone, Debug)]
pub struct CombatStats {
    pub max_hp: i32,
    pub hp: i32,
    pub defense: i32,
    pub power: i32,
    pub deflects: i32,
}

#[derive(Component, ConvertSaveload, Clone, Debug)]
pub struct WantsToMelee {
    pub target: Entity
}

#[derive(Component, ConvertSaveload, Clone, Debug)]
pub struct SufferDamage {
    pub amount: Vec<i32>,
}

#[derive(Component, Serialize, Deserialize, Clone, Debug)]
pub struct Item {}

#[derive(Component, ConvertSaveload, Clone, Debug)]
pub struct ProvidesHealing {
    pub heal_amount: i32,
}
#[derive(Component, ConvertSaveload, Clone, Debug)]
pub struct ProvidesWhack {
    pub whack_amount: i32,
}

#[derive(Component, ConvertSaveload, Clone, Debug)]
pub struct InBackpack {
    pub owner: Entity,
}

#[derive(Component, ConvertSaveload, Clone, Debug)]
pub struct WantsToPickupItem {
    pub collected_by: Entity,
    pub item: Entity,
}

#[derive(Component, Serialize, Deserialize, Clone, Debug)]
pub struct Consumable {
}

#[derive(Component, ConvertSaveload, Clone, Debug)]
pub struct WantsToUseItem {
    pub item: Entity,
    pub target : Option<rltk::Point>,
}

#[derive(Component, ConvertSaveload, Clone, Debug)]
pub struct WantsToDropItem {
    pub item : Entity
}

#[derive(Component, ConvertSaveload, Clone, Debug)]
pub struct Ranged {
    pub range: i32,
}
#[derive(Component, ConvertSaveload, Clone, Debug)]
pub struct InflictsDamage {
    pub damage: i32
}
#[derive(Component, ConvertSaveload, Clone, Debug)]
pub struct AreaOfEffect {
    pub radius : i32
}

#[derive(Component, ConvertSaveload, Clone, Debug)]
pub struct Protects {
    pub deflections: i32
}

#[derive(Component, ConvertSaveload, Clone, Debug)]
pub struct WantsToExplode{
    pub set: bool,
    pub timer: i32,
}

// Special component that exists to help serialize the game data
#[derive(Component, Serialize, Deserialize, Clone)]
pub struct SerializationHelper {
    pub map : super::map::Map
}

impl SufferDamage {
    pub fn new_damage(store: &mut WriteStorage<SufferDamage>, victim: Entity, amount: i32) {
        if let Some(suffering) = store.get_mut(victim) {
            suffering.amount.push(amount);
        }
        else {
            let dmg = SufferDamage { amount: vec![amount] };
            store.insert(victim, dmg).expect("Unable to insert Damage");
        }
    }
}

