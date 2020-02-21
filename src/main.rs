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
mod monster_ai_system;
mod visibility_system;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum RunState {
    Paused,
    Running,
}

pub struct State {
    world: World,
    schedule: schedule::Schedule,
    runstate: RunState,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        self.run_systems(ctx);

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
    }
}

impl State {
    fn run_systems(&mut self, ctx: &mut Rltk) {
        if self.runstate == RunState::Running {
            self.schedule.execute(&mut self.world);
            self.runstate = RunState::Paused;
        } else {
            self.runstate = player_input(self, ctx);
        }
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
            .build(),
        runstate: RunState::Running,
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
            )],
        );
    }

    gs.world.resources.insert(map);

    gs.world.insert(
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
        )],
    );

    rltk::main_loop(context, gs);
}
