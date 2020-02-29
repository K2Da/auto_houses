use legion::entity::Entity;
use rltk::RGB;

// TODO: dataのないcomponentはタグに変えるべき？

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Renderable {
    pub glyph: u8,
    pub fg: RGB,
    pub bg: RGB,
    pub render_order: i32,
}

// TryRead等があるのでtagに変えれない
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player;

#[derive(Clone, Debug, PartialEq)]
pub struct Viewshed {
    pub visible_tiles: Vec<rltk::Point>,
    pub range: i32,
    pub dirty: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Name {
    pub name: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BlocksTile;

#[derive(Clone, Debug, PartialEq)]
pub struct CombatStats {
    pub max_hp: i32,
    pub hp: i32,
    pub defense: i32,
    pub power: i32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WantsToMelee {
    pub target: Entity,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SufferDamage {
    pub amount: i32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Potion {
    pub heal_amount: i32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct InBackpack {
    pub owner: Entity,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WantsToPickupItem {
    pub collected_by: Entity,
    pub item: Entity,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WantsToDrinkPotion {
    pub potion: Entity,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WantsToDropItem {
    pub item: Entity,
}
