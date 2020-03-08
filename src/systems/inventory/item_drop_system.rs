use super::super::*;

pub fn build() -> SystemBox {
    SystemBuilder::<()>::new("ItemDropSystem")
        .with_query(<Read<WantsToDropItem>>::query())
        .read_resource::<Entity>()
        .write_resource::<GameLog>()
        .read_component::<Name>()
        .read_component::<Position>()
        .build(move |commands, world, (player_entity, gamelog), query| {
            let player_entity: &Entity = player_entity;

            for (entity, to_drop) in query.iter_entities(world) {
                let mut dropper_pos: Position = Position { x: 0, y: 0 };
                {
                    let dropped_pos = world.get_component::<Position>(entity).unwrap();
                    dropper_pos.x = dropped_pos.x;
                    dropper_pos.y = dropped_pos.y;
                }
                commands.add_component(
                    to_drop.item.entity(),
                    Position {
                        x: dropper_pos.x,
                        y: dropper_pos.y,
                    },
                );
                commands.remove_component::<InBackpack>(to_drop.item.entity());

                if entity == *player_entity {
                    gamelog.entries.push(format!(
                        "You drop the {}",
                        get_name(world, to_drop.item.entity())
                    ));
                }
                commands.remove_component::<WantsToDropItem>(entity);
            }
        })
}
