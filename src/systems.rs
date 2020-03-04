use super::*;
mod damage_system;
mod delete_the_dead_system;
mod inventory;
mod map_indexing_system;
mod melee_combat_system;
mod monster_ai_system;
mod player;
mod visibility_system;

pub struct PlayerSchedules {
    pub player_move: schedule::Schedule,
    pub get_item: schedule::Schedule,
}

pub struct Schedules {
    pub main: schedule::Schedule,
    pub player: PlayerSchedules,
}

pub fn build_schedules() -> Schedules {
    Schedules {
        main: Schedule::builder()
            .add_system(visibility_system::build())
            .add_system(monster_ai_system::build())
            .flush()
            .add_system(map_indexing_system::build())
            .flush()
            .add_system(melee_combat_system::build())
            .flush()
            .add_system(damage_system::build())
            .flush()
            .add_system(delete_the_dead_system::build())
            .flush()
            .add_system(inventory::inventory_system::build())
            .flush()
            .add_system(inventory::item_use_system::build())
            .flush()
            .add_system(inventory::item_drop_system::build())
            .flush()
            .build(),
        player: PlayerSchedules {
            player_move: player::move_system::schedule(),
            get_item: player::get_item_system::schedule(),
        },
    }
}

fn get_name(world: &legion::system::SubWorld, entity: Entity) -> String {
    world.get_component::<Name>(entity).unwrap().name.to_owned()
}

fn retain_tiles(map: &Map, tiles: &mut Vec<rltk::Point>) {
    tiles.retain(|p| {
        p.x > 0 && p.x < map.width as i32 - 1 && p.y > 0 && p.y < map.height as i32 - 1
    });
}
