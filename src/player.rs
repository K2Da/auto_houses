use super::{Map, Player, Position, State, TileType};
use crate::components::Viewshed;
use legion::prelude::*;
use rltk::{Rltk, VirtualKeyCode};
use std::cmp::{max, min};

fn try_move_player(delta_x: i32, delta_y: i32, gs: &mut State) {
    move_player_system(delta_x, delta_y).run(&mut gs.world);
}

fn move_player_system(delta_x: i32, delta_y: i32) -> Box<dyn legion::schedule::Schedulable> {
    SystemBuilder::<()>::new("MovePlayerSystem")
        .with_query(<(Write<Position>, Write<Viewshed>)>::query().filter(component::<Player>()))
        .read_resource::<Map>()
        .build(move |_commands, world, map, query| {
            for (mut pos, mut viewshed) in query.iter(world) {
                let destination_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);
                if map.tiles[destination_idx] != TileType::Wall {
                    pos.x = min(79, max(0, pos.x + delta_x));
                    pos.y = min(49, max(0, pos.y + delta_y));
                }
                viewshed.dirty = true;
            }
        })
}

pub fn player_input(gs: &mut State, ctx: &mut Rltk) {
    match ctx.key {
        None => {}
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
            _ => {}
        },
    }
}
