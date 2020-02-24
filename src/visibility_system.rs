use super::*;
use rltk::{field_of_view, Point};

pub fn build() -> SystemBox {
    SystemBuilder::<()>::new("VisibilitySystem")
        .with_query(<(Write<Viewshed>, Read<Position>, TryRead<Player>)>::query())
        .write_resource::<Map>()
        .build(move |_commands, world, map, query| {
            let map: &mut Map = map;
            for (mut viewshed, pos, player) in query.iter(&mut *world) {
                if viewshed.dirty {
                    viewshed.dirty = false;
                    viewshed.visible_tiles.clear();
                    viewshed.visible_tiles =
                        field_of_view(Point::new(pos.x, pos.y), viewshed.range, &*map);
                    viewshed.visible_tiles.retain(|p| {
                        p.x > 0
                            && p.x < map.width as i32 - 1
                            && p.y > 0
                            && p.y < map.height as i32 - 1
                    });

                    if player.is_none() {
                        continue;
                    }

                    for t in map.visible_tiles.iter_mut() {
                        *t = false
                    }

                    for vis in viewshed.visible_tiles.iter() {
                        let idx = map.xy_idx(vis.x, vis.y);
                        map.revealed_tiles[idx] = true;
                        map.visible_tiles[idx] = true;
                    }
                }
            }
        })
}
