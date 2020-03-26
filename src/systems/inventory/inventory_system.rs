use super::super::*;

pub fn build() -> SystemBox {
    SystemBuilder::<()>::new("InventorySystem")
        .with_query(<Read<WantsToPickupItem>>::query())
        .read_resource::<Entity>()
        .write_resource::<GameLog>()
        .read_component::<Name>()
        .build(move |commands, world, (player_entity, gamelog), query| {
            let player_entity: &Entity = player_entity;

            for (entity, pickup) in query.iter_entities(world) {
                commands.remove_component::<Position>(pickup.item.entity());
                commands.add_component(
                    pickup.item.entity(),
                    InBackpack {
                        owner: pickup.collected_by.clone(),
                    },
                );

                if pickup.collected_by.entity() == *player_entity {
                    gamelog.push(format!(
                        "You pick up the {}.",
                        world
                            .get_component::<Name>(pickup.item.entity())
                            .unwrap()
                            .name
                    ));
                }
                commands.delete(entity);
            }
        })
}
