use legion::entity::Entity;
use rltk::RGB;

#[derive(Clone, Debug, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Renderable {
    pub glyph: u8,
    pub fg: RGB,
    pub bg: RGB,
    pub render_order: i32,
}

// TryRead等があるのでtagに変えれない
#[derive(Clone, Debug, PartialEq)]
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

// オリジナルではentityに紐づくダメージの配列だが、同じ実装ができないので別entityとする
#[derive(Clone, Debug, PartialEq)]
pub struct SufferDamage {
    pub victim: Entity,
    pub amount: i32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProvidesHealing {
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
pub struct WantsToUseItem {
    pub item: Entity,
    pub target: Option<rltk::Point>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WantsToDropItem {
    pub item: Entity,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Ranged {
    pub range: i32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct InflictsDamage {
    pub damage: i32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AreaOfEffect {
    pub radius: i32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Confusion {
    pub turns: i32,
}
