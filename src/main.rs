use legion::prelude::*;
use legion::schedule;
use rltk::{Console, GameState, Point, RandomNumberGenerator, Rltk, RGB};
mod rect;
use rect::Rect;
mod ecs;
use ecs::components::*;
use ecs::entity_holder::*;
use ecs::resources::*;
use ecs::tags::*;
mod map;
use map::*;
mod player;
use player::*;
mod gamelog;
use gamelog::*;
mod gui;
mod random_table;
use random_table::*;
mod spawner;
mod systems;
use crate::systems::save::save_system::delete_save;
use systems::save::SaveData;
use systems::Schedules;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum RunState {
    AwaitingInput,
    PreRun,
    PlayerTurn,
    MonsterTurn,
    ShowInventory,
    ShowDropItem,
    ShowTargeting {
        range: i32,
        item: Entity,
    },
    MainMenu {
        menu_selection: gui::MainMenuSelection,
    },
    SaveGame,
    NextLevel,
    ShowRemoveItem,
    GameOver,
}

pub struct State {
    world: World,
    schedules: Schedules,
}

type SystemBox = Box<dyn legion::schedule::Schedulable>;

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        let mut newrunstate;
        {
            let runstate = self.world.resources.get::<RunState>().unwrap();
            newrunstate = *runstate;
        }
        ctx.cls();

        match newrunstate {
            RunState::MainMenu { .. } | RunState::GameOver => {}
            _ => {
                draw_map(&mut self.world, ctx);
                {
                    let map = self.world.resources.get::<Map>().unwrap();
                    let mut data = <(Read<Position>, Read<Renderable>)>::query()
                        .iter_immutable(&self.world)
                        .collect::<Vec<_>>();
                    data.sort_by(|a, b| b.1.render_order.cmp(&a.1.render_order));
                    for (pos, render) in data.iter() {
                        let idx = map.xy_idx(pos.x, pos.y);
                        if map.visible_tiles[idx] {
                            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph)
                        }
                    }
                    gui::draw_ui(&self.world, ctx);
                }
            }
        }

        match newrunstate {
            RunState::PreRun => {
                self.run_systems();
                newrunstate = RunState::AwaitingInput;
            }
            RunState::AwaitingInput => {
                newrunstate = player_input(self, ctx);
            }
            RunState::PlayerTurn => {
                self.run_systems();
                newrunstate = RunState::MonsterTurn;
            }
            RunState::MonsterTurn => {
                self.run_systems();
                newrunstate = RunState::AwaitingInput;
            }
            RunState::ShowInventory => {
                let (result, item_entity) = gui::show_inventory(self, ctx);
                match result {
                    gui::ItemMenuResult::Cancel => newrunstate = RunState::AwaitingInput,
                    gui::ItemMenuResult::NoResponse => {}
                    gui::ItemMenuResult::Selected => {
                        let item_entity = item_entity.unwrap();
                        let player = *self.world.resources.get::<Entity>().unwrap();
                        let is_item_ranged = self
                            .world
                            .get_component::<Ranged>(item_entity)
                            .map(|i| (*i).clone());
                        if let Some(is_item_ranged) = is_item_ranged {
                            newrunstate = RunState::ShowTargeting {
                                range: is_item_ranged.range,
                                item: item_entity,
                            }
                        } else {
                            self.world
                                .add_component(player, WantsToUseItem::new(item_entity, None));
                            newrunstate = RunState::PlayerTurn;
                        }
                    }
                }
            }
            RunState::ShowDropItem => {
                let (result, item_entity) = gui::drop_item_menu(self, ctx);
                match result {
                    gui::ItemMenuResult::Cancel => newrunstate = RunState::AwaitingInput,
                    gui::ItemMenuResult::NoResponse => {}
                    gui::ItemMenuResult::Selected => {
                        let item_entity = item_entity.unwrap();
                        let player = *self.world.resources.get::<Entity>().unwrap();
                        self.world
                            .add_component(player, WantsToDropItem::new(item_entity));
                        newrunstate = RunState::PlayerTurn;
                    }
                }
            }
            RunState::ShowTargeting { range, item } => {
                let (result, point) = gui::ranged_target(self, ctx, range);
                match result {
                    gui::ItemMenuResult::Cancel => newrunstate = RunState::AwaitingInput,
                    gui::ItemMenuResult::NoResponse => {}
                    gui::ItemMenuResult::Selected => {
                        let player = *self.world.resources.get::<Entity>().unwrap();
                        self.world.add_component(
                            player,
                            WantsToUseItem {
                                item: EntityHolder::new(item),
                                target: point,
                            },
                        );
                        newrunstate = RunState::PlayerTurn;
                    }
                }
            }
            RunState::MainMenu { .. } => {
                let result = gui::main_menu(self, ctx);
                match result {
                    gui::MainMenuResult::NoSelection { selected } => {
                        newrunstate = RunState::MainMenu {
                            menu_selection: selected,
                        }
                    }
                    gui::MainMenuResult::Selected { selected } => match selected {
                        gui::MainMenuSelection::NewGame => newrunstate = RunState::PreRun,
                        gui::MainMenuSelection::LoadGame => {
                            self.schedules.menu.load.execute(&mut self.world);
                            newrunstate = RunState::AwaitingInput;
                            delete_save();
                        }
                        gui::MainMenuSelection::Quit => {
                            ::std::process::exit(0);
                        }
                    },
                }
            }
            RunState::SaveGame => {
                self.schedules.menu.save.execute(&mut self.world);
                newrunstate = RunState::MainMenu {
                    menu_selection: gui::MainMenuSelection::LoadGame,
                };
            }
            RunState::NextLevel => {
                self.goto_next_level();
                newrunstate = RunState::PreRun;
            }
            RunState::ShowRemoveItem => {
                let (selected, item_entity) = gui::remove_item_menu(self, ctx);
                match selected {
                    gui::ItemMenuResult::Cancel => newrunstate = RunState::AwaitingInput,
                    gui::ItemMenuResult::NoResponse => {}
                    gui::ItemMenuResult::Selected => {
                        let item_entity = item_entity.unwrap();
                        let player = *self.world.resources.get::<Entity>().unwrap();
                        self.world
                            .add_component(player, WantsToRemoveItem::new(item_entity));
                        newrunstate = RunState::PlayerTurn;
                    }
                }
            }
            RunState::GameOver => match gui::game_over(ctx) {
                gui::GameOverResult::NoSelection => {}
                gui::GameOverResult::QuitToMenu => {
                    self.game_over_cleanup();
                    newrunstate = RunState::MainMenu {
                        menu_selection: gui::MainMenuSelection::NewGame,
                    }
                }
            },
        }
        {
            let mut runwriter = self.world.resources.get_mut::<RunState>().unwrap();
            *runwriter = newrunstate;
        }
        self.schedules.delete_the_dead.execute(&mut self.world);
    }
}

