use legion::prelude::*;
use legion::schedule;
use rltk::{Console, GameState, Point, RandomNumberGenerator, Rltk, RGB};
mod rect;
use rect::Rect;
mod ecs;
use ecs::components::*;
use ecs::resources::*;
use ecs::tags::*;
mod map;
use map::*;
mod player;
use player::*;
mod gamelog;
use gamelog::*;
mod gui;
mod spawner;
mod systems;
use systems::Schedules;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum RunState {
    AwaitingInput,
    PreRun,
    PlayerTurn,
    MonsterTurn,
    ShowInventory,
    ShowDropItem,
    ShowTargeting { range: i32, item: Entity },
}

pub struct State {
    world: World,
    schedules: Schedules,
}

type SystemBox = Box<dyn legion::schedule::Schedulable>;

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

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

        let mut newrunstate;
        {
            let runstate = self.world.resources.get::<RunState>().unwrap();
            newrunstate = *runstate;
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
                let result = gui::show_inventory(self, ctx);
                match result.0 {
                    gui::ItemMenuResult::Cancel => newrunstate = RunState::AwaitingInput,
                    gui::ItemMenuResult::NoResponse => {}
                    gui::ItemMenuResult::Selected => {
                        let item_entity = result.1.unwrap();
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
                            self.world.add_component(
                                player,
                                WantsToUseItem {
                                    item: item_entity,
                                    target: None,
                                },
                            );
                            newrunstate = RunState::PlayerTurn;
                        }
                    }
                }
            }
            RunState::ShowDropItem => {
                let result = gui::drop_item_menu(self, ctx);
                match result.0 {
                    gui::ItemMenuResult::Cancel => newrunstate = RunState::AwaitingInput,
                    gui::ItemMenuResult::NoResponse => {}
                    gui::ItemMenuResult::Selected => {
                        let item_entity = result.1.unwrap();
                        let player = *self.world.resources.get::<Entity>().unwrap();
                        self.world
                            .add_component(player, WantsToDropItem { item: item_entity });
                        newrunstate = RunState::PlayerTurn;
                    }
                }
            }
            RunState::ShowTargeting { range, item } => {
                let result = gui::ranged_target(self, ctx, range);
                match result.0 {
                    gui::ItemMenuResult::Cancel => newrunstate = RunState::AwaitingInput,
                    gui::ItemMenuResult::NoResponse => {}
                    gui::ItemMenuResult::Selected => {
                        let player = *self.world.resources.get::<Entity>().unwrap();
                        self.world.add_component(
                            player,
                            WantsToUseItem {
                                item,
                                target: result.1,
                            },
                        );
                        newrunstate = RunState::PlayerTurn;
                    }
                }
            }
        }
        {
            let mut runwriter = self.world.resources.get_mut::<RunState>().unwrap();
            *runwriter = newrunstate;
        }
    }
}

impl State {
    fn run_systems(&mut self) {
        self.schedules.main.execute(&mut self.world);
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

    let map = Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();
    gs.world.resources.insert(Point::new(player_x, player_y));

    for room in map.rooms.iter().skip(1) {
        spawner::spawn_room(&mut gs.world, room);
    }

    gs.world.resources.insert(map);

    let player_entity = spawner::player(&mut gs.world, player_x, player_y);

    gs.world.resources.insert(GameLog {
        entries: vec!["Welcome to Rusty Roguelike".to_string()],
    });

    gs.world.resources.insert(player_entity);
    gs.world.resources.insert(RunState::PreRun);
    gs.world.resources.insert(WantsToMove { x: 0, y: 0 });

    spawner::debug_all_item(&mut gs.world, player_x, player_y);

    rltk::main_loop(context, gs);
}
