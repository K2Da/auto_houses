use super::gui::c;
use super::*;
use rltk::prelude::{BLACK, CYAN, MAGENTA, ORANGE, PINK, RED, YELLOW};
use std::collections::hash_map::HashMap;

const MAX_MONSTERS: i32 = 4;

pub fn spawn_room(world: &mut World, room: &rect::Rect, map_depth: i32) {
    let spawn_table = room_table(map_depth);
    let mut spawn_points: HashMap<usize, String> = HashMap::new();

    {
        let mut rng = world.resources.get_mut::<RandomNumberGenerator>().unwrap();
        let num_spawns = rng.roll_dice(1, MAX_MONSTERS + 3) + (map_depth - 1);

        for _ in 0..num_spawns {
            let mut added = false;
            let mut tries = 0;

            while !added && tries < 20 {
                let x = (room.x1 + rng.roll_dice(1, i32::abs(room.x2 - room.x1))) as usize;
                let y = (room.y1 + rng.roll_dice(1, i32::abs(room.y2 - room.y1))) as usize;
                let idx = (y * MAPWIDTH) + x;
                if !spawn_points.contains_key(&idx) {
                    spawn_points.insert(idx, spawn_table.roll(&mut rng));
                    added = true;
                } else {
                    tries += 1;
                }
            }
        }
    }

    for (idx, spawned) in spawn_points.iter() {
        let x = (*idx % MAPWIDTH) as i32;
        let y = (*idx / MAPWIDTH) as i32;

        match spawned.as_ref() {
            "Goblin" => goblin(world, x, y),
            "Orc" => orc(world, x, y),
            "Health Potion" => health_potion(world, x, y),
            "Fireball Scroll" => fireball_scroll(world, x, y),
            "Confusion Scroll" => confusion_scroll(world, x, y),
            "Magic Missile Scroll" => magic_missile_scroll(world, x, y),
            "Dagger" => dagger(world, x, y),
            "Shield" => shield(world, x, y),
            "Longsword" => longsword(world, x, y),
            "Tower Shield" => tower_shield(world, x, y),
            _ => {}
        }
    }
}

fn room_table(map_depth: i32) -> RandomTable {
    RandomTable::new()
        .add("Goblin", 10)
        .add("Orc", 1 + map_depth)
        .add("Health Potion", 7)
        .add("Fireball Scroll", 2 + map_depth)
        .add("Confusion Scroll", 2 + map_depth)
        .add("Magic Missile Scroll", 4)
        .add("Dagger", 3)
        .add("Shield", 3)
        .add("Longsword", map_depth - 1)
        .add("Tower Shield", map_depth - 1)
}

pub fn player(world: &mut World, player_x: i32, player_y: i32) -> Entity {
    world.insert(
        (SerializeMe,),
        vec![(
            Position::new(player_x, player_y),
            Renderable::new(rltk::to_cp437('@'), c(YELLOW), c(BLACK), 0),
            Player::new(),
            Viewshed::new(Vec::new(), 8, true),
            Name::new("Player"),
            CombatStats::new(30, 30, 2, 5),
        )],
    )[0]
}

fn orc(world: &mut World, x: i32, y: i32) {
    monster(world, x, y, rltk::to_cp437('o'), "Orc");
}

fn goblin(world: &mut World, x: i32, y: i32) {
    monster(world, x, y, rltk::to_cp437('g'), "Goblin");
}

fn monster<S: ToString>(world: &mut World, x: i32, y: i32, glyph: u8, name: S) {
    world.insert(
        (SerializeMe, Monster),
        vec![(
            Position::new(x, y),
            Renderable::new(glyph, c(RED), c(BLACK), 1),
            Viewshed::new(Vec::new(), 8, true),
            Name::new(name),
            BlocksTile::new(),
            CombatStats::new(16, 16, 1, 4),
        )],
    );
}

pub fn debug_all_item(world: &mut World, x: i32, y: i32) {
    health_potion(world, x, y);
    magic_missile_scroll(world, x, y);
    fireball_scroll(world, x, y);
    confusion_scroll(world, x, y);
    dagger(world, x, y);
    shield(world, x, y);
    longsword(world, x, y);
    tower_shield(world, x, y);
}

fn health_potion(world: &mut World, x: i32, y: i32) {
    world.insert(
        (SerializeMe, Item, Consumable),
        vec![(
            Position::new(x, y),
            Renderable::new(rltk::to_cp437('¡'), c(MAGENTA), c(BLACK), 2),
            Name::new("Health Potion"),
            ProvidesHealing::new(8),
        )],
    );
}

fn magic_missile_scroll(world: &mut World, x: i32, y: i32) {
    world.insert(
        (SerializeMe, Item, Consumable),
        vec![(
            Position::new(x, y),
            Renderable::new(rltk::to_cp437(')'), c(CYAN), c(BLACK), 2),
            Name::new("Magic Missile Scroll"),
            Ranged::new(6),
            InflictsDamage::new(8),
        )],
    );
}

fn fireball_scroll(world: &mut World, x: i32, y: i32) {
    world.insert(
        (SerializeMe, Item, Consumable),
        vec![(
            Position::new(x, y),
            Renderable::new(rltk::to_cp437(')'), c(ORANGE), c(BLACK), 2),
            Name::new("Fireball Scroll"),
            Ranged::new(6),
            InflictsDamage::new(20),
            AreaOfEffect::new(3),
        )],
    );
}

fn confusion_scroll(world: &mut World, x: i32, y: i32) {
    world.insert(
        (SerializeMe, Item, Consumable),
        vec![(
            Position::new(x, y),
            Renderable::new(rltk::to_cp437(')'), c(PINK), c(BLACK), 2),
            Name::new("Confusion Scroll"),
            Ranged::new(6),
            Confusion::new(4),
        )],
    );
}

fn dagger(world: &mut World, x: i32, y: i32) {
    world.insert(
        (SerializeMe, Item),
        vec![(
            Position::new(x, y),
            Renderable::new(rltk::to_cp437('/'), c(CYAN), c(BLACK), 2),
            Name::new("Dagger"),
            Equippable::new(EquipmentSlot::Melee),
            MeleePowerBonus::new(2),
        )],
    );
}

fn shield(world: &mut World, x: i32, y: i32) {
    world.insert(
        (SerializeMe, Item),
        vec![(
            Position::new(x, y),
            Renderable::new(rltk::to_cp437('('), c(CYAN), c(BLACK), 2),
            Name::new("Shield"),
            Equippable::new(EquipmentSlot::Shield),
            DefenseBonus::new(1),
        )],
    );
}

fn longsword(world: &mut World, x: i32, y: i32) {
    world.insert(
        (SerializeMe, Item),
        vec![(
            Position::new(x, y),
            Renderable::new(rltk::to_cp437('/'), c(YELLOW), c(BLACK), 2),
            Name::new("Longsword"),
            Equippable::new(EquipmentSlot::Melee),
            MeleePowerBonus::new(4),
        )],
    );
}

fn tower_shield(world: &mut World, x: i32, y: i32) {
    world.insert(
        (SerializeMe, Item),
        vec![(
            Position::new(x, y),
            Renderable::new(rltk::to_cp437('('), c(YELLOW), c(BLACK), 2),
            Name::new("Tower Shield"),
            Equippable::new(EquipmentSlot::Shield),
            DefenseBonus::new(3),
        )],
    );
}
