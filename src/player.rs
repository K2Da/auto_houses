use super::{Map, Player, Position, RunState, State, TileType};
use crate::components::{CombatStats, Viewshed, WantsToMelee};
use legion::prelude::*;
use rltk::{Point, Rltk, VirtualKeyCode};
use std::cmp::{max, min};

fn try_move_player(delta_x: i32, delta_y: i32, gs: &mut State) {
    let mut schedule = Schedule::builder()
        .add_system(move_player_system(delta_x, delta_y))
        .flush()
        .build();
    schedule.execute(&mut gs.world);
}

fn move_player_system(delta_x: i32, delta_y: i32) -> Box<dyn legion::schedule::Schedulable> {
    SystemBuilder::<()>::new("MovePlayerSystem")
        .with_query(<(Write<Position>, Write<Viewshed>)>::query().filter(component::<Player>()))
        .read_component::<CombatStats>()
        .write_component::<WantsToMelee>()
        .read_resource::<Map>()
        .write_resource::<Point>()
        .build(move |commands, world, (map, ppos), query| {
            for (player, (mut pos, mut viewshed)) in query.iter_entities(world) {
                let destination_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);

                for potential_target in map.tile_content[destination_idx].iter() {
                    if let Some(_) = world.get_component::<CombatStats>(*potential_target) {
                        println!("add wants");
                        commands.add_component(
                            player,
                            WantsToMelee {
                                target: *potential_target,
                            },
                        );
                        return;
                    }
                }

                if map.tiles[destination_idx] != TileType::Wall {
                    pos.x = min(79, max(0, pos.x + delta_x));
                    pos.y = min(49, max(0, pos.y + delta_y));
                }

                viewshed.dirty = true;

                ppos.x = pos.x;
                ppos.y = pos.y;
            }
        })
}

pub fn player_input(gs: &mut State, ctx: &mut Rltk) -> RunState {
    // Player movement
    match ctx.key {
        None => return RunState::AwaitingInput, // Nothing happened
        Some(key) => match key {
            VirtualKeyCode::Left | VirtualKeyCode::Numpad4 | VirtualKeyCode::H => {
                try_move_player(-1, 0, gs)
            }

            VirtualKeyCode::Right | VirtualKeyCode::Numpad6 | VirtualKeyCode::L => {
                try_move_player(1, 0, gs)
            }

            VirtualKeyCode::Up | VirtualKeyCode::Numpad8 | VirtualKeyCode::K => {
                try_move_player(0, -1, gs)
            }

            VirtualKeyCode::Down | VirtualKeyCode::Numpad2 | VirtualKeyCode::J => {
                try_move_player(0, 1, gs)
            }

            // Diagonals
            VirtualKeyCode::Numpad9 | VirtualKeyCode::U => try_move_player(1, -1, gs),

            VirtualKeyCode::Numpad7 | VirtualKeyCode::Y => try_move_player(-1, -1, gs),

            VirtualKeyCode::Numpad3 | VirtualKeyCode::N => try_move_player(1, 1, gs),

            VirtualKeyCode::Numpad1 | VirtualKeyCode::B => try_move_player(-1, 1, gs),

            _ => return RunState::AwaitingInput,
        },
    }
    RunState::PlayerTurn
}
