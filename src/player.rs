use super::{xy_idx, Player, Position, State, TileType};
use legion::prelude::*;
use rltk::{Rltk, VirtualKeyCode};
use std::cmp::{max, min};

fn try_move_player(delta_x: i32, delta_y: i32, gs: &mut State) {
    for (mut pos, _) in <(Write<Position>, Read<Player>)>::query().iter(&mut gs.ecs) {
        let destination_idx = xy_idx(pos.x + delta_x, pos.y + delta_y);
        if gs.map[destination_idx] != TileType::Wall {
            pos.x = min(79, max(0, pos.x + delta_x));
            pos.y = min(49, max(0, pos.y + delta_y));
        }
    }
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