impl State {
    fn run_systems(&mut self) {
        self.schedules.main.execute(&mut self.world);
    }

    fn entities_to_remove_on_level_change(&mut self) -> Vec<Entity> {
        let mut to_delete: Vec<Entity> = vec![];
        for (entity, (player, in_backpack, equipped)) in
            <(TryRead<Player>, TryRead<InBackpack>, TryRead<Equipped>)>::query()
                .iter_entities(&mut self.world)
        {
            if player.is_none() && in_backpack.is_none() && equipped.is_none() {
                to_delete.push(entity);
            }
        }

        to_delete
    }

    fn goto_next_level(&mut self) {
        // Delete entities that aren't the player or his/her equipment
        let to_delete = self.entities_to_remove_on_level_change();
        for target in to_delete {
            self.world.delete(target);
        }

        // Build a new map and place the player
        let worldmap;
        let current_depth;
        {
            let mut map = self.world.resources.get_mut::<Map>().unwrap();
            current_depth = map.depth;
            *map = Map::new_map_rooms_and_corridors(map.depth + 1);
            worldmap = map.clone();
        }

        // Spawn bad guys
        for room in worldmap.rooms.iter().skip(1) {
            spawner::spawn_room(&mut self.world, room, current_depth + 1);
        }

        self.initialize_components(worldmap);

        // Notify the player and give them some health
        {
            let mut gamelog = self.world.resources.get_mut::<GameLog>().unwrap();
            gamelog
                .entries
                .push("You descend to the next legel, and take a moment to heal".to_string());
        }

        let player_entity = self.world.resources.get::<Entity>().unwrap().clone();
        {
            let mut player_health = self
                .world
                .get_component_mut::<CombatStats>(player_entity)
                .unwrap();
            player_health.hp = i32::max(player_health.hp, player_health.max_hp / 2);
        }
    }

