use super::super::*;

pub fn build() -> SystemBox {
    SystemBuilder::<()>::new("ItemRemoveSystem")
        .with_query(<Read<WantsToRemoveItem>>::query())
        .read_resource::<Entity>()
        .build(move |commands, world, _resources, query| {
            for (entity, to_remove) in query.iter_entities(world) {
                commands.remove_component::<Equipped>(to_remove.item.entity());
                commands.add_component(to_remove.item.entity(), InBackpack::new(entity));
                commands.remove_component::<WantsToDropItem>(entity);
            }
        })
}
