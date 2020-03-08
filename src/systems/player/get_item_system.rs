use super::super::*;

pub fn build() -> SystemBox {
    SystemBuilder::<()>::new("GetItemSystem")
        .with_query(<Read<Position>>::query().filter(tag::<Item>()))
        .read_resource::<Point>()
        .read_resource::<Entity>()
        .write_resource::<GameLog>()
        .build(
            move |commands, world, (player_pos, player_entity, gamelog), query| {
                let mut target_item: Option<Entity> = None;
                let player_entity: &Entity = player_entity;
                for (item_entity, position) in query.iter_entities(world) {
                    if position.x == player_pos.x && position.y == player_pos.y {
                        target_item = Some(item_entity);
                    }
                }

                match target_item {
                    None => gamelog
                        .entries
                        .push("There is nothing here to pick up.".to_string()),
                    Some(item) => {
                        commands.insert(
                            (),
                            vec![(WantsToPickupItem {
                                collected_by: EntityHolder::new(*player_entity),
                                item: EntityHolder::new(item),
                            },)],
                        );
                    }
                }
            },
        )
}
