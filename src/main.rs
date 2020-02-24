use legion::prelude::*;
use legion::schedule;
use rltk::{Console, GameState, Point, Rltk, RGB};
mod rect;
use rect::Rect;
mod components;
use components::*;
mod map;
use map::*;
mod player;
use crate::components::Monster;
use player::*;
mod damage_system;
mod delete_the_dead_system;
mod gamelog;
use gamelog::*;
mod gui;
mod map_indexing_system;
mod melee_combat_system;
mod monster_ai_system;
mod visibility_system;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum RunState {
    AwaitingInput,
    PreRun,
    PlayerTurn,
    MonsterTurn,
}

pub struct State {
    world: World,
    schedule: schedule::Schedule,
}

type SystemBox = Box<dyn legion::schedule::Schedulable>;

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

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
        }
        {
            let mut runwriter = self.world.resources.get_mut::<RunState>().unwrap();
            *runwriter = newrunstate;
        }

        draw_map(&mut self.world, ctx);

        let map = self.world.resources.get::<Map>().unwrap();
        for (pos, render) in
            <(Read<Position>, Read<Renderable>)>::query().iter_immutable(&self.world)
        {
            let idx = map.xy_idx(pos.x, pos.y);
            if map.visible_tiles[idx] {
                ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph)
            }
        }
        gui::draw_ui(&self.world, ctx);
    }
}

impl State {
    fn run_systems(&mut self) {
        self.schedule.execute(&mut self.world);
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
        schedule: Schedule::builder()
            .add_system(visibility_system::build())
            .add_system(monster_ai_system::build())
            .flush()
            .add_system(map_indexing_system::build())
            .flush()
            .add_system(melee_combat_system::build())
            .flush()
            .add_system(damage_system::build())
            .flush()
            .add_system(delete_the_dead_system::build())
            .flush()
            .build(),
    };
    let map = Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();
    gs.world.resources.insert(Point::new(player_x, player_y));

    let mut rng = rltk::RandomNumberGenerator::new();

    for (i, room) in map.rooms.iter().skip(1).enumerate() {
        let (x, y) = room.center();

        let dice = rng.roll_dice(1, 2);
        let glyph = match dice {
            1 => rltk::to_cp437('g'),
            _ => rltk::to_cp437('o'),
        };
        let name = match dice {
            1 => "Goblin".to_string(),
            _ => "Orc".to_string(),
        };

        gs.world.insert(
            (),
            vec![(
                Position { x, y },
                Renderable {
                    glyph,
                    fg: RGB::named(rltk::RED),
                    bg: RGB::named(rltk::BLACK),
                },
                Viewshed {
                    visible_tiles: Vec::new(),
                    range: 8,
                    dirty: true,
                },
                Monster {},
                Name {
                    name: format!("{} #{}", &name, i),
                },
                BlocksTile {},
                CombatStats {
                    max_hp: 16,
                    hp: 16,
                    defense: 1,
                    power: 4,
                },
            )],
        );
    }

    gs.world.resources.insert(map);

    let player_entity = gs.world.insert(
        (),
        vec![(
            Position {
                x: player_x,
                y: player_y,
            },
            Renderable {
                glyph: rltk::to_cp437('@'),
                fg: RGB::named(rltk::YELLOW),
                bg: RGB::named(rltk::BLACK),
            },
            Player {},
            Viewshed {
                visible_tiles: Vec::new(),
                range: 8,
                dirty: true,
            },
            Name {
                name: "Player".to_string(),
            },
            CombatStats {
                max_hp: 30,
                hp: 30,
                defense: 2,
                power: 5,
            },
        )],
    )[0];

    gs.world.resources.insert(GameLog {
        entries: vec!["Welcome to Rusty Roguelike".to_string()],
    });

    gs.world.resources.insert(player_entity);
    gs.world.resources.insert(RunState::PreRun);

    rltk::main_loop(context, gs);
}
