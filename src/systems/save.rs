use super::super::*;
use serde::{Deserialize, Serialize};

macro_rules! iterate_components {
    ($attempt_macro: ident, $args: expr) => {
        $attempt_macro!(
            $args,
            (AreaOfEffect, area_of_effect),
            (BlocksTile, blocks_tile),
            (CombatStats, combat_stats),
            (Confusion, confusion),
            (DefenseBonus, defense_bonus),
            (Equippable, equippable),
            (Equipped, equipped),
            (InBackpack, in_backpack),
            (InflictsDamage, inflicts_damage),
            (MeleePowerBonus, melee_power_bonus),
            (Name, name),
            (Player, player),
            (Position, position),
            (ProvidesHealing, provides_healing),
            (Ranged, ranged),
            (Renderable, renderable),
            (SufferDamage, suffer_damage),
            (Viewshed, viewshed),
            (WantsToDropItem, wants_to_drop_item),
            (WantsToMelee, wants_to_melee),
            (WantsToPickupItem, wants_to_pickup_item),
            (WantsToRemoveItem, wants_to_remove_item),
            (WantsToUseItem, wants_to_use_item)
        );
    };
}

macro_rules! iterate_tags {
    ($attempt_macro: ident, $args: expr) => {
        $attempt_macro!(
            $args,
            (Consumable, consumable),
            (Item, item),
            (Monster, monster),
            (SerializeMe, serialize_me)
        );
    };
}

macro_rules! component_struct {
    ($_: expr, $(($type:ty, $member:ident)), *) => {
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
        pub struct SavedComponents {
            $($member: Vec<(String, $type)>,)*
        }
    };
}

macro_rules! tag_struct {
    ($_: expr, $(($type:ty, $member:ident)), *) => {
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
        pub struct SavedTags {
            $($member: Vec<String>,)*
        }
    };
}

iterate_components! { component_struct, () }

iterate_tags! { tag_struct, () }

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct SaveData {
    map: Map,
    entities: Vec<String>,
    components: SavedComponents,
    tags: SavedTags,
}

pub mod load_system;
pub mod save_system;
