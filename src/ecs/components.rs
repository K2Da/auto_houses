use super::entity_holder::EntityHolder;
use legion::entity;
use rltk::RGB;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub trait Component {
    fn restore_entity(&mut self, entity_dic: &HashMap<String, entity::Entity>) {
        for member in self.entity_members() {
            member.restore_entity(entity_dic);
        }
    }

    fn store_entity_id(&mut self) {
        for member in self.entity_members() {
            member.store_entity_id();
        }
    }

    fn entity_members(&mut self) -> Vec<&mut EntityHolder> {
        vec![]
    }
}

// A
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AreaOfEffect {
    pub radius: i32,
}

impl Component for AreaOfEffect {}

// B
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlocksTile;

impl Component for BlocksTile {}

// C
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CombatStats {
    pub max_hp: i32,
    pub hp: i32,
    pub defense: i32,
    pub power: i32,
}

impl Component for CombatStats {}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Confusion {
    pub turns: i32,
}

impl Component for Confusion {}

// D
// E
// F
// G
// H
// I
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InBackpack {
    pub owner: EntityHolder,
}

impl Component for InBackpack {
    fn entity_members(&mut self) -> Vec<&mut EntityHolder> {
        vec![&mut self.owner]
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InflictsDamage {
    pub damage: i32,
}

impl Component for InflictsDamage {}

// J
// K
// L
// M
// N
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Name {
    pub name: String,
}

impl Component for Name {}

// O
#[derive(Clone, Debug, PartialEq)]
pub struct OldEntityID {
    pub entity_id: String,
}

impl Component for OldEntityID {}

// P
// TryRead等があるのでtagに変えれない
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Player;

impl Component for Player {}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Component for Position {}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProvidesHealing {
    pub heal_amount: i32,
}

impl Component for ProvidesHealing {}

// Q
// R
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Ranged {
    pub range: i32,
}

impl Component for Ranged {}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Renderable {
    pub glyph: u8,
    pub fg: RGB,
    pub bg: RGB,
    pub render_order: i32,
}

impl Component for Renderable {}

// S
// オリジナルではentityに紐づくダメージの配列だが、同じ実装ができないので別entityとする
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SufferDamage {
    pub victim: EntityHolder,
    pub amount: i32,
}

impl Component for SufferDamage {
    fn entity_members(&mut self) -> Vec<&mut EntityHolder> {
        vec![&mut self.victim]
    }
}

// T
// U
// V
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Viewshed {
    pub visible_tiles: Vec<rltk::Point>,
    pub range: i32,
    pub dirty: bool,
}

impl Component for Viewshed {}

// W
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WantsToDropItem {
    pub item: EntityHolder,
}

impl Component for WantsToDropItem {
    fn entity_members(&mut self) -> Vec<&mut EntityHolder> {
        vec![&mut self.item]
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WantsToMelee {
    pub target: EntityHolder,
}

impl Component for WantsToMelee {
    fn entity_members(&mut self) -> Vec<&mut EntityHolder> {
        vec![&mut self.target]
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WantsToPickupItem {
    pub collected_by: EntityHolder,
    pub item: EntityHolder,
}

impl Component for WantsToPickupItem {
    fn entity_members(&mut self) -> Vec<&mut EntityHolder> {
        vec![&mut self.collected_by, &mut self.item]
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WantsToUseItem {
    pub item: EntityHolder,
    pub target: Option<rltk::Point>,
}

impl Component for WantsToUseItem {
    fn entity_members(&mut self) -> Vec<&mut EntityHolder> {
        vec![&mut self.item]
    }
}

// X
// Y
// Z
