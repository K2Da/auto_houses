extern crate rltk;
use super::*;
use rltk::*;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum MainMenuSelection {
    NewGame,
    LoadGame,
    Quit,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum MainMenuResult {
    NoSelection { selected: MainMenuSelection },
    Selected { selected: MainMenuSelection },
}

pub fn draw_ui(world: &World, ctx: &mut Rltk) {
    ctx.draw_box(0, 43, 79, 6, c(WHITE), c(BLACK));

    for stats in <Read<CombatStats>>::query()
        .filter(component::<Player>())
        .iter_immutable(world)
    {
        let health = format!(" HP: {} / {} ", stats.hp, stats.max_hp);
        ctx.print_color(12, 43, c(YELLOW), c(BLACK), &health);

        ctx.draw_bar_horizontal(28, 43, 51, stats.hp, stats.max_hp, c(RED), c(BLACK));
    }

    let log = world.resources.get::<GameLog>().unwrap();
    let mut y = 44;
    for s in log.entries.iter().rev() {
        if y < 49 {
            ctx.print(2, y, s);
        }
        y += 1;
    }

    let mouse_pos = ctx.mouse_pos();
    ctx.set_bg(mouse_pos.0, mouse_pos.1, c(MAGENTA));
    draw_tooltips(world, ctx);
}

fn draw_tooltips(world: &World, ctx: &mut Rltk) {
    let map = world.resources.get::<Map>().unwrap();

    let mouse_pos = ctx.mouse_pos();
    if mouse_pos.0 >= map.width as i32 || mouse_pos.1 >= map.height as i32 {
        return;
    }

    let mut tooltip: Vec<String> = Vec::new();

    for (name, position) in <(Read<Name>, Read<Position>)>::query().iter_immutable(world) {
        if position.x == mouse_pos.0 && position.y == mouse_pos.1 {
            tooltip.push(name.name.to_string());
        }
    }

    if !tooltip.is_empty() {
        let mut width: i32 = 0;
        for s in tooltip.iter() {
            if width < s.len() as i32 {
                width = s.len() as i32;
            }
        }
        width += 3;

        if mouse_pos.0 > 40 {
            let arrow = Point::new(mouse_pos.0 - 2, mouse_pos.1);
            let left_x = mouse_pos.0 - width;
            let mut y = mouse_pos.1;
            for s in tooltip.iter() {
                ctx.print_color(left_x, y, c(WHITE), c(GREY), s);
                let padding = (width - s.len() as i32) - 1;
                for i in 0..padding {
                    ctx.print_color(arrow.x - i, y, c(WHITE), c(GREY), &" ".to_string());
                }
                y += 1;
            }
            ctx.print_color(arrow.x, arrow.y, c(WHITE), c(GREY), &"->".to_string());
        } else {
            let arrow = Point::new(mouse_pos.0 + 1, mouse_pos.1);
            let left_x = mouse_pos.0 + 3;
            let mut y = mouse_pos.1;
            for s in tooltip.iter() {
                ctx.print_color(left_x + 1, y, c(WHITE), c(GREY), s);
                let padding = (width - s.len() as i32) - 1;
                for i in 0..padding {
                    ctx.print_color(arrow.x + 1 + i, y, c(WHITE), c(GREY), &" ".to_string());
                }
                y += 1;
            }
            ctx.print_color(arrow.x, arrow.y, c(WHITE), c(GREY), &"<-".to_string());
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ItemMenuResult {
    Cancel,
    NoResponse,
    Selected,
}

pub fn show_inventory(gs: &mut State, ctx: &mut Rltk) -> (ItemMenuResult, Option<Entity>) {
    let player_entity = gs.world.resources.get::<Entity>().unwrap();

    let inventory = <(Read<InBackpack>, Read<Name>)>::query();
    let count = inventory
        .iter_immutable(&gs.world)
        .filter(|(pack, _)| pack.owner.entity() == *player_entity)
        .count() as i32;

    let y = (25 - (count / 2)) as i32;
    ctx.draw_box(15, y - 2, 31, (count + 3) as i32, c(WHITE), c(BLACK));
    ctx.print_color(18, y - 2, c(YELLOW), c(BLACK), "Inventory");
    ctx.print_color(18, y + count + 1, c(YELLOW), c(BLACK), "ESCAPE to cancel");

    let equippable = show_backpack(&gs.world, ctx, *player_entity, y);

    key_on_menu(ctx, count, equippable)
}

fn key_on_menu(
    ctx: &mut Rltk,
    count: i32,
    equippable: Vec<Entity>,
) -> (ItemMenuResult, Option<Entity>) {
    match ctx.key {
        None => (ItemMenuResult::NoResponse, None),
        Some(key) => match key {
            VirtualKeyCode::Escape => (ItemMenuResult::Cancel, None),
            _ => {
                let selection = letter_to_option(key);
                if selection > -1 && selection < count {
                    return (
                        ItemMenuResult::Selected,
                        Some(equippable[selection as usize]),
                    );
                }
                (ItemMenuResult::NoResponse, None)
            }
        },
    }
}

pub fn drop_item_menu(gs: &mut State, ctx: &mut Rltk) -> (ItemMenuResult, Option<Entity>) {
    let player_entity = gs.world.resources.get::<Entity>().unwrap();

    let inventory = <(Read<InBackpack>, Read<Name>)>::query();
    let count = inventory
        .iter_immutable(&gs.world)
        .filter(|(pack, _)| pack.owner.entity() == *player_entity)
        .count() as i32;

    let y = (25 - (count / 2)) as i32;
    ctx.draw_box(15, y - 2, 31, (count + 3) as i32, c(WHITE), c(BLACK));
    ctx.print_color(18, y - 2, c(YELLOW), c(BLACK), "Drop Which Item?");
    ctx.print_color(18, y + count + 1, c(YELLOW), c(BLACK), "ESCAPE to cancel");

    let equippable = show_backpack(&gs.world, ctx, *player_entity, y);

    key_on_menu(ctx, count, equippable)
}

fn show_backpack(
    world: &legion::world::World,
    ctx: &mut Rltk,
    player_entity: Entity,
    mut y: i32,
) -> Vec<Entity> {
    let inventory = <(Read<InBackpack>, Read<Name>)>::query();
    let mut equippable: Vec<Entity> = Vec::new();
    let mut j = 0;
    for (entity, (pack, name)) in inventory.iter_entities_immutable(world) {
        if pack.owner.entity() == player_entity {
            ctx.set(17, y, c(WHITE), c(BLACK), to_cp437('('));
            ctx.set(18, y, c(YELLOW), c(BLACK), 97 + j as u8);
            ctx.set(19, y, c(WHITE), c(BLACK), to_cp437(')'));

            ctx.print(21, y, &name.name.to_string());
            equippable.push(entity);
            y += 1;
            j += 1;
        }
    }
    equippable
}

pub fn ranged_target(
    gs: &mut State,
    ctx: &mut Rltk,
    range: i32,
) -> (ItemMenuResult, Option<Point>) {
    let player_entity = *gs.world.resources.get::<Entity>().unwrap();
    let player_pos = *gs.world.resources.get::<Point>().unwrap();

    ctx.print_color(5, 0, c(YELLOW), c(BLACK), "Select Target:");

    // Highlight available target cells
    let mut available_cells = Vec::new();

    let visible = gs.world.get_component::<Viewshed>(player_entity);
    if let Some(visible) = visible {
        // We have a viewshed
        for idx in visible.visible_tiles.iter() {
            let distance = DistanceAlg::Pythagoras.distance2d(player_pos, *idx);
            if distance <= range as f32 {
                ctx.set_bg(idx.x, idx.y, c(BLUE));
                available_cells.push(*idx);
            }
        }
    } else {
        return (ItemMenuResult::Cancel, None);
    }

    // Draw mouse cursor
    let mouse_pos = ctx.mouse_pos();
    let mut valid_target = false;
    for idx in available_cells.iter() {
        if idx.x == mouse_pos.0 && idx.y == mouse_pos.1 {
            valid_target = true;
        }
    }
    if valid_target {
        ctx.set_bg(mouse_pos.0, mouse_pos.1, c(CYAN));
        if ctx.left_click {
            return (
                ItemMenuResult::Selected,
                Some(Point::new(mouse_pos.0, mouse_pos.1)),
            );
        }
    } else {
        ctx.set_bg(mouse_pos.0, mouse_pos.1, c(RED));
        if ctx.left_click {
            return (ItemMenuResult::Cancel, None);
        }
    }

    (ItemMenuResult::NoResponse, None)
}

pub fn main_menu(gs: &mut State, ctx: &mut Rltk) -> MainMenuResult {
    let save_exists = super::systems::save::save_system::does_save_exist();
    let runstate = *gs.world.resources.get::<RunState>().unwrap();

    ctx.print_color_centered(15, c(YELLOW), c(BLACK), "Rust Roguelike Tutorial");

    if let RunState::MainMenu {
        menu_selection: selection,
    } = runstate
    {
        if selection == MainMenuSelection::NewGame {
            ctx.print_color_centered(24, c(MAGENTA), c(BLACK), "Begin New Game");
        } else {
            ctx.print_color_centered(24, c(WHITE), c(BLACK), "Begin New Game");
        }

        if save_exists {
            if selection == MainMenuSelection::LoadGame {
                ctx.print_color_centered(25, c(MAGENTA), c(BLACK), "Load Game");
            } else {
                ctx.print_color_centered(25, c(WHITE), c(BLACK), "Load Game");
            }
        }

        if selection == MainMenuSelection::Quit {
            ctx.print_color_centered(26, c(MAGENTA), c(BLACK), "Quit");
        } else {
            ctx.print_color_centered(26, c(WHITE), RGB::named(BLACK), "Quit");
        }

        return match ctx.key {
            None => MainMenuResult::NoSelection {
                selected: selection,
            },
            Some(key) => match key {
                VirtualKeyCode::Escape => MainMenuResult::NoSelection {
                    selected: MainMenuSelection::Quit,
                },
                VirtualKeyCode::Up => {
                    let mut newselection = match selection {
                        MainMenuSelection::NewGame => MainMenuSelection::Quit,
                        MainMenuSelection::LoadGame => MainMenuSelection::NewGame,
                        MainMenuSelection::Quit => MainMenuSelection::LoadGame,
                    };
                    if newselection == MainMenuSelection::LoadGame && !save_exists {
                        newselection = MainMenuSelection::NewGame;
                    }
                    MainMenuResult::NoSelection {
                        selected: newselection,
                    }
                }
                VirtualKeyCode::Down => {
                    let mut newselection = match selection {
                        MainMenuSelection::NewGame => MainMenuSelection::LoadGame,
                        MainMenuSelection::LoadGame => MainMenuSelection::Quit,
                        MainMenuSelection::Quit => MainMenuSelection::NewGame,
                    };
                    if newselection == MainMenuSelection::LoadGame && !save_exists {
                        newselection = MainMenuSelection::Quit;
                    }
                    MainMenuResult::NoSelection {
                        selected: newselection,
                    }
                }
                VirtualKeyCode::Return => MainMenuResult::Selected {
                    selected: selection,
                },
                _ => MainMenuResult::NoSelection {
                    selected: selection,
                },
            },
        };
    }

    MainMenuResult::NoSelection {
        selected: MainMenuSelection::NewGame,
    }
}

pub fn c(col: (u8, u8, u8)) -> RGB {
    RGB::named(col)
}
