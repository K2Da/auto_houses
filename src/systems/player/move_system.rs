use super::super::*;
use std::cmp::{max, min};

pub fn build() -> SystemBox {
    SystemBuilder::<()>::new("MovePlayerSystem")
        .with_query(<(Write<Position>, Write<Viewshed>)>::query().filter(component::<Player>()))
        .read_component::<CombatStats>()
        .read_resource::<Map>()
        .write_resource::<Point>()
        .write_resource::<WantsToMove>()
        .build(move |commands, world, (map, ppos, wants_to_move), query| {
            for (player, (mut pos, mut viewshed)) in query.iter_entities(world) {
                let destination_idx = map.xy_idx(pos.x + wants_to_move.x, pos.y + wants_to_move.y);

                for potential_target in map.tile_content[destination_idx].iter() {
                    if let Some(_) = world.get_component::<CombatStats>(*potential_target) {
                        commands.add_component(player, WantsToMelee::new(*potential_target));
                        return;
                    }
                }

                if map.tiles[destination_idx] != TileType::Wall {
                    pos.x = min(79, max(0, pos.x + wants_to_move.x));
                    pos.y = min(49, max(0, pos.y + wants_to_move.y));
                }

                viewshed.dirty = true;

                ppos.x = pos.x;
                ppos.y = pos.y;
            }
        })
}
