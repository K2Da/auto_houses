use super::*;

const MAX_MONSTERS: i32 = 4;
const MAX_ITEMS: i32 = 2;

pub fn spawn_room(world: &mut World, room: &Rect) {
    let mut monster_spawn_points: Vec<usize> = Vec::new();
    let mut item_spawn_points: Vec<usize> = Vec::new();

    {
        let mut rng = world.resources.get_mut::<RandomNumberGenerator>().unwrap();

        let num_monsters = rng.roll_dice(1, MAX_MONSTERS + 2) - 3;
        for _ in 0..num_monsters {
            spawn(room, &mut monster_spawn_points, &mut rng)
        }

        let num_items = rng.roll_dice(1, MAX_ITEMS + 2) - 3;
        for _ in 0..num_items {
            spawn(room, &mut item_spawn_points, &mut rng);
        }
    }

    for idx in monster_spawn_points.iter() {
        let x = *idx % MAPWIDTH;
        let y = *idx / MAPWIDTH;
        random_monster(world, x as i32, y as i32);
    }

    for idx in item_spawn_points.iter() {
        let x = *idx % MAPWIDTH;
        let y = *idx / MAPWIDTH;
        random_item(world, x as i32, y as i32);
    }
}

fn spawn(room: &Rect, spawn_points: &mut Vec<usize>, rng: &mut RandomNumberGenerator) -> () {
    let mut added = false;
    while !added {
        let x = (room.x1 + rng.roll_dice(1, i32::abs(room.x2 - room.x1))) as usize;
        let y = (room.y1 + rng.roll_dice(1, i32::abs(room.y2 - room.y1))) as usize;
        let idx = (y * MAPWIDTH) + x;
        if !spawn_points.contains(&idx) {
            spawn_points.push(idx);
            added = true;
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

pub fn random_monster(world: &mut World, x: i32, y: i32) {
    let roll: i32;
    {
        let mut rng = world.resources.get_mut::<RandomNumberGenerator>().unwrap();
        roll = rng.roll_dice(1, 2);
    }
    match roll {
        1 => orc(world, x, y),
        _ => goblin(world, x, y),
    }
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

fn random_item(world: &mut World, x: i32, y: i32) {
    let roll: i32;
    {
        let mut rng = world.resources.get_mut::<RandomNumberGenerator>().unwrap();
        roll = rng.roll_dice(1, 4);
    }

    match roll {
        1 => health_potion(world, x, y),
        2 => fireball_scroll(world, x, y),
        3 => confusion_scroll(world, x, y),
        _ => magic_missile_scroll(world, x, y),
    }
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