    fn initialize_components(&mut self, worldmap: Map) {
        // Place the player and update resources
        let (player_x, player_y) = worldmap.rooms[0].center();
        {
            let mut player_position = self.world.resources.get_mut::<Point>().unwrap();
            *player_position = Point::new(player_x, player_y);
        }
        let player_entity = self.world.resources.get::<Entity>().unwrap().clone();
        {
            let mut player_pos_comp = self
                .world
                .get_component_mut::<Position>(player_entity)
                .unwrap();
            player_pos_comp.x = player_x;
            player_pos_comp.y = player_y;
        }
        // Mark the player's visibility as dirty
        {
            let mut player_viewshed = self
                .world
                .get_component_mut::<Viewshed>(player_entity)
                .unwrap();
            player_viewshed.dirty = true;
        }
    }

    fn game_over_cleanup(&mut self) {
        let mut to_delete: Vec<Entity> = vec![];
        for (entity, _) in <TryRead<Name>>::query().iter_entities(&mut self.world) {
            to_delete.push(entity);
        }

        for target in to_delete {
            self.world.delete(target);
        }

        let worldmap;
        {
            let mut map_resource = self.world.resources.get_mut::<Map>().unwrap();
            *map_resource = Map::new_map_rooms_and_corridors(1);
            worldmap = map_resource.clone();
        }

        for room in worldmap.rooms.iter().skip(1) {
            spawner::spawn_room(&mut self.world, room, 1);
        }

        let (player_x, player_y) = worldmap.rooms[0].center();
        let player_entity = spawner::player(&mut self.world, player_x, player_y);
        self.world.resources.insert(player_entity);

        self.initialize_components(worldmap);
    }
}

fn main() {
    use rltk::RltkBuilder;

    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build();

    let universe = Universe::new();

    let mut gs = State {
        world: universe.create_world(),
        schedules: systems::build_schedules(),
    };

    gs.world
        .resources
        .insert(rltk::RandomNumberGenerator::new());

    let map = Map::new_map_rooms_and_corridors(1);
    let (player_x, player_y) = map.rooms[0].center();
    gs.world.resources.insert(Point::new(player_x, player_y));

    for room in map.rooms.iter().skip(1) {
        spawner::spawn_room(&mut gs.world, room, 1);
    }

    gs.world.resources.insert(map);

    let player_entity = spawner::player(&mut gs.world, player_x, player_y);

    gs.world.resources.insert(GameLog {
        entries: vec!["Welcome to Rusty Roguelike".to_string()],
    });

    gs.world.resources.insert(player_entity);
    gs.world.resources.insert(RunState::MainMenu {
        menu_selection: gui::MainMenuSelection::NewGame,
    });
    gs.world.resources.insert(WantsToMove { x: 0, y: 0 });
    gs.world.resources.insert(SaveData::default());

    spawner::debug_all_item(&mut gs.world, player_x, player_y);

    rltk::main_loop(context, gs);
}
