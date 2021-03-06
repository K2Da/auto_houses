use super::*;
mod damage_system;
mod delete_the_dead_system;
mod inventory;
mod map_indexing_system;
mod melee_combat_system;
mod monster_ai_system;
mod player;
pub mod save;
mod visibility_system;

pub struct PlayerSchedules {
    pub player_move: schedule::Schedule,
    pub get_item: schedule::Schedule,
}

pub struct MenuSchedules {
    pub save: schedule::Schedule,
    pub load: schedule::Schedule,
}

pub struct Schedules {
    pub main: schedule::Schedule,
    pub delete_the_dead: schedule::Schedule,
    pub player: PlayerSchedules,
    pub menu: MenuSchedules,
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
            .add_system(inventory::inventory_system::build())
            .flush()
            .add_system(inventory::item_use_system::build())
            .flush()
            .add_system(inventory::item_drop_system::build())
            .flush()
            .add_system(inventory::item_remove_system::build())
            .flush()
            .build(),
        delete_the_dead: schedule(delete_the_dead_system::build()),
        player: PlayerSchedules {
            player_move: schedule(player::move_system::build()),
            get_item: schedule(player::get_item_system::build()),
        },
        menu: MenuSchedules {
            save: schedule(save::save_system::build()),
            load: save::load_system::schedule(),
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

pub fn schedule(system_box: SystemBox) -> legion::schedule::Schedule {
    Schedule::builder().add_system(system_box).flush().build()
}
