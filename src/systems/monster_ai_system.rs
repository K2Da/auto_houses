use super::*;
use rltk::Point;

pub fn build() -> SystemBox {
    SystemBuilder::<()>::new("MonsterAISystem")
        .with_query(
            <(Write<Viewshed>, Write<Position>, Read<Name>)>::query().filter(tag::<Monster>()),
        )
        .write_resource::<Map>()
        .read_resource::<Point>()
        .read_resource::<RunState>()
        .read_resource::<Entity>()
        .write_component::<Confusion>()
        .build(
            move |commands, world, (map, player_pos, runstate, player_entity), query| {
                let map: &mut Map = map;
                let runstate: &RunState = runstate;
                let player_pos: &Point = player_pos;
                let player_entity: &Entity = player_entity;

                if *runstate != RunState::MonsterTurn {
                    return;
                }

                for (entity, (mut viewshed, mut pos, _name)) in query.iter_entities(world) {
                    let mut can_act = true;
                    let is_confused = world.get_component_mut::<Confusion>(entity);
                    if let Some(mut i_am_confused) = is_confused {
                        i_am_confused.turns -= 1;
                        if i_am_confused.turns < 1 {
                            commands.remove_component::<Confusion>(entity);
                        }
                        can_act = false;
                    }

                    if can_act {
                        let distance = rltk::DistanceAlg::Pythagoras
                            .distance2d(Point::new(pos.x, pos.y), *player_pos);
                        if distance < 1.5 {
                            commands.add_component(
                                entity,
                                WantsToMelee {
                                    target: *player_entity,
                                },
                            );
                        } else if viewshed.visible_tiles.contains(player_pos) {
                            let path = rltk::a_star_search(
                                map.xy_idx(pos.x, pos.y) as i32,
                                map.xy_idx(player_pos.x, player_pos.y) as i32,
                                map,
                            );
                            if path.success && path.steps.len() > 1 {
                                let mut idx = map.xy_idx(pos.x, pos.y);
                                map.blocked[idx] = false;
                                pos.x = (path.steps[1] % map.width as usize) as i32;
                                pos.y = (path.steps[1] / map.width as usize) as i32;
                                idx = map.xy_idx(pos.x, pos.y);
                                map.blocked[idx] = true;
                                viewshed.dirty = true;
                            }
                        }
                    }
                }
            },
        )
}
