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

fn skip_turn(gs: &mut State) -> RunState {
    let player_entity = gs.world.resources.get::<Entity>().unwrap().clone();
    let mut can_heal = true;

    {
        let viewshed = gs.world.get_component::<Viewshed>(player_entity).unwrap();
        let world_map = gs.world.resources.get::<Map>().unwrap();

        for tile in viewshed.visible_tiles.iter() {
            let idx = world_map.xy_idx(tile.x, tile.y);
            for entity_id in world_map.tile_content[idx].iter() {
                match gs.world.get_tag::<Monster>(*entity_id) {
                    None => {}
                    Some(_) => can_heal = false,
                }
            }
        }
    }

    if can_heal {
        let mut player_hp = gs
            .world
            .get_component_mut::<CombatStats>(player_entity)
            .unwrap();

        player_hp.hp = i32::min(player_hp.hp + 1, player_hp.max_hp);
    }

    RunState::PlayerTurn
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
            VirtualKeyCode::Numpad5 | VirtualKeyCode::Space => return skip_turn(gs),
            VirtualKeyCode::Period => {
                if try_next_level(gs) {
                    return RunState::NextLevel;
                }
            }
            VirtualKeyCode::R => return RunState::ShowRemoveItem,
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
