use super::*;
use std::collections::hash_map::HashMap;

const MAX_MONSTERS: i32 = 4;

pub fn spawn_room(world: &mut World, room: &Rect, map_depth: i32) {
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

    for spawn in spawn_points.iter() {
        let x = (*spawn.0 % MAPWIDTH) as i32;
        let y = (*spawn.0 / MAPWIDTH) as i32;

        match spawn.1.as_ref() {
            "Goblin" => goblin(world, x, y),
            "Orc" => orc(world, x, y),
            "Health Potion" => health_potion(world, x, y),
            "Fireball Scroll" => fireball_scroll(world, x, y),
            "Confusion Scroll" => confusion_scroll(world, x, y),
            "Magic Missile Scroll" => magic_missile_scroll(world, x, y),
            _ => {}
        }
    }
}

pub fn player(world: &mut World, player_x: i32, player_y: i32) -> Entity {
    world.insert(
        (SerializeMe,),
        vec![(
            Position {
                x: player_x,
                y: player_y,
            },
            Renderable {
                glyph: rltk::to_cp437('@'),
                fg: RGB::named(rltk::YELLOW),
                bg: RGB::named(rltk::BLACK),
                render_order: 0,
            },
            Player {},
            Viewshed {
                visible_tiles: Vec::new(),
                range: 8,
                dirty: true,
            },
            Name {
                name: "Player".to_string(),
            },
            CombatStats {
                max_hp: 30,
                hp: 30,
                defense: 2,
                power: 5,
            },
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
            Position { x, y },
            Renderable {
                glyph,
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
                render_order: 1,
            },
            Viewshed {
                visible_tiles: Vec::new(),
                range: 8,
                dirty: true,
            },
            Name {
                name: name.to_string(),
            },
            BlocksTile {},
            CombatStats {
                max_hp: 16,
                hp: 16,
                defense: 1,
                power: 4,
            },
        )],
    );
}

pub fn debug_all_item(world: &mut World, x: i32, y: i32) {
    health_potion(world, x, y);
    magic_missile_scroll(world, x, y);
    fireball_scroll(world, x, y);
    confusion_scroll(world, x, y);
}

fn health_potion(world: &mut World, x: i32, y: i32) {
    world.insert(
        (SerializeMe, Item, Consumable),
        vec![(
            Position { x, y },
            Renderable {
                glyph: rltk::to_cp437('¡'),
                fg: RGB::named(rltk::MAGENTA),
                bg: RGB::named(rltk::BLACK),
                render_order: 2,
            },
            Name {
                name: "Health Potion".to_string(),
            },
            ProvidesHealing { heal_amount: 8 },
        )],
    );
}

fn magic_missile_scroll(world: &mut World, x: i32, y: i32) {
    world.insert(
        (SerializeMe, Item, Consumable),
        vec![(
            Position { x, y },
            Renderable {
                glyph: rltk::to_cp437(')'),
                fg: RGB::named(rltk::CYAN),
                bg: RGB::named(rltk::BLACK),
                render_order: 2,
            },
            Name {
                name: "Magic Missile Scroll".to_string(),
            },
            Ranged { range: 6 },
            InflictsDamage { damage: 8 },
        )],
    );
}

fn fireball_scroll(world: &mut World, x: i32, y: i32) {
    world.insert(
        (SerializeMe, Item, Consumable),
        vec![(
            Position { x, y },
            Renderable {
                glyph: rltk::to_cp437(')'),
                fg: RGB::named(rltk::ORANGE),
                bg: RGB::named(rltk::BLACK),
                render_order: 2,
            },
            Name {
                name: "Fireball Scroll".to_string(),
            },
            Ranged { range: 6 },
            InflictsDamage { damage: 20 },
            AreaOfEffect { radius: 3 },
        )],
    );
}

fn confusion_scroll(world: &mut World, x: i32, y: i32) {
    world.insert(
        (SerializeMe, Item, Consumable),
        vec![(
            Position { x, y },
            Renderable {
                glyph: rltk::to_cp437(')'),
                fg: RGB::named(rltk::PINK),
                bg: RGB::named(rltk::BLACK),
                render_order: 2,
            },
            Name {
                name: "Confusion Scroll".to_string(),
            },
            Ranged { range: 6 },
            Confusion { turns: 4 },
        )],
    );
}

fn room_table(map_depth: i32) -> RandomTable {
    RandomTable::new()
        .add("Goblin", 10)
        .add("Orc", 1 + map_depth)
        .add("Health Potion", 7)
        .add("Fireball Scroll", 2 + map_depth)
        .add("Confusion Scroll", 2 + map_depth)
        .add("Magic Missile Scroll", 4)
}
