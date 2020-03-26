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

impl AreaOfEffect {
    pub fn new(radius: i32) -> Self {
        Self { radius }
    }
}

// B
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlocksTile;

impl Component for BlocksTile {}

impl BlocksTile {
    pub fn new() -> Self {
        Self {}
    }
}

// C
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CombatStats {
    pub max_hp: i32,
    pub hp: i32,
    pub defense: i32,
    pub power: i32,
}

impl CombatStats {
    pub fn new(max_hp: i32, hp: i32, defense: i32, power: i32) -> Self {
        Self {
            max_hp,
            hp,
            defense,
            power,
        }
    }
}

impl Component for CombatStats {}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Confusion {
    pub turns: i32,
}

impl Confusion {
    pub fn new(turns: i32) -> Self {
        Self { turns }
    }
}

impl Component for Confusion {}

// D
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DefenseBonus {
    pub defense: i32,
}

impl DefenseBonus {
    pub fn new(defense: i32) -> Self {
        Self { defense }
    }
}

impl Component for DefenseBonus {}

// E
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum EquipmentSlot {
    Melee,
    Shield,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Equippable {
    pub slot: EquipmentSlot,
}

impl Equippable {
    pub fn new(slot: EquipmentSlot) -> Self {
        Self { slot }
    }
}

impl Component for Equippable {}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Equipped {
    pub owner: EntityHolder,
    pub slot: EquipmentSlot,
}

impl Equipped {
    pub fn new(owner: legion::entity::Entity, slot: EquipmentSlot) -> Self {
        Self {
            owner: EntityHolder::new(owner),
            slot,
        }
    }
}

impl Component for Equipped {
    fn entity_members(&mut self) -> Vec<&mut EntityHolder> {
        vec![&mut self.owner]
    }
}

// F
// G
// H
// I
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InBackpack {
    pub owner: EntityHolder,
}

impl InBackpack {
    pub fn new(owner: legion::entity::Entity) -> Self {
        Self {
            owner: EntityHolder::new(owner),
        }
    }
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

impl InflictsDamage {
    pub fn new(damage: i32) -> Self {
        Self { damage }
    }
}

impl Component for InflictsDamage {}

// J
// K
// L
// M
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MeleePowerBonus {
    pub power: i32,
}

impl MeleePowerBonus {
    pub fn new(power: i32) -> Self {
        Self { power }
    }
}

impl Component for MeleePowerBonus {}

// N
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Name {
    pub name: String,
}

impl Name {
    pub fn new<S: ToString>(name: S) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl Component for Name {}

// O
#[derive(Clone, Debug, PartialEq)]
pub struct OldEntityID {
    pub entity_id: String,
}

impl OldEntityID {
    pub fn new<S: ToString>(entity_id: S) -> Self {
        Self {
            entity_id: entity_id.to_string(),
        }
    }
}

impl Component for OldEntityID {}

// P
// TryRead等があるのでtagに変えれない
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Player;

impl Player {
    pub fn new() -> Self {
        Self {}
    }
}

impl Component for Player {}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Component for Position {}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProvidesHealing {
    pub heal_amount: i32,
}

impl ProvidesHealing {
    pub fn new(heal_amount: i32) -> Self {
        Self { heal_amount }
    }
}

impl Component for ProvidesHealing {}

// Q
// R
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Ranged {
    pub range: i32,
}

impl Ranged {
    pub fn new(range: i32) -> Self {
        Self { range }
    }
}

impl Component for Ranged {}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Renderable {
    pub glyph: u8,
    pub fg: RGB,
    pub bg: RGB,
    pub render_order: i32,
}

impl Renderable {
    pub fn new(glyph: u8, fg: RGB, bg: RGB, render_order: i32) -> Self {
        Self {
            glyph,
            fg,
            bg,
            render_order,
        }
    }
}

impl Component for Renderable {}

// S
// オリジナルではentityに紐づくダメージの配列だが、同じ実装ができないので別entityとする
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SufferDamage {
    pub victim: EntityHolder,
    pub amount: i32,
}

impl SufferDamage {
    pub fn new(victim: legion::entity::Entity, amount: i32) -> Self {
        Self {
            victim: EntityHolder::new(victim),
            amount,
        }
    }
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

impl Viewshed {
    pub fn new(visible_tiles: Vec<rltk::Point>, range: i32, dirty: bool) -> Self {
        Self {
            visible_tiles,
            range,
            dirty,
        }
    }
}

impl Component for Viewshed {}

// W
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WantsToDropItem {
    pub item: EntityHolder,
}

impl WantsToDropItem {
    pub fn new(item: legion::entity::Entity) -> Self {
        Self {
            item: EntityHolder::new(item),
        }
    }
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

impl WantsToMelee {
    pub fn new(target: legion::entity::Entity) -> Self {
        Self {
            target: EntityHolder::new(target),
        }
    }
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

impl WantsToPickupItem {
    pub fn new(collected_by: legion::entity::Entity, item: legion::entity::Entity) -> Self {
        Self {
            collected_by: EntityHolder::new(collected_by),
            item: EntityHolder::new(item),
        }
    }
}

impl Component for WantsToPickupItem {
    fn entity_members(&mut self) -> Vec<&mut EntityHolder> {
        vec![&mut self.collected_by, &mut self.item]
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WantsToRemoveItem {
    pub item: EntityHolder,
}

impl WantsToRemoveItem {
    pub fn new(item: legion::entity::Entity) -> Self {
        Self {
            item: EntityHolder::new(item),
        }
    }
}

impl Component for WantsToRemoveItem {
    fn entity_members(&mut self) -> Vec<&mut EntityHolder> {
        vec![&mut self.item]
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WantsToUseItem {
    pub item: EntityHolder,
    pub target: Option<rltk::Point>,
}

impl WantsToUseItem {
    pub fn new(item: legion::entity::Entity, target: Option<rltk::Point>) -> Self {
        Self {
            item: EntityHolder::new(item),
            target,
        }
    }
}

impl Component for WantsToUseItem {
    fn entity_members(&mut self) -> Vec<&mut EntityHolder> {
        vec![&mut self.item]
    }
}

// X
// Y
// Z
