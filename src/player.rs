use super::{Map, Player, Position, RunState, State, TileType};
use crate::components::Viewshed;
use legion::prelude::*;
use rltk::{Point, Rltk, VirtualKeyCode};
use std::cmp::{max, min};

fn try_move_player(delta_x: i32, delta_y: i32, gs: &mut State) {
    move_player_system(delta_x, delta_y).run(&mut gs.world);
}

fn move_player_system(delta_x: i32, delta_y: i32) -> Box<dyn legion::schedule::Schedulable> {
    SystemBuilder::<()>::new("MovePlayerSystem")
        .with_query(<(Write<Position>, Write<Viewshed>)>::query().filter(component::<Player>()))
        .read_resource::<Map>()
        .write_resource::<Point>()
        .build(move |_commands, world, (map, ppos), query| {
            for (mut pos, mut viewshed) in query.iter(world) {
                let destination_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);
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
    match ctx.key {
        None => return RunState::Paused,
        Some(key) => match key {
            VirtualKeyCode::Left => try_move_player(-1, 0, gs),
            VirtualKeyCode::Numpad4 => try_move_player(-1, 0, gs),
            VirtualKeyCode::H => try_move_player(-1, 0, gs),
            VirtualKeyCode::Right => try_move_player(1, 0, gs),
            VirtualKeyCode::Numpad6 => try_move_player(1, 0, gs),
            VirtualKeyCode::L => try_move_player(1, 0, gs),
            VirtualKeyCode::Up => try_move_player(0, -1, gs),
            VirtualKeyCode::Numpad8 => try_move_player(0, -1, gs),
            VirtualKeyCode::K => try_move_player(0, -1, gs),
            VirtualKeyCode::Down => try_move_player(0, 1, gs),
            VirtualKeyCode::Numpad2 => try_move_player(0, 1, gs),
            VirtualKeyCode::J => try_move_player(0, 1, gs),
            _ => return RunState::Paused,
        },
    }
    RunState::Running
}
