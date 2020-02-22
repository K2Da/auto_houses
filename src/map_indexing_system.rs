use super::*;

pub fn build() -> Box<dyn legion::schedule::Schedulable> {
    SystemBuilder::<()>::new("MapIndexingSystem")
        .with_query(<(Read<Position>, TryRead<BlocksTile>)>::query())
        .write_resource::<Map>()
        .build(move |_commands, world, map, query| {
            let map: &mut Map = map;

            map.populate_blocked();
            map.clear_content_index();
            for (entity, (position, blockers)) in query.iter_entities(world) {
                let idx = map.xy_idx(position.x, position.y);

                // If they block, update the blocking list
                if blockers.is_some() {
                    map.blocked[idx] = true;
                }

                // Push the entity to the appropriate index slot. It's a Copy
                // type, so we don't need to clone it (we want to avoid moving it out of the ECS!)
                map.tile_content[idx].push(entity);
            }
        })
}
