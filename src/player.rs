use super::*;
use rltk::{Rltk, VirtualKeyCode};

fn try_move_player(delta_x: i32, delta_y: i32, gs: &mut State) {
    {
        let mut wants_move = gs.world.resources.get_mut::<WantsToMove>().unwrap();
        wants_move.x = delta_x;
        wants_move.y = delta_y;
    }

    gs.schedules.player.player_move.execute(&mut gs.world);
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
            VirtualKeyCode::Numpad9 | VirtualKeyCode::U => try_move_player(1, -1, gs),
            VirtualKeyCode::Numpad7 | VirtualKeyCode::Y => try_move_player(-1, -1, gs),
            VirtualKeyCode::Numpad3 | VirtualKeyCode::N => try_move_player(1, 1, gs),
            VirtualKeyCode::Numpad1 | VirtualKeyCode::B => try_move_player(-1, 1, gs),
            VirtualKeyCode::G => get_item(gs),
            VirtualKeyCode::I => return RunState::ShowInventory,
            VirtualKeyCode::D => return RunState::ShowDropItem,
            VirtualKeyCode::Escape => return RunState::SaveGame,
            VirtualKeyCode::Period => {
                if try_next_level(gs) {
                    return RunState::NextLevel;
                }
            }
            _ => return RunState::AwaitingInput,
        },
    }
    RunState::PlayerTurn
}

fn get_item(gs: &mut State) {
    gs.schedules.player.get_item.execute(&mut gs.world);
}

pub fn try_next_level(gs: &mut State) -> bool {
    let player_pos = gs.world.resources.get::<Point>().unwrap();
    let map = gs.world.resources.get::<Map>().unwrap();
    let player_idx = map.xy_idx(player_pos.x, player_pos.y);

    if map.tiles[player_idx] == TileType::DownStairs {
        true
    } else {
        let mut gamelog = gs.world.resources.get_mut::<GameLog>().unwrap();
        gamelog
            .entries
            .push("There is no way down from here.".to_string());
        false
    }
}
