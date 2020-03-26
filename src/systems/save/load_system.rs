use super::super::*;
use super::*;
use std::collections::HashMap;
use std::fs::File;

pub fn schedule() -> legion::schedule::Schedule {
    Schedule::builder()
        .add_system(initialize_entities())
        .flush()
        .add_system(load_components())
        .flush()
        .add_system(write_player_resource())
        .flush()
        .build()
}

pub fn initialize_entities() -> SystemBox {
    SystemBuilder::<()>::new("InitializeEntities")
        .with_query(<Tagged<SerializeMe>>::query())
        .write_resource::<SaveData>()
        .build(move |commands, world, save_data, query| {
            let save_data: &mut SaveData = save_data;

            for (entity, _) in query.iter_entities(world) {
                commands.delete(entity);
            }

            *save_data = serde_json::from_reader(File::open("./savegame.json").unwrap()).unwrap();

            for entity_id in &save_data.entities {
                commands.insert((SerializeMe,), vec![(OldEntityID::new(entity_id),)]);
            }
        })
}

macro_rules! load_components {
    ($args: expr, $(($type:ty, $member:ident)), *) => {
        $(
            let (save_data, entity_dic, commands) = $args;

            for (entity_id, component) in save_data.components.$member.iter() {
                let entity = entity_dic.get(entity_id).unwrap().clone();
                let mut component = component.clone();
                component.restore_entity(entity_dic);
                commands.add_component(entity, component);
            }
        )*
    };
}

macro_rules! load_tags {
    ($args: expr, $(($type:ty, $member:ident)), *) => {
        $(
            let (save_data, entity_dic, commands) = $args;

            for entity_id in save_data.tags.$member.iter() {
                let entity = entity_dic.get(entity_id).unwrap().clone();
                let tag = <$type>::default();
                commands.add_tag::<$type>(entity, tag);
            }
        )*
    };
}

pub fn load_components() -> SystemBox {
    SystemBuilder::<()>::new("LoadComponents")
        .with_query(<Read<OldEntityID>>::query())
        .write_resource::<SaveData>()
        .write_resource::<Map>()
        .build(move |commands, world, (save_data, map), query| {
            let save_data: &mut SaveData = save_data;
            let mut entity_dic = HashMap::new();

            for (entity, old_entity_id) in query.iter_entities(world) {
                entity_dic.insert(old_entity_id.entity_id.to_owned(), entity);
            }

            iterate_components!(load_components, (&save_data, &entity_dic, &commands));

            iterate_tags!(load_tags, (&save_data, &entity_dic, &commands));

            let map: &mut Map = map;
            *map = save_data.map.clone();
            map.tile_content = vec![Vec::new(); super::super::map::MAPCOUNT];
        })
}

pub fn write_player_resource() -> SystemBox {
    SystemBuilder::<()>::new("WritePlayerResource")
        .with_query(<(Read<Player>, Read<Position>)>::query())
        .write_resource::<Entity>()
        .write_resource::<Point>()
        .build(
            move |_commands, world, (player_entity, player_position), query| {
                let player_entity: &mut Entity = player_entity;
                let player_position: &mut Point = player_position;

                for (entity, (_player, position)) in query.iter_entities(world) {
                    *player_position = Point::new(position.x, position.y);
                    *player_entity = entity;
                }
            },
        )
}
