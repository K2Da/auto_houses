use super::super::*;
use super::*;
use std::fs::File;
use std::path::Path;

macro_rules! serialize_tags {
    ($args: expr, $(($type:ty, $member:ident)), *) => {
        $(
            let (save, world, entity) = $args;
            if let Some(_) = world.get_tag::<$type>(entity) {
                save.tags.$member.push(format!("{}", entity));
            }
        )*
    };
}

macro_rules! serialize_individually {
    ($none: expr, $(($type:ty, $member:ident)), *) => {
        SystemBuilder::<()>::new("SaveSystem")
            $(.read_component::<$type>())*
            .read_resource::<Map>()
            .with_query(<Tagged<SerializeMe>>::query())
            .build(move |_commands, world, map, query| {
                let mut save = SaveData::default();
                let map: &Map = map;
                save.map = map.clone();
                for (entity, _) in query.iter_entities(world) {
                    save.entities.push(format!("{}", entity));
                    $(
                        if let Some(component) = world.get_component::<$type>(entity) {
                            let mut component = (&*component).clone();
                            component.store_entity_id();
                            save.components.$member.push((format!("{}", entity), component));
                        }
                    )*
                    iterate_tags!(serialize_tags, (&mut save, &world, entity));
                }
                let writer = File::create("./savegame.json").unwrap();
                serde_json::to_writer(writer, &save).unwrap();
            })
    };
}

pub fn build() -> SystemBox {
    iterate_components!(serialize_individually, ())
}

pub fn does_save_exist() -> bool {
    Path::new("./savegame.json").exists()
}

pub fn delete_save() {
    if Path::new("./savegame.json").exists() {
        std::fs::remove_file("./savegame.json").expect("Unable to delete file");
    }
}
